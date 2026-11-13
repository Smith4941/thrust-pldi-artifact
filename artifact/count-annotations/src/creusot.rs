//! `creusot`: the specification and code lines of the Thrust tests of RQ1 (Table tab:creusot),
//! or with `--creusot` of Creusot's files of the cases (creusot.csv's `loc` and `spec`), by the
//! same rules in Creusot's syntax: an attribute is spec unless it is `derive`, `allow`, `cfg`,
//! `cfg_attr`, `inline`, `doc`, `must_use`, `warn` or `deny`; `snapshot!`, `pearlite!`,
//! `proof_assert!` and `ghost!` are the spec macros; a `#[logic]`, `#[predicate]` or `#[law]`
//! function is spec whole; a `View` or `DeepModel` impl is the counterpart of a `Model` impl.
//!
//! Each row of CASE_ITEMS_CSV (`id,file,items`) names a test file of THRUST_TREE and the items of
//! its case part, what Creusot's file of the case carries too: a name selects the struct, enum,
//! trait or function of that name and every impl for the type of that name; `T::f` selects
//! function `f` of trait `T`, and `!T::f` leaves out method `f` of the impls for `T`. The copy of the Iterator trait, adapters copied into a client and
//! `main` are not selected. Within the case part, blank lines, `//` comments and lines holding only
//! `{` are not counted, nor is code under `#[cfg(thrust_int_range)]`, nor the lines of any other
//! `cfg` attribute that names `thrust_int_range` (such as `#[cfg(not(thrust_int_range))]` on the
//! kept alternative): the evaluation does not set `THRUST_INT_RANGE` (decisions.md D62), so only
//! the configuration it verifies is counted.
//!
//! A counted line is a spec line when it touches
//! - a `#[thrust_macros::..]` or `#[thrust::..]` attribute;
//! - a `thrust_macros::..!` macro call, except `closure!`, of which only the `requires`, `ensures`
//!   and `captures` arguments are spec; or an `assert!` call;
//! - a whole function that is a predicate (`predicate` or `logic` attribute), a law (`law`
//!   attribute), the proof of a law (a method of a trait impl named like a law of a trait of the
//!   file) or a lemma (no result, at least one parameter, every parameter a `Ghost<..>`);
//! - a call of a law or a lemma;
//! - a whole `impl .. Model for ..` block;
//! - a where-clause predicate that names `Model`, and the `where` itself when every predicate of
//!   the clause does.
//!
//! A counted line is a code line when it holds a token outside these other than a `,` or `;`
//! (the separator after a where-predicate or a macro statement), so a line holding both
//! (`let s = thrust_macros::ghost!(..)`) is a code line and a spec line. The output is one row per
//! test: `id,loc,spec_lines`.

use std::collections::{BTreeSet, HashSet};
use std::error::Error;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use crate::{
    pos, range, separator, tokens, within, Range,
    blank, is_model_impl, last_segment, lines, parse, read_csv, thrust_attr, thrust_attrs,
};

/// The spec lines of a file by the rules above, those of its `assert!` calls apart, and the lines
/// of its attributes.
pub struct Spec {
    pub lines: BTreeSet<usize>,
    pub asserts: BTreeSet<usize>,
    /// The spans of the spec, and those of the `assert!` calls.
    pub ranges: Vec<Range>,
    pub assert_ranges: Vec<Range>,
    pub attributes: BTreeSet<usize>,
    /// The functions whose calls are proof steps: the laws a trait declares, and the lemmas.
    proof_steps: HashSet<String>,
    laws: HashSet<String>,
    in_trait_impl: bool,
    /// The file is Creusot's (`--creusot`): its own spec syntax, under the same rules.
    creusot: bool,
}

/// Creusot's attributes that are code, not spec; every other attribute of a Creusot file is spec.
const CREUSOT_CODE_ATTRS: [&str; 9] =
    ["derive", "allow", "cfg", "cfg_attr", "inline", "doc", "must_use", "warn", "deny"];

/// Creusot's spec macros, the counterparts of `thrust_macros::..!`.
const CREUSOT_SPEC_MACROS: [&str; 4] = ["snapshot", "pearlite", "proof_assert", "ghost"];

/// Creusot's model traits, the counterparts of Thrust's `Model`.
const CREUSOT_MODELS: [&str; 2] = ["View", "DeepModel"];

impl Spec {
    /// Nothing marked yet; the laws and lemmas are those of `file`.
    pub fn new(file: &syn::File) -> Spec {
        let mut proofs = Proofs::default();
        proofs.visit_file(file);
        Spec {
            lines: BTreeSet::new(),
            asserts: BTreeSet::new(),
            ranges: Vec::new(),
            assert_ranges: Vec::new(),
            attributes: BTreeSet::new(),
            proof_steps: &proofs.laws | &proofs.lemmas,
            laws: proofs.laws,
            in_trait_impl: false,
            creusot: false,
        }
    }

    /// As `new`, for a file of Creusot's.
    pub fn creusot(file: &syn::File) -> Spec {
        Spec { creusot: true, ..Spec::new(file) }
    }

    /// Whether an attribute is spec: Thrust's, or in a Creusot file any but CREUSOT_CODE_ATTRS.
    fn spec_attr(&self, a: &syn::Attribute) -> bool {
        match self.creusot {
            true => !CREUSOT_CODE_ATTRS.contains(&last_segment(a.path()).as_str()),
            false => thrust_attr(a).is_some(),
        }
    }

    /// The names of the spec attributes, Thrust's or Creusot's (`logic` for `#[logic(law)]`).
    fn attr_names(&self, attrs: &[syn::Attribute]) -> Vec<String> {
        match self.creusot {
            true => attrs.iter().filter(|a| self.spec_attr(a)).map(|a| last_segment(a.path())).collect(),
            false => thrust_attrs(attrs),
        }
    }

    fn is_model(&self, i: &syn::ItemImpl) -> bool {
        match (self.creusot, &i.trait_) {
            (true, Some((_, path, _))) => CREUSOT_MODELS.contains(&last_segment(path).as_str()),
            (true, None) => false,
            (false, _) => is_model_impl(i),
        }
    }

    fn mark(&mut self, span: Span) {
        self.lines.extend(lines(span));
        self.ranges.push(range(span));
    }

    /// Marks a whole function that is a law, a law's proof, a lemma or a predicate; returns
    /// whether it was one.
    fn whole_fn(&mut self, attrs: &[syn::Attribute], sig: &syn::Signature, span: Span) -> bool {
        let names = self.attr_names(attrs);
        let proof = self.in_trait_impl && self.laws.contains(&sig.ident.to_string());
        let whole = proof
            || is_lemma(sig)
            || names.iter().any(|n| n == "law" || n == "predicate" || n == "logic");
        if whole {
            self.mark(span);
        }
        whole
    }

    /// Marks the `requires`, `ensures` and `captures` arguments of `closure!`.
    fn closure_args(&mut self, tokens: TokenStream) {
        let tokens: Vec<TokenTree> = tokens.into_iter().collect();
        for pair in tokens.windows(2) {
            let (TokenTree::Ident(name), TokenTree::Group(args)) = (&pair[0], &pair[1]) else {
                continue;
            };
            if name == "requires" || name == "ensures" || name == "captures" {
                self.lines.extend(name.span().start().line..=args.span().end().line);
                self.ranges.push((pos(name.span().start()), pos(args.span().end())));
            }
        }
    }
}

/// The laws a trait declares, and the lemmas.
#[derive(Default)]
struct Proofs {
    laws: HashSet<String>,
    lemmas: HashSet<String>,
}

impl<'a> Visit<'a> for Proofs {
    fn visit_signature(&mut self, sig: &'a syn::Signature) {
        if is_lemma(sig) {
            self.lemmas.insert(sig.ident.to_string());
        }
    }

    fn visit_trait_item_fn(&mut self, f: &'a syn::TraitItemFn) {
        if thrust_attrs(&f.attrs).iter().any(|n| n == "law") {
            self.laws.insert(f.sig.ident.to_string());
        }
        visit::visit_trait_item_fn(self, f);
    }
}

impl<'a> Visit<'a> for Spec {
    fn visit_attribute(&mut self, a: &'a syn::Attribute) {
        self.attributes.extend(lines(a.span()));
        if self.spec_attr(a) {
            self.mark(a.span());
        }
    }

    fn visit_item_fn(&mut self, f: &'a syn::ItemFn) {
        if !self.whole_fn(&f.attrs, &f.sig, f.span()) {
            visit::visit_item_fn(self, f);
        }
    }

    fn visit_trait_item_fn(&mut self, f: &'a syn::TraitItemFn) {
        if !self.whole_fn(&f.attrs, &f.sig, f.span()) {
            visit::visit_trait_item_fn(self, f);
        }
    }

    fn visit_impl_item_fn(&mut self, f: &'a syn::ImplItemFn) {
        if !self.whole_fn(&f.attrs, &f.sig, f.span()) {
            visit::visit_impl_item_fn(self, f);
        }
    }

    fn visit_item_impl(&mut self, i: &'a syn::ItemImpl) {
        if self.is_model(i) {
            self.mark(i.span());
            return;
        }
        self.in_trait_impl = i.trait_.is_some();
        visit::visit_item_impl(self, i);
        self.in_trait_impl = false;
    }

    fn visit_where_clause(&mut self, w: &'a syn::WhereClause) {
        let model: Vec<_> =
            w.predicates.iter().filter(|p| names_model(p.to_token_stream())).collect();
        if model.len() == w.predicates.len() {
            self.mark(w.where_token.span);
        }
        for p in model {
            self.mark(p.span());
        }
    }

    fn visit_macro(&mut self, m: &'a syn::Macro) {
        let first = m.path.segments.first().map(|s| s.ident.to_string());
        if self.creusot && CREUSOT_SPEC_MACROS.contains(&last_segment(&m.path).as_str()) {
            self.mark(m.span());
            return;
        }
        match (first.as_deref(), last_segment(&m.path).as_str()) {
            (Some("thrust_macros"), "closure") => self.closure_args(m.tokens.clone()),
            (Some("thrust_macros"), _) => self.mark(m.span()),
            (Some("assert"), "assert") => {
                self.asserts.extend(lines(m.span()));
                self.assert_ranges.push(range(m.span()));
            }
            _ => {}
        }
    }

    fn visit_expr_call(&mut self, c: &'a syn::ExprCall) {
        if let syn::Expr::Path(p) = &*c.func {
            if self.proof_steps.contains(&last_segment(&p.path)) {
                self.mark(c.span());
            }
        }
        visit::visit_expr_call(self, c);
    }
}

/// A lemma: a function without a result whose parameters, at least one, are all `Ghost<..>`.
fn is_lemma(sig: &syn::Signature) -> bool {
    let ghost = |arg: &syn::FnArg| match arg {
        syn::FnArg::Typed(t) => {
            matches!(&*t.ty, syn::Type::Path(p) if last_segment(&p.path) == "Ghost")
        }
        syn::FnArg::Receiver(_) => false,
    };
    matches!(sig.output, syn::ReturnType::Default)
        && !sig.inputs.is_empty()
        && sig.inputs.iter().all(ghost)
}

fn names_model(tokens: TokenStream) -> bool {
    tokens.into_iter().any(|t| match t {
        TokenTree::Ident(i) => i == "Model",
        TokenTree::Group(g) => names_model(g.stream()),
        _ => false,
    })
}

/// The name an item is selected by: its own, or for an impl the last segment of its type.
fn owner(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Struct(s) => Some(s.ident.to_string()),
        syn::Item::Enum(e) => Some(e.ident.to_string()),
        syn::Item::Fn(f) => Some(f.sig.ident.to_string()),
        syn::Item::Trait(t) => Some(t.ident.to_string()),
        syn::Item::Impl(i) => match &*i.self_ty {
            syn::Type::Path(p) => Some(last_segment(&p.path)),
            _ => None,
        },
        _ => None,
    }
}

/// Marks the case part, the items `names` selects, and returns the lines it spans; a name `!T::f`
/// leaves out method `f` of the impls for `T`.
fn visit_case(file: &syn::File, names: &[&str], spec: &mut Spec) -> BTreeSet<usize> {
    let mut case = BTreeSet::new();
    let left_out: Vec<&str> = names.iter().filter_map(|n| n.strip_prefix('!')).collect();
    for item in &file.items {
        let Some(owner) = owner(item) else { continue };
        if names.contains(&owner.as_str()) {
            case.extend(lines(item.span()));
            spec.visit_item(item);
            continue;
        }
        let syn::Item::Trait(t) = item else { continue };
        for member in &t.items {
            let syn::TraitItem::Fn(f) = member else { continue };
            if names.contains(&format!("{owner}::{}", f.sig.ident).as_str()) {
                case.extend(lines(f.span()));
                spec.visit_trait_item_fn(f);
            }
        }
    }
    for item in &file.items {
        let (Some(owner), syn::Item::Impl(i)) = (owner(item), item) else { continue };
        for member in &i.items {
            let syn::ImplItem::Fn(f) = member else { continue };
            if left_out.contains(&format!("{owner}::{}", f.sig.ident).as_str()) {
                lines(f.span()).for_each(|n| {
                    case.remove(&n);
                });
            }
        }
    }
    case
}

/// The configuration the evaluation does not set: code under `#[cfg(thrust_int_range)]` is left out.
const UNSET_CFG: &str = "thrust_int_range";

/// Whether `attrs` hold `#[cfg(thrust_int_range)]`.
fn unset_cfg(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        let syn::Meta::List(l) = &a.meta else { return false };
        l.path.is_ident("cfg") && l.tokens.to_string() == UNSET_CFG
    })
}

/// Whether `a` is a `cfg` attribute whose predicate names `thrust_int_range`.
fn names_unset_cfg(a: &syn::Attribute) -> bool {
    let syn::Meta::List(l) = &a.meta else { return false };
    l.path.is_ident("cfg") && l.tokens.clone().into_iter().any(|t| names_ident(t, UNSET_CFG))
}

/// Whether the token tree `t` holds the identifier `name`.
fn names_ident(t: TokenTree, name: &str) -> bool {
    match t {
        TokenTree::Ident(i) => i == name,
        TokenTree::Group(g) => g.stream().into_iter().any(|t| names_ident(t, name)),
        _ => false,
    }
}

/// The lines of the statements and items under `#[cfg(thrust_int_range)]`, and of the other `cfg`
/// attributes that name `thrust_int_range`.
#[derive(Default)]
struct Unset {
    lines: BTreeSet<usize>,
}

impl<'a> Visit<'a> for Unset {
    fn visit_attribute(&mut self, a: &'a syn::Attribute) {
        if names_unset_cfg(a) {
            self.lines.extend(lines(a.span()));
        }
    }

    fn visit_local(&mut self, l: &'a syn::Local) {
        if unset_cfg(&l.attrs) {
            self.lines.extend(lines(l.span()));
            return;
        }
        visit::visit_local(self, l);
    }

    fn visit_item(&mut self, i: &'a syn::Item) {
        let attrs = match i {
            syn::Item::Fn(f) => &f.attrs,
            syn::Item::Impl(x) => &x.attrs,
            syn::Item::Type(x) => &x.attrs,
            syn::Item::Struct(x) => &x.attrs,
            syn::Item::Use(x) => &x.attrs,
            _ => {
                visit::visit_item(self, i);
                return;
            }
        };
        if unset_cfg(attrs) {
            self.lines.extend(lines(i.span()));
            return;
        }
        visit::visit_item(self, i);
    }
}

/// The lines that hold a token, other than a separator, outside the spec and the `assert!` calls;
/// a token over several lines (a string) marks its first and last line.
fn code_lines(file: &syn::File, spec: &Spec) -> BTreeSet<usize> {
    let marked = |p| spec.ranges.iter().chain(&spec.assert_ranges).any(|r| within(r, p));
    let mut code = BTreeSet::new();
    for t in tokens(file.to_token_stream()).iter().filter(|t| !separator(t) && !marked(t.range.0)) {
        code.extend([t.range.0 .0, t.range.1 .0]);
    }
    code
}

/// The rows of annotations.csv, or with `creusot` the lines of Creusot's files (creusot.csv's `loc`
/// and `spec`) by the same rules in Creusot's syntax.
pub fn run(tree: &str, cases: &str, creusot: bool) -> Result<(), Box<dyn Error>> {
    println!("id,loc,spec_lines");
    for row in read_csv(cases)? {
        let [id, path, items] = &row[..] else {
            return Err(format!("{cases}: not id,file,items: {row:?}").into());
        };
        let (source, file) = parse(&format!("{tree}/{path}"))?;
        let names: Vec<&str> = items.split_whitespace().collect();
        let mut spec = if creusot { Spec::creusot(&file) } else { Spec::new(&file) };
        let mut case = visit_case(&file, &names, &mut spec);
        let mut unset = Unset::default();
        unset.visit_file(&file);
        case.retain(|n| !unset.lines.contains(n));
        let text: Vec<&str> = source.lines().collect();
        let counted: Vec<usize> =
            case.iter().copied().filter(|&n| !blank(text[n - 1]) && text[n - 1].trim() != "{").collect();
        let code = code_lines(&file, &spec);
        let spec_lines = counted.iter().filter(|n| spec.lines.contains(n) || spec.asserts.contains(n));
        let code_lines = counted.iter().filter(|n| code.contains(n));
        println!("{id},{},{}", code_lines.count(), spec_lines.count());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The code and spec lines of the items `names` of `src`, in Thrust's or Creusot's syntax.
    fn kinds(src: &str, names: &[&str], creusot: bool) -> (Vec<usize>, Vec<usize>) {
        let file: syn::File = syn::parse_str(src).unwrap();
        let mut spec = if creusot { Spec::creusot(&file) } else { Spec::new(&file) };
        let case = visit_case(&file, names, &mut spec);
        let code = code_lines(&file, &spec);
        let text: Vec<&str> = src.lines().collect();
        let counted: Vec<usize> = case.into_iter().filter(|&n| !blank(text[n - 1])).collect();
        let spec_lines = counted.iter().copied().filter(|n| spec.lines.contains(n) || spec.asserts.contains(n));
        (counted.iter().copied().filter(|n| code.contains(n)).collect(), spec_lines.collect())
    }

    #[test]
    fn a_line_holding_code_and_spec_is_both_on_either_side() {
        let thrust = "#[thrust_macros::requires(x > 0)]
fn f(x: u32) {
let s = thrust_macros::ghost!(|| -> u32 { x });
thrust_macros::proof!(lemma(s));
}
";
        assert_eq!(kinds(thrust, &["f"], false), (vec![2, 3, 5], vec![1, 3, 4]));
        let creusot = "#[requires(x@ > 0)]
fn f(x: u32) {
let s = snapshot! { x };
proof_assert! { s@ > 0 };
}
";
        assert_eq!(kinds(creusot, &["f"], true), (vec![2, 3, 5], vec![1, 3, 4]));
    }

    #[test]
    fn code_under_the_unset_cfg_is_not_counted() {
        let src = "fn f() {
#[cfg(thrust_int_range)]
let g = thrust_macros::closure!(requires(true), |x: u32| -> u32 { x });
#[cfg(not(thrust_int_range))]
let g = 0;
}
";
        let file: syn::File = syn::parse_str(src).unwrap();
        let mut unset = Unset::default();
        unset.visit_file(&file);
        assert_eq!(unset.lines.into_iter().collect::<Vec<_>>(), vec![2, 3, 4]);
    }

    #[test]
    fn a_left_out_method_is_not_counted() {
        let src = "struct R {
}
impl R {
fn a() {
}
fn b() {
}
}
";
        let (code, _) = kinds(src, &["R", "!R::b"], true);
        assert_eq!(code, vec![1, 2, 3, 4, 5, 8]);
    }
}
