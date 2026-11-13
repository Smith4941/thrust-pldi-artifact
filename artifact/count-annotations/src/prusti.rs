//! `prusti`: the code and specification lines of the Prusti ports of RQ1's cases
//! (`../comparison-prusti/`), by the line rule of `creusot` (decisions.md D84, D85): a counted line
//! that holds a token of the specification below is a spec line, and one that holds a token outside
//! it, other than a `,` or `;`, a code line, so a line can be both.
//!
//! Each row of CASES_CSV (`id,file,items`) names a file of DIR and the items of its case part:
//! - `Name` selects the struct, enum, trait or function of that name and every impl for the type
//!   of that name;
//! - `=Name` selects only the struct, enum, trait or function of that name;
//! - `Trait/Type` selects the impl of `Trait` for `Type`, and `/Type` the inherent impls of `Type`;
//! - `/Type:shell` selects the first and the last line of the inherent impls of `Type` (the
//!   `impl Type {` and its `}`), as Creusot's tally puts them in the case part;
//! - `T::f` selects function `f` of trait `T`, or of the inherent impls of type `T`.
//!
//! A name also selects a `mod` of that name. Empty `items` select every item of the file but its
//! `mod` and `use` items. Within the case
//! part, blank lines, `//` comments and lines holding only `{` are not counted.
//!
//! A counted line is a spec line when it touches
//! - a Prusti attribute (`requires`, `ensures`, `after_expiry`, `assert_on_expiry`, `pure`,
//!   `trusted`, `invariant`, `refine_trait_spec`, `extern_spec`, `model`, `terminates`);
//! - a `predicate!` item or a `body_invariant!`, `prusti_assert!`, `prusti_assume!` or
//!   `prusti_refute!` call;
//! - a whole function that is pure (Prusti's specification function), a lemma (no result, at least
//!   one `ensures`, no `&mut` parameter), the proof of a lemma a trait declares (a method of a
//!   trait impl named like it), or a specification stub (trusted, its body only `unimplemented!()`);
//! - ghost code: a `let` whose value calls `ghost_copy`, `snap_` or a `Seq::` function, an assignment to a
//!   variable such a `let` binds in the same function, a call of a lemma, a field of type
//!   `GhostSeq` (the ghost fields of the iterator prototype's style), a method call on such a
//!   field, and an `if` whose
//!   branches are not empty and hold only ghost code (its condition and braces then count too);
//! - a whole impl or trait whose items are all specification, and a whole `#[extern_spec]` trait.
//!
//! `#[refine_trait_spec]` counts as specification: Prusti needs it only for an impl's contracts,
//! and Creusot has no counterpart, so it is one line Prusti adds per impl that states a contract.
//!
//! The lemmas are those of every file CASES_CSV names, so that a case file sees the lemmas of the
//! library files it includes.
//!
//! The output is one row per case:
//! `id,loc,spec,trusted,lemma_calls`, where `trusted` counts the `trusted` attributes and
//! `lemma_calls` the calls of lemmas in the case part. `--lines ID` prints instead every counted
//! line of that case with its kind (`code`, `spec` or `both`), to check the rules without reading
//! the totals.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::error::Error;

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use quote::ToTokens;

use crate::{blank, last_segment, lines, parse, range, read_csv, separator, tokens, within, Range};

const ATTRIBUTES: [&str; 11] = [
    "requires",
    "ensures",
    "after_expiry",
    "assert_on_expiry",
    "pure",
    "trusted",
    "invariant",
    "refine_trait_spec",
    "extern_spec",
    "model",
    "terminates",
];

const MACROS: [&str; 5] =
    ["predicate", "body_invariant", "prusti_assert", "prusti_assume", "prusti_refute"];

fn attr_name(a: &syn::Attribute) -> String {
    last_segment(a.path())
}

fn has_attr(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs.iter().any(|a| attr_name(a) == name)
}

/// A lemma: no result, at least one `ensures`, no `&mut` parameter or receiver.
fn is_lemma(attrs: &[syn::Attribute], sig: &syn::Signature) -> bool {
    let mutable = |arg: &syn::FnArg| match arg {
        syn::FnArg::Receiver(r) => r.mutability.is_some() || r.reference.is_none(),
        syn::FnArg::Typed(t) => matches!(&*t.ty, syn::Type::Reference(r) if r.mutability.is_some()),
    };
    matches!(sig.output, syn::ReturnType::Default)
        && has_attr(attrs, "ensures")
        && !sig.inputs.iter().any(mutable)
}

/// The names of the lemmas, for the calls and the proofs in trait impls.
#[derive(Default)]
struct Lemmas(HashSet<String>);

impl<'a> Visit<'a> for Lemmas {
    fn visit_item_fn(&mut self, f: &'a syn::ItemFn) {
        if is_lemma(&f.attrs, &f.sig) {
            self.0.insert(f.sig.ident.to_string());
        }
    }

    fn visit_trait_item_fn(&mut self, f: &'a syn::TraitItemFn) {
        if is_lemma(&f.attrs, &f.sig) {
            self.0.insert(f.sig.ident.to_string());
        }
    }

    fn visit_impl_item_fn(&mut self, f: &'a syn::ImplItemFn) {
        if is_lemma(&f.attrs, &f.sig) {
            self.0.insert(f.sig.ident.to_string());
        }
    }
}

/// The spec lines of a case part and its counts.
struct Spec {
    lines: BTreeSet<usize>,
    /// The spans the lines come from.
    ranges: Vec<Range>,
    lemmas: HashSet<String>,
    ghost_vars: HashSet<String>,
    ghost_fields: HashSet<String>,
    in_trait_impl: bool,
    trusted: usize,
    lemma_calls: usize,
}

impl Spec {
    fn new(lemmas: &HashSet<String>, file: &syn::File) -> Spec {
        let mut fields = GhostFields::default();
        fields.visit_file(file);
        Spec {
            lines: BTreeSet::new(),
            ranges: Vec::new(),
            lemmas: lemmas.clone(),
            ghost_vars: HashSet::new(),
            ghost_fields: fields.0,
            in_trait_impl: false,
            trusted: 0,
            lemma_calls: 0,
        }
    }

    fn mark(&mut self, span: Span) {
        self.lines.extend(lines(span));
        self.ranges.push(range(span));
    }

    /// Marks a whole function that is pure, a lemma or a lemma's proof; returns whether it was.
    fn whole_fn(
        &mut self,
        attrs: &[syn::Attribute],
        sig: &syn::Signature,
        body: Option<&syn::Block>,
        span: Span,
    ) -> bool {
        let proof = self.in_trait_impl && self.lemmas.contains(&sig.ident.to_string());
        let whole =
            proof || is_lemma(attrs, sig) || has_attr(attrs, "pure") || is_stub(attrs, body);
        if whole {
            self.mark(span);
        }
        whole
    }

    /// Whether an expression calls `ghost_copy` or a `Seq::` function.
    fn ghost_value(expr: &syn::Expr) -> bool {
        let mut calls = GhostCalls(false);
        calls.visit_expr(expr);
        calls.0
    }
}

/// Finds a call of `ghost_copy` or of a `Seq::` function.
struct GhostCalls(bool);

impl<'a> Visit<'a> for GhostCalls {
    fn visit_expr_call(&mut self, c: &'a syn::ExprCall) {
        if let syn::Expr::Path(p) = &*c.func {
            let first = p.path.segments.first().map(|s| s.ident.to_string()).unwrap_or_default();
            let name = last_segment(&p.path);
            self.0 |= first == "Seq" || name == "ghost_copy" || name == "snap_";
        }
        visit::visit_expr_call(self, c);
    }
}

/// A specification stub: a trusted function whose body is only `unimplemented!()`.
fn is_stub(attrs: &[syn::Attribute], body: Option<&syn::Block>) -> bool {
    let unimplemented = |m: &syn::Macro| last_segment(&m.path) == "unimplemented";
    let only = |b: &syn::Block| match &b.stmts[..] {
        [syn::Stmt::Macro(m)] => unimplemented(&m.mac),
        [syn::Stmt::Expr(syn::Expr::Macro(m), _)] => unimplemented(&m.mac),
        _ => false,
    };
    has_attr(attrs, "trusted") && body.is_some_and(only)
}

/// Whether a field's type is a `GhostSeq`.
fn is_ghost_field(f: &syn::Field) -> bool {
    matches!(&f.ty, syn::Type::Path(p) if last_segment(&p.path) == "GhostSeq")
}

/// The names of the ghost fields of a file.
#[derive(Default)]
struct GhostFields(HashSet<String>);

impl<'a> Visit<'a> for GhostFields {
    fn visit_field(&mut self, f: &'a syn::Field) {
        if let (true, Some(name)) = (is_ghost_field(f), &f.ident) {
            self.0.insert(name.to_string());
        }
    }
}

fn pattern_name(pat: &syn::Pat) -> Option<String> {
    match pat {
        syn::Pat::Ident(i) => Some(i.ident.to_string()),
        syn::Pat::Type(t) => pattern_name(&t.pat),
        _ => None,
    }
}

impl<'a> Visit<'a> for Spec {
    fn visit_attribute(&mut self, a: &'a syn::Attribute) {
        let name = attr_name(a);
        if ATTRIBUTES.contains(&name.as_str()) {
            self.mark(a.span());
        }
        if name == "trusted" {
            self.trusted += 1;
        }
    }

    fn visit_item_fn(&mut self, f: &'a syn::ItemFn) {
        self.ghost_vars.clear();
        self.whole_fn(&f.attrs, &f.sig, Some(&f.block), f.span());
        visit::visit_item_fn(self, f);
    }

    fn visit_trait_item_fn(&mut self, f: &'a syn::TraitItemFn) {
        self.whole_fn(&f.attrs, &f.sig, f.default.as_ref(), f.span());
        visit::visit_trait_item_fn(self, f);
    }

    fn visit_impl_item_fn(&mut self, f: &'a syn::ImplItemFn) {
        self.ghost_vars.clear();
        self.whole_fn(&f.attrs, &f.sig, Some(&f.block), f.span());
        visit::visit_impl_item_fn(self, f);
    }

    fn visit_expr_if(&mut self, e: &'a syn::ExprIf) {
        visit::visit_expr_if(self, e);
        let else_block = match e.else_branch.as_ref().map(|(_, x)| &**x) {
            None => None,
            Some(syn::Expr::Block(b)) => Some(&b.block),
            Some(_) => return,
        };
        let ghost = |b: &syn::Block| {
            !b.stmts.is_empty()
                && b.stmts.iter().all(|st| lines(st.span()).all(|n| self.lines.contains(&n)))
        };
        if ghost(&e.then_branch) && else_block.map_or(true, ghost) {
            self.mark(e.span());
        }
    }

    fn visit_item_trait(&mut self, t: &'a syn::ItemTrait) {
        visit::visit_item_trait(self, t);
        let spec_only = |item: &syn::TraitItem| match item {
            syn::TraitItem::Type(_) => false,
            other => lines(other.span()).all(|n| self.lines.contains(&n)),
        };
        let all_spec = !t.items.is_empty() && t.items.iter().all(spec_only);
        if all_spec || has_attr(&t.attrs, "extern_spec") {
            self.mark(t.span());
        }
    }

    fn visit_item_impl(&mut self, i: &'a syn::ItemImpl) {
        if has_attr(&i.attrs, "extern_spec") {
            self.mark(i.span());
        }
        self.in_trait_impl = i.trait_.is_some();
        visit::visit_item_impl(self, i);
        self.in_trait_impl = false;
        let spec_only = |item: &syn::ImplItem| match item {
            syn::ImplItem::Type(_) => false,
            other => lines(other.span()).all(|n| self.lines.contains(&n)),
        };
        if !i.items.is_empty() && i.items.iter().all(spec_only) {
            self.mark(i.span());
        }
    }

    fn visit_macro(&mut self, m: &'a syn::Macro) {
        if MACROS.contains(&last_segment(&m.path).as_str()) {
            self.mark(m.span());
        }
    }

    fn visit_local(&mut self, l: &'a syn::Local) {
        let ghost = l.init.as_ref().is_some_and(|init| Spec::ghost_value(&init.expr));
        if let (true, Some(name)) = (ghost, pattern_name(&l.pat)) {
            self.ghost_vars.insert(name);
            self.mark(l.span());
        }
        visit::visit_local(self, l);
    }

    fn visit_field(&mut self, f: &'a syn::Field) {
        if is_ghost_field(f) {
            self.mark(f.span());
        }
        visit::visit_field(self, f);
    }

    fn visit_expr_method_call(&mut self, c: &'a syn::ExprMethodCall) {
        if let syn::Expr::Field(f) = &*c.receiver {
            if let syn::Member::Named(name) = &f.member {
                if self.ghost_fields.contains(&name.to_string()) {
                    self.mark(c.span());
                }
            }
        }
        visit::visit_expr_method_call(self, c);
    }

    fn visit_expr_assign(&mut self, a: &'a syn::ExprAssign) {
        if let syn::Expr::Path(p) = &*a.left {
            if self.ghost_vars.contains(&last_segment(&p.path)) {
                self.mark(a.span());
            }
        }
        visit::visit_expr_assign(self, a);
    }

    fn visit_expr_call(&mut self, c: &'a syn::ExprCall) {
        if let syn::Expr::Path(p) = &*c.func {
            if self.lemmas.contains(&last_segment(&p.path)) {
                self.mark(c.span());
                self.lemma_calls += 1;
            }
        }
        visit::visit_expr_call(self, c);
    }
}

/// The name an item is selected by: its own, or for an impl the last segment of its type.
fn owner(item: &syn::Item) -> Option<String> {
    match item {
        syn::Item::Struct(s) => Some(s.ident.to_string()),
        syn::Item::Enum(e) => Some(e.ident.to_string()),
        syn::Item::Fn(f) => Some(f.sig.ident.to_string()),
        syn::Item::Trait(t) => Some(t.ident.to_string()),
        syn::Item::Mod(m) => Some(m.ident.to_string()),
        syn::Item::Impl(i) => match &*i.self_ty {
            syn::Type::Path(p) => Some(last_segment(&p.path)),
            _ => None,
        },
        _ => None,
    }
}

/// Whether `item` is selected by one of `names` (the forms in the header).
fn selected(item: &syn::Item, names: &[&str]) -> bool {
    if names.is_empty() {
        return counted_by_default(item);
    }
    let Some(own) = owner(item) else { return false };
    if names.contains(&own.as_str()) {
        return true;
    }
    if let syn::Item::Impl(i) = item {
        let tr = i.trait_.as_ref().map(|(_, path, _)| last_segment(path)).unwrap_or_default();
        return names.contains(&format!("{tr}/{own}").as_str());
    }
    names.contains(&format!("={own}").as_str())
}

/// Whether an item belongs to the case part when no items are named.
fn counted_by_default(item: &syn::Item) -> bool {
    !matches!(item, syn::Item::Mod(_) | syn::Item::Use(_))
}

/// Marks the case part, the items `names` selects, and returns the lines it spans.
fn visit_case(file: &syn::File, names: &[&str], spec: &mut Spec) -> BTreeSet<usize> {
    let mut case = BTreeSet::new();
    for item in &file.items {
        if selected(item, names) {
            case.extend(lines(item.span()));
            spec.visit_item(item);
            continue;
        }
        if let syn::Item::Impl(i) = item {
            visit_inherent(i, names, spec, &mut case);
            continue;
        }
        let syn::Item::Trait(t) = item else { continue };
        for member in &t.items {
            let syn::TraitItem::Fn(f) = member else { continue };
            if names.contains(&format!("{}::{}", t.ident, f.sig.ident).as_str()) {
                case.extend(lines(f.span()));
                spec.visit_trait_item_fn(f);
            }
        }
    }
    case
}

/// Marks the parts of an inherent impl that `/Type:shell` and `Type::f` select.
fn visit_inherent(i: &syn::ItemImpl, names: &[&str], spec: &mut Spec, case: &mut BTreeSet<usize>) {
    let (None, syn::Type::Path(p)) = (&i.trait_, &*i.self_ty) else { return };
    let ty = last_segment(&p.path);
    if names.contains(&format!("/{ty}:shell").as_str()) {
        case.insert(i.span().start().line);
        case.insert(i.span().end().line);
    }
    for member in &i.items {
        let syn::ImplItem::Fn(f) = member else { continue };
        if names.contains(&format!("{ty}::{}", f.sig.ident).as_str()) {
            case.extend(lines(f.span()));
            spec.visit_impl_item_fn(f);
        }
    }
}

/// The counted lines of one case, each with whether it is a spec line and whether a code line,
/// and the case's counts.
struct Case {
    lines: BTreeMap<usize, (bool, bool)>,
    trusted: usize,
    lemma_calls: usize,
}

fn count(
    dir: &str,
    path: &str,
    items: &str,
    lemmas: &HashSet<String>,
) -> Result<(Case, Vec<String>), Box<dyn Error>> {
    let (source, file) = parse(&format!("{dir}/{path}"))?;
    let names: Vec<&str> = items.split_whitespace().collect();
    let mut spec = Spec::new(lemmas, &file);
    let case = visit_case(&file, &names, &mut spec);
    let text: Vec<String> = source.lines().map(String::from).collect();
    let counted = case.into_iter().filter(|&n| !blank(&text[n - 1]) && text[n - 1].trim() != "{");
    let mut code = BTreeSet::new();
    for t in tokens(file.to_token_stream()) {
        if !separator(&t) && !spec.ranges.iter().any(|r| within(r, t.range.0)) {
            code.extend([t.range.0 .0, t.range.1 .0]);
        }
    }
    let lines = counted.map(|n| (n, (spec.lines.contains(&n), code.contains(&n)))).collect();
    let case = Case { lines, trusted: spec.trusted, lemma_calls: spec.lemma_calls };
    Ok((case, text))
}

/// The rows of prusti-annotations.csv, or with `only` the classified lines of that case.
pub fn run(dir: &str, cases: &str, only: Option<&str>) -> Result<(), Box<dyn Error>> {
    let rows = read_csv(cases)?;
    let mut lemmas = Lemmas::default();
    for row in &rows {
        let [_, path, _] = &row[..] else {
            return Err(format!("{cases}: not id,file,items: {row:?}").into());
        };
        lemmas.visit_file(&parse(&format!("{dir}/{path}"))?.1);
    }
    if only.is_none() {
        println!("id,loc,spec,trusted,lemma_calls");
    }
    for row in &rows {
        let [id, path, items] = &row[..] else { unreachable!() };
        if only.is_some_and(|o| o != id) {
            continue;
        }
        let (case, text) = count(dir, path, items, &lemmas.0)?;
        if only.is_some() {
            for (n, kinds) in &case.lines {
                let kind = match kinds {
                    (true, true) => "both",
                    (true, false) => "spec",
                    _ => "code",
                };
                println!("{path}:{n:<4} {kind:<4} {}", text[n - 1]);
            }
            continue;
        }
        let spec = case.lines.values().filter(|(s, _)| *s).count();
        let loc = case.lines.values().filter(|(_, c)| *c).count();
        println!("{id},{loc},{spec},{},{}", case.trusted, case.lemma_calls);
    }
    Ok(())
}
