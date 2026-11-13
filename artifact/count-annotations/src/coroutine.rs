//! `coroutine`: the counts of the rustc coroutine layout case study (RQ4), one row per stage
//! (stages.csv), with `--functions` one row per function (functions.csv, with its stage, status,
//! lines of code, loops and contract), or with `--rows` one row per row of Table tab:coroutine
//! (rows.csv, D82).
//!
//! PARTS_CSV (`file,scope,from,part`) says which stage each part of the files of the case study
//! belongs to; a file is a path relative to `tests/ui/pass/rustc_coroutine/`, so the module tree
//! the stage roots include is reachable. A `root` row names a root file of a stage and, in `from`,
//! the row of Table tab:coroutine the root's `THRUST_VERIFY_ONLY` selects (a function row by its
//! function, a module row by its module); the run ids come from its file, and it counts nothing. A `section` row (its `from` may
//! hold commas) starts a part at the first line that begins with `from` (the first line of the
//! file when `from` is empty), and the part runs to the next section. A `fn` row puts the
//! function whose first line, without its indentation, begins with `from`, with its attributes,
//! in a part of its own; a section that starts at an item starts at the attribute lines right
//! above it. A file that PARTS_CSV does not name counts nowhere. A part is a stage number, `copy`
//! (code a stage file copies from an earlier stage, counted there), `driver` (test code, counted
//! nowhere) or `std` (a stand-in for std that the case study writes, such as `thrust/iter.rs` and
//! the own iterators of the rustc files, counted nowhere, as RQ1 leaves out the std adapters a
//! client copies; decisions.md D83) or `model` (a file of `Model` impls, `thrust/models.rs`: each
//! impl counts in the stage and row of the file that defines the type it models, so the impls of
//! a stand-in's types count nowhere, and so do those of the std types in `STD_MODELED`).
//!
//! The functions are those of the items of a file, of its inline modules, and of its impls and
//! traits. A function that
//! has a body and is no predicate or specification, but that the upstream copy of the file
//! (UPSTREAM_DIR, laid out like `tests/rustc_coroutine/`; none for a file under `thrust/`, the case
//! study's own, such as `thrust/iter.rs`; any other file without a copy stops the count) does not
//! have, under the same type, trait and name, is not code: its lines are annotation. If it is
//! `added` (a law proof, a lemma, a helper or any other function a rewrite adds) it is counted as
//! no function, whether or not it is ignored or trusted, and its lines are spec lines; an added ignored or trusted helper is an
//! annotation (D79), and `trusted_helpers` counts it so that its trust stays visible. The count
//! stops when a stage file has more functions under one key than the upstream copy, the added
//! ones apart. Otherwise a function is a
//! predicate (`predicate` or `logic` attribute), the specification of a verified or of a trusted
//! function (`extern_spec_fn`: trusted when the function it calls through `<T as Iterator>::f`, or
//! else its own name, is an ignored or trusted function of the same type), ignored (`ignored`), trusted
//! (`trusted`, or `trusted-callable` without `requires` or `ensures`), a declaration (no body),
//! assumed (it would be verified but its body holds an `assume!`, D79) or verified. Per stage:
//! - lines of code: the lines that hold a token of code, other than a `,` or `;`, outside the
//!   annotations and the spec ranges (below); as in RQ1, lines holding only `{` are not counted.
//!   Code is the items
//!   with their attributes other than Thrust's, the header and closing brace of an impl or trait
//!   that holds code, and the functions. Annotation is the Thrust attributes, the predicate and
//!   `extern_spec_fn` functions, `impl .. Model for ..` blocks, the
//!   header and closing brace of an impl, trait or inline module that holds no code, and the
//!   `thrust_macros::invariant!(..)`, `partial_invariant!(..)` and `assume!(..)` calls; `use` items
//!   (also `use thrust_models`, as RQ1 counts no `use`) and inner attributes are neither.
//! - functions verified, and those of them that are generic: in a trait, in an impl with a type
//!   parameter under a trait bound, with a bounded parameter of their own, or with an `impl` type
//!   in a parameter.
//! - loops: the `while`, `loop` and `for PAT in` loops of the verified functions; invariants
//!   written: the loops whose body holds an `invariant!` call outside a nested loop, however many
//!   calls state it; inferred: the other loops, also one whose body holds a `partial_invariant!`,
//!   which adds a conjunct to the inferred invariant (its lines are spec lines).
//! - contracts inferred: the verified and assumed functions without `requires`, `ensures` or
//!   `#[thrust::callable]` of their own whose contract Thrust infers: outside a trait impl, or
//!   implementing a method of a trait the files declare whose contract is not written in full
//!   (`requires` and `ensures`, or `callable`), which gives the impl none: "the impl's contract
//!   is inferred from its own body" (Thrust's README at fork 61faaf05, line 261); a
//!   method of a trait the files do not declare takes that trait's contract and is not counted. Whether a function is analyzed is its status: one that an `unchecked`
//!   row or a `compile-only` row of PARTS_CSV names is not counted here, and one no such row names
//!   is counted as analyzed.
//! - functions checked: the verified and assumed functions, whose bodies a run checks against a
//!   contract: written on them, taken from the trait method they implement, or inferred (the
//!   contracts inferred above). Trusted, ignored, unchecked and compile-only functions are not
//!   checked. Table tab:coroutine prints contracts inferred of functions checked, as it prints
//!   invariants inferred of loops. `generic_functions_checked` and `generic_contracts_inferred`
//!   are the generic ones of the two (generic as for functions verified above), which the table
//!   prints in a column of their own, since Thrust infers their contract per instance.
//! - functions extracted: the added functions other than lemmas and law proofs, trusted ones
//!   included: the functions a rewrite extracted from a rustc function (`functions_extracted`).
//! - contracts: the `requires` and `ensures` attributes of the verified functions, of declarations
//!   and of the specifications of verified functions.
//! - predicates; trusted functions: the trusted and the ignored ones, of which `ignored_functions`
//!   are the ignored.
//! - assumes: the assumed functions, counted apart from the verified and the trusted ones; they
//!   add their contract clauses to the contracts but no loop, invariant or generic function.
//!   opaque, reveals, assume_stmts: the `#[thrust::opaque]` attributes, the `#[thrust::reveal]`
//!   and `#[thrust::reveal(..)]` attributes, and the `assume!(..)` calls of the stage's functions
//!   (D78, D79); all three are annotations, so their lines are not lines of code.
//! - spec_lines: the lines that hold a token of an annotation (above) or of a range
//!   `creusot::Spec` marks, the rule of RQ1's `spec_lines` (Table tab:creusot), except that
//!   `assert!` calls are rustc's code here. What RQ1's tests do not have is spec as an annotation:
//!   each `added` or `extern_spec_fn` function whole, with its attributes, `assume!` calls however
//!   pathed, and the header and brace of an impl or inline module that holds only annotations.
//!   Blank and `//` lines and lines holding only `{` are not counted. A line that holds code and
//!   spec (`let s = thrust_macros::ghost!(..)`) is a line of code and a spec line, as in RQ1
//!   (decisions.md D83).
//! - unchecked: functions with a contract and a body that no root's run checks: outside every
//!   running root's selection, or called by nothing its root analyzes; every root takes the
//!   contract where its code calls them. An `unchecked` row of PARTS_CSV
//!   (`file,unchecked,Type::name,stage`, `name` alone outside an impl or trait) names one. A
//!   function without a contract that no run checks, whose clauses Thrust leaves out of the query,
//!   is in the tree only if the code needs it to compile: a `compile-only` row names it, the count
//!   stops if it has a contract or is trusted or ignored, or if an `unchecked` row names it too, and it counts in `loc`, in
//!   `compile_only_functions` and `compile_only_loc`, and in nothing else.
//!   `scripts/stage-report-check.py` compares both lists with the runs' reports.
//!
//! The rows of Table tab:coroutine (D82): a rustc function (a function of the upstream copy that
//! is no predicate or specification) with a body and at least ROW_LINES lines of code is a row of
//! its own, with its lines, attributes and body; the rest of a file that holds another rustc
//! function is a row (its module), with what the file holds outside its function rows in every
//! stage (so the rest of `rustc_index`'s files holds their stage-2 and stage-3 code, and of
//! `layout/coroutine.rs` none, its functions being rows of their own). A `row` row of PARTS_CSV (`file,row,,name`) puts the rest of
//! its file into the module row `name`, one row for all files it names (`rustc_index`), and names
//! the row of a single file (`rustc_hashes`, `rustc_abi`) for the `root` row that selects it; a
//! `name` that is a function row's `Type::name` (`name` alone outside an impl or trait) puts the
//! rest of the file into that function row, whose stage the rest must have (layout/simple.rs, whose
//! `LayoutData::scalar_pair` only `univariant_biased` needs). The count stops on a `name` that is
//! neither a function row nor a module row that a `root` row names. A `helpers` row of PARTS_CSV
//! (`file,helpers,from,name`) puts the lines of its file from the first line that begins with
//! `from` on (to the next `helpers` row) that no function row holds into the function row `name`:
//! the helpers a rewrite added for that function, which D82 counts in its row (layout.rs's lemmas
//! of the field placement and its helpers from `univariant_biased_post` on, and thrust/iter.rs's
//! `iter_enumerated_trusted`, are `univariant_biased`'s). The count stops on a `name`
//! that is no function row, or one of another stage than such a line. What a file without such a rest holds outside its function rows
//! (its types and predicates, or a file of the case study's own such as `thrust/iter.rs`) goes to
//! the first function row of the same stage in that file, else in any file. Only the parts that
//! are stages count, and a row has the same counts as a stage, over what it holds; the rows' counts
//! add up to the stages' or the count stops. A row's `files` are the roots that name it, the runs
//! that check its functions (none for a row without a verified or assumed function; the count
//! stops when a root names no row, or no root names a row with such a function), and `unchecked`
//! names the row's unchecked functions, whose contracts every root takes where its code calls
//! them, and `compile_only` its functions kept only so that the code compiles (`compile-only` rows
//! of PARTS_CSV), which no root's run checks and nothing assumes; `loc` counts their lines, and
//! `compile_only_functions` and `compile_only_loc` count them.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::error::Error;

use proc_macro2::{Delimiter, LineColumn, Span, TokenTree};
use quote::ToTokens;
use syn::spanned::Spanned;

use syn::visit::Visit;

use crate::creusot::Spec;
use crate::{
    blank, is_model_impl, last_segment, parse, pos, range, read_csv, separator, thrust_attr,
    thrust_attrs, tokens, within, Range, Token,
};

const DIR: &str = "tests/ui/pass/rustc_coroutine";

/// The `while`, `loop` and `for PAT in` loops of a body, and how many of them hold an `invariant!`
/// call outside a nested loop.
fn loops(body: &[Token]) -> (usize, usize) {
    let text = |i: usize| body.get(i).map_or("", |t| t.text.as_str());
    let mut bodies = Vec::new();
    for i in 0..body.len() {
        if !(text(i) == "while" || text(i) == "loop" || starts_for_loop(body, i)) {
            continue;
        }
        let brace = body[i..].iter().find_map(|t| match t.group {
            Some((Delimiter::Brace, end)) => Some((t.range.0, end)),
            _ => None,
        });
        bodies.push(brace.expect("a loop without a body"));
    }
    let owners: HashSet<Range> = macro_calls(body, &["invariant"])
        .iter()
        .filter_map(|call| bodies.iter().filter(|b| within(b, call.0)).max().copied())
        .collect();
    (bodies.len(), owners.len())
}

/// Whether the token at `i` is the `for` of a loop, `for PAT in EXPR { .. }`, whatever the pattern:
/// an `in` follows it outside the pattern's own groups. The `for` of a higher-ranked bound
/// (`for<'a>`) is followed by `<`, and that of `impl Trait for Type` reaches the impl's `{` or the
/// end of its group first. A struct pattern (`for S { a } in v`) would be taken for an impl; the
/// case study has none.
fn starts_for_loop(body: &[Token], i: usize) -> bool {
    if body[i].text != "for" || body.get(i + 1).is_some_and(|t| t.text == "<") {
        return false;
    }
    let mut depth = 0usize;
    for t in &body[i + 1..] {
        match (&t.group, t.text.as_str()) {
            (Some((Delimiter::Brace, _)), _) if depth == 0 => return false,
            (Some(_), _) => depth += 1,
            (None, "") if depth == 0 => return false,
            (None, "") => depth -= 1,
            (None, "in" | ";") if depth == 0 => return t.text == "in",
            _ => {}
        }
    }
    false
}

/// The ranges of the `thrust_macros::<name>!(..)` calls of a body for each name of `names`:
/// `invariant` states a loop's invariant; `partial_invariant` adds a conjunct to an inferred one.
fn macro_calls(body: &[Token], names: &[&str]) -> Vec<Range> {
    let mut calls = Vec::new();
    for w in body.windows(6) {
        let path: Vec<&str> = w[..5].iter().map(|t| t.text.as_str()).collect();
        if let (["thrust_macros", ":", ":", name, "!"], Some((_, end))) = (&path[..], w[5].group) {
            if names.contains(name) {
                calls.push((w[0].range.0, end));
            }
        }
    }
    calls
}

/// The ranges of the `assume!(..)` calls of a body (D79), however the macro is pathed.
fn assume_calls(body: &[Token]) -> Vec<Range> {
    let mut calls = Vec::new();
    for w in body.windows(3) {
        if let ("assume", "!", Some((_, end))) =
            (w[0].text.as_str(), w[1].text.as_str(), w[2].group)
        {
            calls.push((w[0].range.0, end));
        }
    }
    calls
}

/// The type an impl or a trait gives its functions: the trait, or the impl's type (a path's last
/// segment; any other type, such as a slice or a tuple, as printed). A free function has none.
fn owner_name(item: &syn::Item) -> String {
    fn name(ty: &syn::Type) -> String {
        match ty {
            syn::Type::Path(p) => last_segment(&p.path),
            syn::Type::Reference(r) => name(&r.elem),
            other => other.to_token_stream().to_string(),
        }
    }
    match item {
        syn::Item::Impl(i) => name(&i.self_ty),
        syn::Item::Trait(t) => t.ident.to_string(),
        _ => String::new(),
    }
}

/// The trait an impl implements; none for an inherent impl, a trait or a free function.
fn trait_name(item: &syn::Item) -> String {
    match item {
        syn::Item::Impl(syn::ItemImpl { trait_: Some((_, path, _)), .. }) => last_segment(path),
        _ => String::new(),
    }
}

/// Whether an impl or trait makes its functions generic: a trait, or an impl with a type
/// parameter under a trait bound.
fn generic_owner(item: &syn::Item) -> bool {
    let syn::Item::Impl(i) = item else { return matches!(item, syn::Item::Trait(_)) };
    i.generics.params.iter().any(|p| match p {
        syn::GenericParam::Type(t) => matches!(t.bounds.first(),
            Some(syn::TypeParamBound::Trait(b)) if matches!(b.modifier, syn::TraitBoundModifier::None)),
        syn::GenericParam::Const(_) => true,
        syn::GenericParam::Lifetime(_) => false,
    })
}

/// Whether a function has a bounded parameter of its own or an `impl` type in a parameter.
fn generic_fn(sig: &syn::Signature) -> bool {
    let bounded = sig.generics.params.iter().any(|p| match p {
        syn::GenericParam::Type(t) => !t.bounds.is_empty(),
        syn::GenericParam::Lifetime(l) => !l.bounds.is_empty(),
        syn::GenericParam::Const(_) => true,
    });
    let impl_type = |a: &syn::FnArg| match a {
        syn::FnArg::Typed(t) => tokens(t.ty.to_token_stream()).iter().any(|t| t.text == "impl"),
        syn::FnArg::Receiver(_) => false,
    };
    bounded || sig.inputs.iter().any(impl_type)
}

/// The parts of a function the counts read.
struct FnParts<'a> {
    attrs: &'a [syn::Attribute],
    sig: &'a syn::Signature,
    block: Option<&'a syn::Block>,
}

fn impl_fn(member: &syn::ImplItem) -> Option<FnParts<'_>> {
    let syn::ImplItem::Fn(f) = member else { return None };
    Some(FnParts { attrs: &f.attrs, sig: &f.sig, block: Some(&f.block) })
}

fn trait_fn(member: &syn::TraitItem) -> Option<FnParts<'_>> {
    let syn::TraitItem::Fn(f) = member else { return None };
    Some(FnParts { attrs: &f.attrs, sig: &f.sig, block: f.default.as_ref() })
}

/// A function of a stage file.
struct Function {
    line: usize,
    first_line: usize,
    last_line: usize,
    owner: String,
    /// The trait of the impl the function is in, or empty.
    of_trait: String,
    name: String,
    attrs: Vec<String>,
    /// The spans of the `requires` and `ensures` attributes.
    contract: Vec<Span>,
    generic: bool,
    body: Option<Vec<Token>>,
    /// The `assume!` calls of the body.
    assumes: usize,
    /// Has a body and is no predicate or specification, but is not a function of the upstream
    /// copy.
    added: bool,
    /// A method declared in a trait.
    in_trait: bool,
}

impl Function {
    fn key(&self) -> Key {
        (self.owner.clone(), self.of_trait.clone(), self.name.clone())
    }

    /// Whether the function is a predicate or a specification: an annotation, never `added`.
    fn is_spec(&self) -> bool {
        self.has("predicate") || self.has("logic") || self.has("extern_spec_fn")
    }

    /// Whether the contract is written in full: `requires` and `ensures`, or `#[thrust::callable]`.
    fn full_contract(&self) -> bool {
        (self.has("requires") && self.has("ensures")) || self.has("callable")
    }

    /// Whether Thrust infers the function's contract, as Thrust's README says: it has no
    /// `requires`, `ensures` or `#[thrust::callable]` of its own, and implements no trait method,
    /// or one of a local trait (`traits`: its methods, by trait and name, with whether their
    /// contract is written in full) whose contract is not written in full. A method of a trait
    /// that the counted files do not declare takes that trait's contract and is not counted.
    fn contract_inferred(&self, traits: &HashMap<(String, String), bool>) -> bool {
        let own = self.contract.is_empty() && !self.has("callable");
        let taken = match self.of_trait.is_empty() {
            true => false,
            false => traits.get(&(self.of_trait.clone(), self.name.clone())).is_none_or(|full| *full),
        };
        own && !taken
    }

    /// `Type::name`, or `name` outside an impl or trait.
    fn path(&self) -> String {
        match self.owner.is_empty() {
            true => self.name.clone(),
            false => format!("{}::{}", self.owner, self.name),
        }
    }

    fn has(&self, attr: &str) -> bool {
        self.attrs.iter().any(|a| a == attr)
    }

    /// How many attributes `attr` the function has (`reveal` may be given several times).
    fn attr_count(&self, attr: &str) -> usize {
        self.attrs.iter().filter(|a| *a == attr).count()
    }

    /// The function an `extern_spec_fn` wrapper specifies.
    fn target(&self) -> String {
        let t: Vec<&str> = self.body.iter().flatten().map(|t| t.text.as_str()).collect();
        let call = t.windows(5).find(|w| w[..4] == ["Iterator", ">", ":", ":"]);
        call.map_or(self.name.clone(), |w| w[4].to_string())
    }

    /// Whether the verdict relies on the function's contract unchecked: a trusted function, or an
    /// added helper marked trusted (an annotation, D79, whose contract still has to be true of
    /// rustc's code). An ignored function is not run, so none relies on it.
    fn is_trusted(&self, status: &str) -> bool {
        matches!(status, "trusted" | "trusted-callable") || (status == "added" && self.has("trusted"))
    }

    fn status(&self, ignored: &HashSet<(&str, &str)>) -> &'static str {
        if self.has("predicate") || self.has("logic") {
            "predicate"
        } else if self.has("extern_spec_fn") {
            match ignored.contains(&(self.owner.as_str(), self.target().as_str())) {
                true => "spec-of-trusted",
                false => "spec-of-verified",
            }
        } else if self.added {
            "added"
        } else if self.has("ignored") {
            "ignored"
        } else if self.has("trusted") {
            match self.contract.is_empty() {
                true => "trusted-callable",
                false => "trusted",
            }
        } else if self.body.is_none() {
            "declaration"
        } else if self.assumes > 0 {
            "assumed"
        } else {
            "verified"
        }
    }
}

/// The (type, trait, name) of a function.
type Key = (String, String, String);

/// How many functions with a body the upstream copy of a file has per key.
type Upstream = HashMap<Key, usize>;

/// The functions of the upstream copy of `file` under `dir`; none for a file of the case study's
/// own (under `thrust/`). Any other file without a copy is an error.
fn upstream_fns(dir: &str, file: &str) -> Result<Upstream, Box<dyn Error>> {
    let relative = file.trim_start_matches("../../../rustc_coroutine/");
    if relative.starts_with("thrust/") {
        return Ok(Upstream::new());
    }
    let path = format!("{dir}/{relative}");
    let (_, ast) = parse(&path)?;
    let mut spans = Spans::default();
    for item in &ast.items {
        spans.item(item);
    }
    let mut fns = Upstream::new();
    for f in spans.fns.iter().filter(|f| f.body.is_some()) {
        *fns.entry(f.key()).or_default() += 1;
    }
    Ok(fns)
}

/// The name of a struct, enum, union or type alias.
fn item_name(item: &syn::Item) -> String {
    match item {
        syn::Item::Struct(s) => s.ident.to_string(),
        syn::Item::Enum(e) => e.ident.to_string(),
        syn::Item::Union(u) => u.ident.to_string(),
        syn::Item::Type(t) => t.ident.to_string(),
        _ => String::new(),
    }
}

/// The code and annotation ranges and the functions of a stage file.
#[derive(Default)]
struct Spans {
    /// The `Model` impls, by the type they model and their first and last line.
    models: Vec<(String, usize, usize)>,
    /// The structs, enums, unions and type aliases, by name and line.
    types: Vec<(String, usize)>,
    upstream: Upstream,
    code: Vec<Range>,
    annotation: Vec<Range>,
    fns: Vec<Function>,
}

impl Spans {
    /// Classifies the outer attributes an item starts with, Thrust's as annotation and the others
    /// but doc comments as code; returns the range of the item after them.
    fn attrs(&mut self, item: &impl ToTokens) -> Range {
        let trees: Vec<TokenTree> = item.to_token_stream().into_iter().collect();
        let mut i = 0;
        while let [TokenTree::Punct(p), TokenTree::Group(g), ..] = &trees[i..] {
            if p.as_char() != '#' || g.delimiter() != Delimiter::Bracket {
                break;
            }
            let name = g.stream().into_iter().next().map(|t| t.to_string()).unwrap_or_default();
            let r = (pos(p.span().start()), pos(g.span().end()));
            match name.as_str() {
                "doc" => {}
                n if n.starts_with("thrust") => self.annotation.push(r),
                _ => self.code.push(r),
            }
            i += 2;
        }
        (range(trees[i].span()).0, range(trees[trees.len() - 1].span()).1)
    }

    fn item(&mut self, item: &syn::Item) {
        let body = self.attrs(item);
        match item {
            syn::Item::Fn(f) => {
                let parts = FnParts { attrs: &f.attrs, sig: &f.sig, block: Some(&f.block) };
                self.function(parts, body, None);
            }
            syn::Item::Impl(i) if is_model_impl(i) => {
                self.annotation.push(body);
                self.models.push((owner_name(item), body.0 .0, body.1 .0));
            }
            syn::Item::Struct(_) | syn::Item::Enum(_) | syn::Item::Union(_) | syn::Item::Type(_) => {
                self.types.push((item_name(item), body.0 .0));
                self.code.push(body);
            }
            syn::Item::Impl(i) => self.block(item, body, &i.items, impl_fn, i.brace_token.span),
            syn::Item::Trait(t) => self.block(item, body, &t.items, trait_fn, t.brace_token.span),
            syn::Item::Mod(syn::ItemMod { content: Some((brace, items)), .. }) => self.module(body, items, brace.span),
            syn::Item::Use(_) => {}
            _ => self.code.push(body),
        }
    }

    /// The members of an impl or trait, and its header and closing brace: code when a code range
    /// starts inside.
    fn block<M: ToTokens>(
        &mut self,
        owner: &syn::Item,
        body: Range,
        members: &[M],
        as_fn: fn(&M) -> Option<FnParts<'_>>,
        brace: proc_macro2::extra::DelimSpan,
    ) {
        let before = self.code.len();
        for member in members {
            let span = self.attrs(member);
            match as_fn(member) {
                Some(parts) => self.function(parts, span, Some(owner)),
                None => self.code.push(span),
            }
        }
        self.ends(before, body, brace);
    }

    /// The header and closing brace of an impl, trait or inline module: code when a code range
    /// pushed since `before` starts inside its body.
    fn ends(&mut self, before: usize, body: Range, brace: proc_macro2::extra::DelimSpan) {
        let ends = [(body.0, range(brace.open()).1), range(brace.close())];
        let code = self.code[before..].iter().any(|c| within(&body, c.0));
        (if code { &mut self.code } else { &mut self.annotation }).extend(ends);
    }

    /// The items of an inline module, and its header and closing brace: code when a code range
    /// starts inside.
    fn module(&mut self, body: Range, items: &[syn::Item], brace: proc_macro2::extra::DelimSpan) {
        let before = self.code.len();
        for item in items {
            self.item(item);
        }
        self.ends(before, body, brace);
    }

    fn function(&mut self, f: FnParts, span: Range, owner: Option<&syn::Item>) {
        let attrs = thrust_attrs(f.attrs);
        let body = f.block.map(|b| tokens(b.to_token_stream()));
        let first = f.attrs.iter().find(|a| !a.path().is_ident("doc"));
        let mut function = Function {
            line: span.0 .0,
            first_line: first.map_or(span.0 .0, |a| a.span().start().line),
            last_line: span.1 .0,
            owner: owner.map(owner_name).unwrap_or_default(),
            of_trait: owner.map(trait_name).unwrap_or_default(),
            name: f.sig.ident.to_string(),
            contract: f
                .attrs
                .iter()
                .filter(|a| matches!(thrust_attr(a).as_deref(), Some("requires" | "ensures")))
                .map(|a| a.span())
                .collect(),
            attrs,
            generic: owner.is_some_and(generic_owner) || generic_fn(f.sig),
            assumes: body.as_deref().map_or(0, |b| assume_calls(b).len()),
            body,
            added: false,
            in_trait: matches!(owner, Some(syn::Item::Trait(_))),
        };
        function.added = function.body.is_some()
            && !function.is_spec()
            && !self.upstream.contains_key(&function.key());
        let annotation = function.added || function.is_spec();
        match annotation {
            // the whole function, its attributes other than Thrust's too
            true => self.annotation.push((first.map_or(span.0, |a| pos(a.span().start())), span.1)),
            false => self.code.push(span),
        }
        self.annotation.extend(function.body.iter().flat_map(|b| macro_calls(b, &["invariant", "partial_invariant"])));
        self.annotation.extend(function.body.iter().flat_map(|b| assume_calls(b)));
        self.fns.push(function);
    }
}

/// The lines of code and the spec lines of a file. A token is spec inside an annotation (Spans) or
/// a range `creusot::Spec` marks (its `assert!` calls apart, rustc's code here), and code inside a
/// code range otherwise; a line holding a token of code other than a separator is a line of code,
/// and one holding a spec token a spec line, so a line can be both. A token over several lines (a
/// string) marks its first and last line. Blank and `//` lines and lines holding only `{` are
/// neither, as in RQ1.
fn line_kinds(file: &syn::File, text: &[&str], spans: &Spans) -> (BTreeSet<usize>, BTreeSet<usize>) {
    let mut spec = Spec::new(file);
    spec.visit_file(file);
    let in_spec = |p| spec.ranges.iter().chain(&spans.annotation).any(|r| within(r, p));
    let (mut code, mut specs) = (BTreeSet::new(), BTreeSet::new());
    for t in tokens(file.to_token_stream()) {
        let (start, lines) = (t.range.0, [t.range.0 .0, t.range.1 .0]);
        if in_spec(start) {
            specs.extend(lines);
        } else if !separator(&t) && spans.code.iter().any(|r| within(r, start)) {
            code.extend(lines);
        }
    }
    let counted = |n: &usize| !blank(text[n - 1]) && text[n - 1].trim() != "{";
    code.retain(counted);
    specs.retain(counted);
    (code, specs)
}

/// The part of each line of a file: its section, unless a `fn` row of `parts`
/// (`file,scope,from,part`) names its function.
fn parts_of(
    file: &str,
    text: &[&str],
    fns: &[Function],
    parts: &[Vec<String>],
) -> Result<Vec<String>, String> {
    let rows = |scope: &'static str| parts.iter().filter(move |p| p[0] == file && p[1] == scope);
    let mut starts = Vec::new();
    for p in rows("section") {
        let line = text.iter().position(|l| l.starts_with(&p[2]));
        let mut start = line.ok_or(format!("{file}: no line starts with {:?}", p[2]))? + 1;
        // a section that starts at an item starts at the attribute lines right above it
        while start > 1 && text[start - 2].starts_with("#[") && text[start - 2].trim_end().ends_with(']') {
            start -= 1;
        }
        starts.push((start, &p[3]));
    }
    starts.sort();
    let mut part = vec![String::new(); text.len() + 1];
    for (n, part) in part.iter_mut().enumerate().skip(1) {
        if let Some((_, p)) = starts.iter().rev().find(|(s, _)| *s <= n) {
            *part = p.to_string();
        }
    }
    for p in rows("fn") {
        let f = fns.iter().find(|f| text[f.line - 1].trim_start().starts_with(&p[2]));
        let f = f.ok_or(format!("{file}: no function starts with {:?}", p[2]))?;
        part[f.first_line..=f.last_line].fill(p[3].clone());
    }
    Ok(part)
}

/// The first lines of the functions the rows of `parts` with `scope` name by `Type::name` (`name`
/// for a function outside an impl or trait): `unchecked` (verified by attributes, but no root's run
/// checks their body) or `compile-only` (kept only so that the code compiles). Stops when a row
/// names no function of the file, or more than one.
fn listed_lines(file: &str, fns: &[Function], parts: &[Vec<String>], scope: &str) -> Result<HashSet<usize>, String> {
    let mut lines = HashSet::new();
    for p in parts.iter().filter(|p| p[0] == file && p[1] == scope) {
        let named: Vec<&Function> = fns.iter().filter(|f| f.path() == p[2]).collect();
        match named[..] {
            [f] => lines.insert(f.line),
            [] => return Err(format!("{file}: no function {:?}", p[2])),
            _ => return Err(format!("{file}: more than one function {:?}", p[2])),
        };
    }
    Ok(lines)
}

/// The status of a function a `compile-only` row names; stops when it has a contract or is trusted
/// or ignored, since then something assumes its contract or it is counted as trusted.
fn compile_only_status(file: &str, f: &Function) -> Result<&'static str, String> {
    let marked = ["callable", "trusted", "ignored"].iter().any(|a| f.has(a));
    match f.contract.is_empty() && !marked {
        true => Ok("compile-only"),
        false => Err(format!("{file}: compile-only {} has a contract or is trusted or ignored", f.path())),
    }
}

#[derive(Default)]
struct Stage {
    files: Vec<String>,
    loc: usize,
    verified: usize,
    generic: usize,
    loops: usize,
    contracts: usize,
    predicates: usize,
    trusted: usize,
    ignored: usize,
    unchecked: usize,
    invariants: usize,
    /// Functions that would be verified but hold an `assume!` (D79).
    assumed: usize,
    opaque: usize,
    reveals: usize,
    assume_stmts: usize,
    /// Added helpers marked ignored or trusted (D79).
    trusted_helpers: usize,
    /// Spec lines (`spec_lines`).
    spec: usize,
    /// Analyzed rustc functions whose contract is inferred (`contracts_inferred`).
    inferred: usize,
    /// The verified and assumed functions (`functions_checked`), of which `inferred` is a part.
    checked: usize,
    /// The generic ones of `checked` (`generic_functions_checked`) and of `inferred`
    /// (`generic_contracts_inferred`), whose contract Thrust infers per instance.
    generic_checked: usize,
    generic_inferred: usize,
    /// Added functions that are neither lemmas nor law proofs: functions a rewrite extracted from a
    /// rustc function (`functions_extracted`), trusted ones included.
    extracted: usize,
    /// rustc functions kept only so that the code compiles, and their lines of code, which `loc`
    /// includes.
    compile_only: usize,
    compile_only_loc: usize,
}

impl Stage {
    /// A function whose body a run checks: it counts as checked, and as inferred when its contract is.
    fn count_checked(&mut self, f: &Function, inferred: bool) {
        self.checked += 1;
        self.inferred += inferred as usize;
        self.generic_checked += f.generic as usize;
        self.generic_inferred += (f.generic && inferred) as usize;
    }

    fn add(&mut self, f: &Function, status: &str, loops: usize, invariants: usize, inferred: bool, loc: usize) {
        self.opaque += f.attr_count("opaque");
        self.reveals += f.attr_count("reveal");
        self.assume_stmts += f.assumes;
        self.trusted_helpers += (f.added && (f.has("ignored") || f.has("trusted"))) as usize;
        self.extracted += (f.added && !f.has("lemma") && !f.has("law")) as usize;
        match status {
            "assumed" => {
                self.assumed += 1;
                self.count_checked(f, inferred);
            }
            "unchecked" => self.unchecked += 1,
            "compile-only" => {
                self.compile_only += 1;
                self.compile_only_loc += loc;
            }
            "predicate" => self.predicates += 1,
            "verified" => {
                self.verified += 1;
                self.count_checked(f, inferred);
                self.generic += f.generic as usize;
                self.loops += loops;
                self.invariants += invariants;
            }
            "trusted" | "trusted-callable" => self.trusted += 1,
            "ignored" => {
                self.trusted += 1;
                self.ignored += 1;
            }
            _ => {}
        }
        if matches!(status, "verified" | "assumed" | "declaration" | "spec-of-verified") {
            self.contracts += f.contract.len();
        }
    }
}

/// Stops when the stage file has more functions with a body than the upstream copy under one key:
/// the match by (type, trait, name) could then take a function the rewrite added for upstream's.
fn check_multiplicity(file: &str, spans: &Spans) -> Result<(), String> {
    let mut seen: HashMap<Key, usize> = HashMap::new();
    for f in spans.fns.iter().filter(|f| f.body.is_some() && !f.is_spec() && !f.added) {
        *seen.entry(f.key()).or_default() += 1;
    }
    for (key, n) in seen {
        if n > spans.upstream[&key] {
            return Err(format!("{file}: {n} functions {key:?}, upstream has {}", spans.upstream[&key]));
        }
    }
    Ok(())
}

/// A function of a stage file as counted: its status, its part, the loops and invariants it adds
/// and its lines of code.
struct Counted {
    f: Function,
    /// Whether its contract is inferred (`Function::contract_inferred`), set once every file is
    /// counted.
    inferred: bool,
    status: &'static str,
    part: String,
    loops: usize,
    invariants: usize,
    loc: usize,
}

/// A stage file as counted: its functions, and the part of each of its lines of code. `module`
/// names the row its rest belongs to: the `row` of PARTS_CSV, else empty (its own).
#[derive(Default)]
struct CountedFile {
    name: String,
    module: String,
    fns: Vec<Counted>,
    code: Vec<(usize, String)>,
    /// The spec lines (`spec_lines`), with their parts.
    spec: Vec<(usize, String)>,
    /// The part of each line.
    parts: Vec<String>,
    /// The `Model` impls and the types of the file (Spans).
    models: Vec<(String, usize, usize)>,
    types: Vec<(String, usize)>,
    /// The `helpers` rows of PARTS_CSV: the first line of each and the function row it names.
    helpers: Vec<(usize, String)>,
}

/// Counts one stage file; returns it with its rows of functions.csv.
fn count(
    tree: &str,
    upstream: &str,
    file: &str,
    parts: &[Vec<String>],
) -> Result<(CountedFile, Vec<String>), Box<dyn Error>> {
    let (source, ast) = parse(&format!("{tree}/{DIR}/{file}"))?;
    let mut spans = Spans { upstream: upstream_fns(upstream, file)?, ..Spans::default() };
    for item in &ast.items {
        spans.item(item);
    }
    check_multiplicity(file, &spans)?;
    let text: Vec<&str> = source.lines().collect();
    let part = parts_of(file, &text, &spans.fns, parts)?;
    let unchecked = listed_lines(file, &spans.fns, parts, "unchecked")?;
    let compile_only = listed_lines(file, &spans.fns, parts, "compile-only")?;
    let (code, spec) = line_kinds(&ast, &text, &spans);
    let ignored: HashSet<(String, String)> = spans
        .fns
        .iter()
        .filter(|f| f.has("ignored") || f.has("trusted"))
        .map(|f| (f.owner.clone(), f.name.clone()))
        .collect();
    let ignored: HashSet<(&str, &str)> = ignored.iter().map(|(o, n)| (o.as_str(), n.as_str())).collect();
    let mut rows = Vec::new();
    let mut fns = Vec::new();
    for f in std::mem::take(&mut spans.fns) {
        let status = match (unchecked.contains(&f.line), compile_only.contains(&f.line)) {
            (true, true) => return Err(format!("{file}: {} is both unchecked and compile-only", f.path()).into()),
            (true, false) => "unchecked",
            (false, true) => compile_only_status(file, &f)?,
            (false, false) => f.status(&ignored),
        };
        let (loops, invariants) = match (&f.body, status) {
            (Some(body), "verified" | "assumed") => loops(body),
            _ => (0, 0),
        };
        let stage = part[f.line].clone();
        let (line, owner, name, generic) = (f.line, quoted(f.owner.clone()), &f.name, f.generic as usize);
        let loc = code.range(f.line..=f.last_line).count();
        let clauses: Vec<String> = f.contract.iter().map(|&a| text_of(&source, a)).collect();
        let (n, contract) = (clauses.len(), quoted(clauses.join(" ")));
        let trusted = f.is_trusted(status);
        rows.push(format!(
            "{stage},{file},{line},{owner},{name},{status},{loc},{generic},{loops},{invariants},{n},{contract},{trusted}"
        ));
        fns.push(Counted { f, status, part: stage, loops, invariants, loc, inferred: false });
    }
    let code = code.iter().map(|&n| (n, part[n].clone())).collect();
    let spec = spec.iter().map(|&n| (n, part[n].clone())).collect();
    let module = module_of(file, parts)?;
    let helpers = helpers_of(file, &text, parts)?;
    let (models, types) = (std::mem::take(&mut spans.models), std::mem::take(&mut spans.types));
    let name = file.to_string();
    Ok((CountedFile { name, module, fns, code, spec, parts: part, models, types, helpers }, rows))
}

/// The `helpers` rows of `file` in `parts`: the first line of each, the line that begins with its
/// `from`, and the function row it names, in the order of the file.
fn helpers_of(file: &str, text: &[&str], parts: &[Vec<String>]) -> Result<Vec<(usize, String)>, String> {
    let mut helpers = Vec::new();
    for p in parts.iter().filter(|p| p[0] == file && p[1] == "helpers") {
        let line = text.iter().position(|l| l.starts_with(&p[2]));
        let line = line.ok_or(format!("{file}: no line starts with {:?}", p[2]))?;
        helpers.push((line + 1, p[3].clone()));
    }
    helpers.sort();
    Ok(helpers)
}

/// The row the rest of `file` belongs to (a module row, or a function row by `Type::name`): the
/// name of its `row` row of `parts`, else empty; stops on a file with more than one.
fn module_of(file: &str, parts: &[Vec<String>]) -> Result<String, String> {
    let names: Vec<&String> = parts.iter().filter(|p| p[0] == file && p[1] == "row").map(|p| &p[3]).collect();
    match names[..] {
        [] => Ok(String::new()),
        [name] => Ok(name.clone()),
        _ => Err(format!("{file}: more than one `row` row: {names:?}")),
    }
}

/// Sets which functions' contracts are inferred, from the trait methods the files declare.
fn set_inferred(files: &mut [CountedFile]) {
    let traits: HashMap<(String, String), bool> = files
        .iter()
        .flat_map(|file| file.fns.iter().filter(|c| c.f.in_trait))
        .map(|c| ((c.f.owner.clone(), c.f.name.clone()), c.f.full_contract()))
        .collect();
    for c in files.iter_mut().flat_map(|file| file.fns.iter_mut()) {
        c.inferred = matches!(c.status, "verified" | "assumed") && c.f.contract_inferred(&traits);
    }
}

/// Adds the lines of code and the functions of `files` to the entry of `out` that `key` gives
/// for (file index, line, part); nothing when it gives none.
fn tally<K: Ord>(
    files: &[CountedFile],
    key: impl Fn(usize, usize, &str) -> Result<Option<K>, String>,
    out: &mut BTreeMap<K, Stage>,
) -> Result<(), String> {
    let key = |i, line, part| home(files, i, line, part).and_then(|(j, l, p)| key(j, l, p));
    for (i, file) in files.iter().enumerate() {
        for (line, part) in &file.code {
            if let Some(k) = key(i, *line, part)? {
                out.entry(k).or_default().loc += 1;
            }
        }
        for (line, part) in &file.spec {
            if let Some(k) = key(i, *line, part)? {
                out.entry(k).or_default().spec += 1;
            }
        }
        for c in &file.fns {
            if let Some(k) = key(i, c.f.line, &c.part)? {
                out.entry(k).or_default().add(&c.f, c.status, c.loops, c.invariants, c.inferred, c.loc);
            }
        }
    }
    Ok(())
}

/// The std types `thrust/models.rs` gives a `Model` impl; no counted file defines them.
const STD_MODELED: &[&str] = &["Reverse"];

/// Where a line counts: a line of a `Model` impl in a part `model` counts at the type it models
/// (the file, line and part that define it), any other line where it is. A std type of
/// `STD_MODELED`, which no counted file defines, leaves the line in its part `model`, which counts
/// nowhere, as the impls of a stand-in's types do. Stops on any other modeled type that no counted
/// file or more than one defines, so that a misspelt or renamed type is not dropped silently.
fn home<'a>(files: &'a [CountedFile], file: usize, line: usize, part: &'a str) -> Result<(usize, usize, &'a str), String> {
    let model = files[file].models.iter().find(|(_, a, b)| *a <= line && line <= *b);
    let (Some((ty, _, _)), "model") = (model, part) else { return Ok((file, line, part)) };
    let defs: Vec<(usize, usize)> = files
        .iter()
        .enumerate()
        .flat_map(|(j, f)| f.types.iter().filter(|(n, _)| n == ty).map(move |(_, l)| (j, *l)))
        .collect();
    match defs[..] {
        [] if STD_MODELED.contains(&ty.as_str()) => Ok((file, line, part)),
        [(j, l)] => Ok((j, l, files[j].parts[l].as_str())),
        _ => Err(format!("{}:{line}: {} counted files define {ty}", files[file].name, defs.len())),
    }
}

/// A rustc function of at least this many lines of code is a row of Table tab:coroutine of its
/// own (D82).
const ROW_LINES: usize = 40;

/// A row of Table tab:coroutine (D82): a rustc function of at least ROW_LINES lines of code
/// (`span` its lines), or the rest of a file (`span` none) when it holds another rustc function.
/// `stage` and `line` are those of the function, or of the rest's first rustc function.
struct Row {
    stage: String,
    file: usize,
    /// The `module` of the rest's files; empty for a function row and for the rest of a file that
    /// no `row` row names.
    module: String,
    line: usize,
    span: Option<(usize, usize)>,
    owner: String,
    name: String,
}

impl Row {
    /// `Type::name` of a function row, `name` alone outside an impl or trait.
    fn path(&self) -> String {
        match self.owner.is_empty() {
            true => self.name.clone(),
            false => format!("{}::{}", self.owner, self.name),
        }
    }
}

/// Whether a function is rustc's: a function of the upstream copy that is no predicate or
/// specification.
fn is_rustc(status: &str) -> bool {
    matches!(
        status,
        "verified" | "assumed" | "trusted" | "trusted-callable" | "ignored" | "unchecked" | "compile-only" | "declaration"
    )
}

/// The rows of Table tab:coroutine, ordered by stage, file and line. Only the parts that are
/// stages count.
fn table_rows(files: &[CountedFile], stages: &BTreeMap<String, Stage>) -> Vec<Row> {
    let mut rows: Vec<Row> = Vec::new();
    for (i, file) in files.iter().enumerate() {
        let rustc = file.fns.iter().filter(|c| is_rustc(c.status) && stages.contains_key(&c.part));
        let (own, rest): (Vec<&Counted>, Vec<&Counted>) =
            rustc.partition(|c| c.f.body.is_some() && c.loc >= ROW_LINES);
        for c in own {
            let span = Some((c.f.first_line, c.f.last_line));
            let (owner, name) = (c.f.owner.clone(), c.f.name.clone());
            let module = String::new();
            rows.push(Row { stage: c.part.clone(), file: i, module, line: c.f.line, span, owner, name });
        }
        let grouped = !file.module.is_empty();
        let known = rows.iter().any(|r| r.span.is_none() && grouped && r.module == file.module);
        if let (Some(c), false) = (rest.first(), known) {
            let (owner, name, module) = (String::new(), String::new(), file.module.clone());
            rows.push(Row { stage: c.part.clone(), file: i, module, line: c.f.line, span: None, owner, name });
        }
    }
    let functions: HashSet<String> = rows.iter().filter(|r| r.span.is_some()).map(Row::path).collect();
    rows.retain(|r| r.span.is_some() || !functions.contains(&r.module));
    rows.sort_by(|a, b| (&a.stage, a.file, a.line).cmp(&(&b.stage, b.file, b.line)));
    rows
}

/// The row a line of a file belongs to: the function row whose lines hold it, else the function
/// row of its stage that the file's `helpers` row above the line names, else the function
/// row of its stage that its file's `row` row names, else the rest of its file or of the module it belongs to
/// (whatever the line's stage), else (a file whose rustc
/// functions are all rows of their own, or a file of the case study's own) the first function row
/// of its stage in its file, else in any file.
fn row_of(rows: &[Row], files: &[CountedFile], file: usize, line: usize, part: &str) -> Option<usize> {
    let holds = |r: &Row| r.span.is_some_and(|(a, b)| a <= line && line <= b);
    let of_stage = |r: &Row| r.span.is_some() && r.stage == part;
    let module = &files[file].module;
    let rest = |r: &Row| r.span.is_none() && if module.is_empty() { r.file == file } else { &r.module == module };
    let helper = files[file].helpers.iter().rev().find(|(start, _)| *start <= line).map(|(_, name)| name);
    rows.iter()
        .position(|r| r.file == file && holds(r))
        .or_else(|| rows.iter().position(|r| r.span.is_some() && r.stage == part && helper == Some(&r.path())))
        .or_else(|| rows.iter().position(|r| r.span.is_some() && r.stage == part && !module.is_empty() && r.path() == *module))
        .or_else(|| rows.iter().position(rest))
        .or_else(|| rows.iter().position(|r| r.file == file && of_stage(r)))
        .or_else(|| rows.iter().position(of_stage))
}

/// The source text of a span, its white space collapsed to single spaces.
fn text_of(source: &str, span: Span) -> String {
    let lines: Vec<&str> = source.split('\n').collect();
    let offset = |lc: LineColumn| {
        let before: usize = lines[..lc.line - 1].iter().map(|l| l.len() + 1).sum();
        let line = lines[lc.line - 1];
        before + line.char_indices().nth(lc.column).map_or(line.len(), |(i, _)| i)
    };
    let text = &source[offset(span.start())..offset(span.end())];
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A CSV field, quoted when it holds a comma or a quote.
fn quoted(field: String) -> String {
    match field.contains(',') || field.contains('"') {
        true => format!("\"{}\"", field.replace('"', "\"\"")),
        false => field,
    }
}

/// What `run` prints.
pub enum Output {
    /// stages.csv: the counts per stage.
    Stages,
    /// functions.csv: one row per function.
    Functions,
    /// rows.csv: the counts per row of Table tab:coroutine (D82).
    Rows,
}

/// The number of COUNTS fields.
const COUNT_FIELDS: usize = 24;

const COUNTS: &str = "loc,functions_verified,generic,loops,contracts,predicates,trusted_functions,invariants_written,invariants_inferred,ignored_functions,unchecked_functions,assumes,opaque,reveals,assume_stmts,trusted_helpers,spec_lines,contracts_inferred,compile_only_functions,compile_only_loc,functions_checked,generic_functions_checked,generic_contracts_inferred,functions_extracted";

impl Stage {
    /// The COUNTS fields.
    fn fields(&self) -> [usize; COUNT_FIELDS] {
        [
            self.loc,
            self.verified,
            self.generic,
            self.loops,
            self.contracts,
            self.predicates,
            self.trusted,
            self.invariants,
            self.loops - self.invariants,
            self.ignored,
            self.unchecked,
            self.assumed,
            self.opaque,
            self.reveals,
            self.assume_stmts,
            self.trusted_helpers,
            self.spec,
            self.inferred,
            self.compile_only,
            self.compile_only_loc,
            self.checked,
            self.generic_checked,
            self.generic_inferred,
            self.extracted,
        ]
    }

    fn counts(&self) -> String {
        self.fields().iter().map(usize::to_string).collect::<Vec<_>>().join(",")
    }
}

/// The rows of stages.csv, functions.csv or rows.csv.
pub fn run(tree: &str, parts_csv: &str, upstream: &str, output: Output) -> Result<(), Box<dyn Error>> {
    let mut parts = read_csv(parts_csv)?;
    if let Some(r) = parts.iter().find(|r| r.len() < 4) {
        return Err(format!("{parts_csv}: not file,scope,from,part: {r:?}").into());
    }
    // `from` may hold commas: it is everything between the scope and the part.
    for r in &mut parts {
        let part = r.pop().unwrap();
        let from = r.split_off(2).join(",");
        r.extend([from, part]);
    }
    let mut stages: BTreeMap<String, Stage> = BTreeMap::new();
    let mut files: Vec<&str> = Vec::new();
    let mut roots: Vec<(&str, &str)> = Vec::new();
    for p in &parts {
        if p[1] == "root" {
            let s = stages.entry(p[3].clone()).or_default();
            s.files.push(p[0].clone());
            if !roots.contains(&(p[0].as_str(), p[2].as_str())) {
                roots.push((&p[0], &p[2]));
            }
            continue;
        }
        if !files.contains(&p[0].as_str()) {
            files.push(&p[0]);
        }
        if !matches!(p[1].as_str(), "unchecked" | "row" | "helpers") && !matches!(p[3].as_str(), "copy" | "driver" | "std" | "model") {
            stages.entry(p[3].clone()).or_default();
        }
    }
    let mut rows = vec!["stage,file,line,type,function,status,loc,generic,loops,invariants,contract_clauses,contract,trusted".to_string()];
    let mut counted = Vec::new();
    for file in files {
        let (c, r) = count(tree, upstream, file, &parts)?;
        counted.push(c);
        rows.extend(r);
    }
    set_inferred(&mut counted);
    match output {
        Output::Functions => println!("{}", rows.join("\n")),
        Output::Stages => print_stages(&counted, stages)?,
        Output::Rows => print_rows(&counted, &stages, &roots)?,
    }
    Ok(())
}

fn print_stages(files: &[CountedFile], mut stages: BTreeMap<String, Stage>) -> Result<(), String> {
    let names: HashSet<String> = stages.keys().cloned().collect();
    tally(files, |_, _, part| Ok(names.contains(part).then(|| part.to_string())), &mut stages)?;
    println!("stage,files,{COUNTS}");
    for (stage, s) in &stages {
        println!("{stage},{},{}", s.files.join(" "), s.counts());
    }
    Ok(())
}

/// Each row with its stage, file, type and function (none for the rest of a file), the roots whose
/// runs check its functions (the roots that name the row, `roots` as (file, row), when it has a
/// verified or assumed function), its unchecked functions, and its counts. Stops when the rows'
/// counts do not add up to the stages', when a root names no row, or when a row with a verified or
/// assumed function is named by no root.
fn print_rows(files: &[CountedFile], stages: &BTreeMap<String, Stage>, roots: &[(&str, &str)]) -> Result<(), String> {
    let rows = table_rows(files, stages);
    check_row_names(files, &rows, stages, roots)?;
    check_helper_names(files, &rows, stages)?;
    let key = |file: usize, line: usize, part: &str| match stages.contains_key(part) {
        false => Ok(None),
        true => row_of(&rows, files, file, line, part)
            .map(Some)
            .ok_or(format!("{}:{line}: no row of stage {part}", files[file].name)),
    };
    let mut counts = BTreeMap::new();
    tally(files, key, &mut counts)?;
    check_totals(files, stages, &counts)?;
    let mut checked = vec![false; rows.len()];
    let mut unchecked: Vec<Vec<String>> = vec![Vec::new(); rows.len()];
    let mut compile_only: Vec<Vec<String>> = vec![Vec::new(); rows.len()];
    for (i, file) in files.iter().enumerate() {
        for c in file.fns.iter().filter(|c| stages.contains_key(&c.part)) {
            let Some(r) = key(i, c.f.line, &c.part)? else { continue };
            match c.status {
                "unchecked" => unchecked[r].push(c.f.path()),
                "compile-only" => compile_only[r].push(c.f.path()),
                _ => {}
            }
            checked[r] |= matches!(c.status, "verified" | "assumed");
        }
    }
    let row_roots = roots_of_rows(&rows, roots, &checked)?;
    println!("stage,file,module,type,function,files,unchecked,compile_only,{COUNTS}");
    for (i, r) in rows.iter().enumerate() {
        let s = counts.remove(&i).unwrap_or_default();
        let (file, owner) = (&files[r.file].name, quoted(r.owner.clone()));
        let (files, unchecked, compile) = (row_roots[i].join(" "), unchecked[i].join(" "), compile_only[i].join(" "));
        println!("{},{file},{},{owner},{},{files},{unchecked},{compile},{}", r.stage, r.module, r.name, s.counts());
    }
    Ok(())
}

/// Stops when a `helpers` row of a file names no function row (by `Type::name`), or one of another
/// stage than a line it puts there, so that a mistake in PARTS_CSV does not move lines across stages.
fn check_helper_names(files: &[CountedFile], rows: &[Row], stages: &BTreeMap<String, Stage>) -> Result<(), String> {
    for (i, f) in files.iter().enumerate() {
        for (n, (start, name)) in f.helpers.iter().enumerate() {
            let Some(target) = rows.iter().find(|r| r.span.is_some() && r.path() == *name) else {
                return Err(format!("{}: its `helpers` row names {name:?}, which is no function row", f.name));
            };
            let end = f.helpers.get(n + 1).map_or(usize::MAX, |(next, _)| *next);
            let own = |line: usize| rows.iter().any(|r| r.file == i && r.span.is_some_and(|(a, b)| a <= line && line <= b));
            let lines = f.code.iter().chain(&f.spec).map(|(l, p)| (*l, p.as_str()));
            let lines = lines.chain(f.fns.iter().map(|c| (c.f.line, c.part.as_str())));
            let mut lines = lines.filter(|(l, p)| *start <= *l && *l < end && stages.contains_key(*p) && !own(*l));
            if let Some((line, part)) = lines.find(|(_, p)| *p != target.stage) {
                return Err(format!("{}:{line}: of stage {part}, but its `helpers` row names {name:?}, a row of stage {}", f.name, target.stage));
            }
        }
    }
    Ok(())
}

/// Stops when a `row` row of a file names neither a function row (by `Type::name`) nor a module row
/// that a root names, or names a function row of another stage than a line of the file's rest, so
/// that a mistake in PARTS_CSV does not become a row of its own or move lines across stages.
fn check_row_names(
    files: &[CountedFile],
    rows: &[Row],
    stages: &BTreeMap<String, Stage>,
    roots: &[(&str, &str)],
) -> Result<(), String> {
    for (i, f) in files.iter().enumerate().filter(|(_, f)| !f.module.is_empty()) {
        let Some(target) = rows.iter().find(|r| r.span.is_some() && r.path() == f.module) else {
            if roots.iter().any(|(_, name)| *name == f.module) {
                continue;
            }
            return Err(format!(
                "{}: its `row` row names {:?}, which is neither a function row nor a module row a root names",
                f.name, f.module
            ));
        };
        let own = |line: usize| rows.iter().any(|r| r.file == i && r.span.is_some_and(|(a, b)| a <= line && line <= b));
        let lines = f.code.iter().chain(&f.spec).map(|(l, p)| (*l, p.as_str()));
        let lines = lines.chain(f.fns.iter().map(|c| (c.f.line, c.part.as_str())));
        let other = lines.filter(|(l, p)| stages.contains_key(*p) && !own(*l)).find(|(_, p)| *p != target.stage);
        if let Some((line, part)) = other {
            return Err(format!(
                "{}:{line}: of stage {part}, but its `row` row names {:?}, a row of stage {}",
                f.name, f.module, target.stage
            ));
        }
    }
    Ok(())
}

/// The name a root gives a row: its function, or the module of the rest.
fn row_name(r: &Row) -> &str {
    match r.span {
        Some(_) => &r.name,
        None => &r.module,
    }
}

/// The roots of each row: those that name it, for a row whose functions some are `checked`
/// (verified or assumed), and none for the others. Stops when a root names no row, or when a
/// checked row is named by none.
fn roots_of_rows<'a>(rows: &[Row], roots: &[(&'a str, &str)], checked: &[bool]) -> Result<Vec<Vec<&'a str>>, String> {
    if let Some((root, name)) = roots.iter().find(|(_, name)| !rows.iter().any(|r| row_name(r) == *name)) {
        return Err(format!("the root {root} names the row {name:?}, which is none"));
    }
    let mut out = Vec::new();
    for (r, checked) in rows.iter().zip(checked) {
        let named: Vec<&str> = roots.iter().filter(|(_, name)| *name == row_name(r)).map(|(root, _)| *root).collect();
        if *checked && named.is_empty() {
            return Err(format!("no root names the row {:?}, which has verified functions", row_name(r)));
        }
        out.push(if *checked { named } else { Vec::new() });
    }
    Ok(out)
}

/// Stops when the counts of the rows do not add up to those of the stages, so that a change to the
/// row rules cannot drop or double a count unseen.
fn check_totals(
    files: &[CountedFile],
    stages: &BTreeMap<String, Stage>,
    rows: &BTreeMap<usize, Stage>,
) -> Result<(), String> {
    let mut by_stage = BTreeMap::new();
    let key = |_, _, part: &str| Ok(stages.contains_key(part).then(|| part.to_string()));
    tally(files, key, &mut by_stage)?;
    let total = |counts: Vec<[usize; COUNT_FIELDS]>| {
        counts.iter().fold([0; COUNT_FIELDS], |mut sum, c| {
            sum.iter_mut().zip(c).for_each(|(s, n)| *s += n);
            sum
        })
    };
    let (of_stages, of_rows) = (
        total(by_stage.values().map(Stage::fields).collect()),
        total(rows.values().map(Stage::fields).collect()),
    );
    match of_stages == of_rows {
        true => Ok(()),
        false => Err(format!("the rows count {of_rows:?}, the stages {of_stages:?} ({COUNTS})")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loops_of(body: &str) -> (usize, usize) {
        loops(&tokens(body.parse().unwrap()))
    }

    #[test]
    fn a_for_loop_counts_whatever_its_pattern() {
        assert_eq!(loops_of("for x in v { thrust_macros::invariant!(|| true); }"), (1, 1));
        assert_eq!(loops_of("for (a, b) in v.iter().enumerate() { f(a, b); }"), (1, 0));
        assert_eq!(loops_of("for &i in &v { f(i); }"), (1, 0));
        assert_eq!(loops_of("'l: for (i, &j) in v { for k in w { f(i, j, k); } }"), (2, 0));
    }

    #[test]
    fn a_higher_ranked_bound_or_an_impl_for_is_no_loop() {
        assert_eq!(loops_of("let g: Box<dyn for<'a> Fn(&'a u8)> = h; g(&x);"), (0, 0));
        assert_eq!(loops_of("impl Trait for S { fn m(&self) -> bool { true } } S.m();"), (0, 0));
        let iterator = "impl Iterator for S { type Item = u8; fn next(&mut self) -> Option<u8> { None } }";
        assert_eq!(loops_of(iterator), (0, 0));
    }

    #[test]
    fn a_for_loop_over_a_struct_pattern_is_missed_as_documented() {
        assert_eq!(loops_of("for S { a } in v { f(a); }"), (0, 0));
    }

    fn file_with(models: Vec<(String, usize, usize)>, types: Vec<(String, usize)>, parts: &[&str]) -> CountedFile {
        CountedFile {
            name: "f.rs".into(),
            module: String::new(),
            fns: Vec::new(),
            code: Vec::new(),
            spec: Vec::new(),
            parts: parts.iter().map(|p| p.to_string()).collect(),
            models,
            types,
            helpers: Vec::new(),
        }
    }

    #[test]
    fn a_model_impl_counts_at_its_type_or_nowhere_for_a_std_type() {
        let models = || file_with(vec![("S".into(), 1, 2), ("Reverse".into(), 3, 4)], vec![], &["model"; 5]);
        let defines = |stage: &str| file_with(vec![], vec![("S".into(), 1)], &["", stage]);
        let files = [models(), defines("6")];
        assert_eq!(home(&files, 0, 1, "model"), Ok((1, 1, "6")));
        assert_eq!(home(&files, 0, 3, "model"), Ok((0, 3, "model")));
        let twice = [models(), defines("6"), defines("7")];
        assert!(home(&twice, 0, 1, "model").is_err());
        let misspelt = [file_with(vec![("T".into(), 1, 2)], vec![], &["model"; 3]), defines("6")];
        assert!(home(&misspelt, 0, 1, "model").is_err());
    }

    #[test]
    fn the_header_names_every_count_field() {
        assert_eq!(COUNTS.split(',').count(), COUNT_FIELDS);
    }

    #[test]
    fn a_line_goes_to_its_function_row_else_the_rest_of_its_module_else_its_stage() {
        let row = |stage: &str, file, module: &str, line, span| Row {
            stage: stage.into(),
            file,
            module: module.into(),
            line,
            span,
            owner: String::new(),
            name: String::new(),
        };
        let file = |module: &str| CountedFile { module: module.into(), ..Default::default() };
        // file 0: a function row and the rest; file 1: two function rows of stage 6, `T::f` and
        // `T::g`, no rest; file 3: the rest only, of stage 6; file 2 holds no rustc function;
        // files 4 and 5 are one module `m`, whose rest row is file 4's; the rest of file 6 goes
        // into the function row `T::g`.
        let files = [file(""), file(""), file(""), file(""), file("m"), file("m"), file("T::g")];
        let function = |r: Row, name: &str| Row { owner: "T".into(), name: name.into(), ..r };
        let rows = [
            row("1", 0, "", 10, Some((8, 60))),
            row("1", 0, "", 70, None),
            row("2", 4, "m", 1, None),
            row("6", 3, "", 5, None),
            function(row("6", 1, "", 100, Some((99, 150))), "f"),
            function(row("6", 1, "", 200, Some((198, 300))), "g"),
        ];
        assert_eq!(row_of(&rows, &files, 0, 30, "1"), Some(0));
        assert_eq!(row_of(&rows, &files, 0, 5, "2"), Some(1));
        assert_eq!(row_of(&rows, &files, 5, 5, "3"), Some(2));
        assert_eq!(row_of(&rows, &files, 1, 250, "6"), Some(5));
        assert_eq!(row_of(&rows, &files, 1, 20, "6"), Some(4));
        assert_eq!(row_of(&rows, &files, 2, 1, "6"), Some(4));
        assert_eq!(row_of(&rows, &files, 2, 1, "1"), Some(0));
        assert_eq!(row_of(&rows, &files, 2, 1, "7"), None);
        assert_eq!(row_of(&rows, &files, 6, 1, "6"), Some(5));
        // a line of another stage does not go to the named function row
        assert_eq!(row_of(&rows, &files, 6, 1, "7"), None);
    }

    #[test]
    fn a_line_below_a_helpers_row_goes_to_the_function_row_it_names() {
        let function = |line, span: (usize, usize), name: &str| Row {
            stage: "6".into(),
            file: 0,
            module: String::new(),
            line,
            span: Some(span),
            owner: "T".into(),
            name: name.into(),
        };
        // file 0: `T::f` at 10 to 20, `T::g` at 30 to 40, and `T::g`'s helpers from line 25 on
        let helpers = vec![(25, "T::g".to_string())];
        let files = [CountedFile { helpers, ..Default::default() }];
        let rows = [function(10, (10, 20), "f"), function(30, (30, 40), "g")];
        assert_eq!(row_of(&rows, &files, 0, 5, "6"), Some(0));
        assert_eq!(row_of(&rows, &files, 0, 15, "6"), Some(0));
        assert_eq!(row_of(&rows, &files, 0, 26, "6"), Some(1));
        assert_eq!(row_of(&rows, &files, 0, 50, "6"), Some(1));
        let stages: BTreeMap<String, Stage> = [("6".to_string(), Stage::default())].into();
        assert!(check_helper_names(&files, &rows, &stages).is_ok());
        let misnamed = [CountedFile { helpers: vec![(25, "T::h".to_string())], ..Default::default() }];
        assert!(check_helper_names(&misnamed, &rows, &stages).is_err());
        // a line of another stage below the `helpers` row stops the count
        let stages: BTreeMap<String, Stage> = [("6".to_string(), Stage::default()), ("7".to_string(), Stage::default())].into();
        let helpers = vec![(25, "T::g".to_string())];
        let crossing = [CountedFile { helpers, spec: vec![(26, "7".to_string())], ..Default::default() }];
        assert!(check_helper_names(&crossing, &rows, &stages).is_err());
    }

    #[test]
    fn a_row_with_verified_functions_takes_the_roots_that_name_it() {
        let row = |module: &str, name: &str, span| Row {
            stage: "1".into(),
            file: 0,
            module: module.into(),
            line: 1,
            span,
            owner: String::new(),
            name: name.into(),
        };
        let rows = [row("m", "", None), row("", "f", Some((1, 50))), row("", "g", Some((60, 99)))];
        let roots = [("m.rs", "m"), ("f.rs", "f"), ("g.rs", "g")];
        assert_eq!(roots_of_rows(&rows, &roots, &[true, true, false]).unwrap(), [vec!["m.rs"], vec!["f.rs"], vec![]]);
        assert!(roots_of_rows(&rows, &roots[..2], &[true, true, true]).is_err());
        assert!(roots_of_rows(&rows, &[("h.rs", "h")], &[false, false, false]).is_err());
    }

    /// A counted function `name` of `part` with `loc` lines of code from `line`, with a body or none.

    /// A counted function `name` of `part` with `loc` lines of code from `line`, with a body or none.
    fn counted(name: &str, status: &'static str, part: &str, line: usize, loc: usize, body: bool) -> Counted {
        let f = Function {
            line,
            first_line: line,
            last_line: line + loc,
            owner: String::new(),
            of_trait: String::new(),
            name: name.into(),
            attrs: Vec::new(),
            contract: Vec::new(),
            generic: false,
            body: body.then(Vec::new),
            assumes: 0,
            added: false,
            in_trait: false,
        };
        Counted { f, status, part: part.into(), loops: 0, invariants: 0, loc, inferred: false }
    }

    #[test]
    fn a_rustc_function_of_row_lines_with_a_body_is_a_row_and_the_rest_of_its_file_another() {
        let file = |module: &str, fns| CountedFile { module: module.into(), fns, ..Default::default() };
        let files = [
            // a long verified function, a long declaration, a short function, a long predicate
            file("", vec![
                counted("long", "verified", "2", 10, ROW_LINES, true),
                counted("decl", "declaration", "2", 60, ROW_LINES, false),
                counted("short", "trusted", "1", 120, ROW_LINES - 1, true),
                counted("pred", "predicate", "2", 200, 90, true),
            ]),
            // only long functions, one in a part that is no stage
            file("", vec![
                counted("alone", "unchecked", "1", 5, 100, true),
                counted("copied", "verified", "copy", 300, 100, true),
            ]),
        ];
        let stages: BTreeMap<String, Stage> = ["1", "2"].map(|s| (s.to_string(), Stage::default())).into();
        let rows = table_rows(&files, &stages);
        let shape: Vec<(&str, usize, &str)> =
            rows.iter().map(|r| (r.stage.as_str(), r.file, r.name.as_str())).collect();
        // the rest of file 0 starts at its first rustc function without a row, `decl` of stage 2
        assert_eq!(shape, [("1", 1, "alone"), ("2", 0, "long"), ("2", 0, "")]);

        // two files of one module have one rest row, at the first of them
        let files = [
            file("m", vec![counted("a", "verified", "3", 1, 1, true)]),
            file("m", vec![counted("b", "verified", "2", 1, 1, true)]),
        ];
        let stages: BTreeMap<String, Stage> = ["2", "3"].map(|s| (s.to_string(), Stage::default())).into();
        let shape: Vec<(String, usize, String)> =
            table_rows(&files, &stages).into_iter().map(|r| (r.stage, r.file, r.module)).collect();
        assert_eq!(shape, [("3".to_string(), 0, "m".to_string())]);

        // a file whose `row` row names a function row has no rest row of its own
        let files = [
            file("", vec![counted("long", "verified", "2", 10, ROW_LINES, true)]),
            file("long", vec![counted("short", "trusted", "2", 1, 1, true)]),
        ];
        let stages: BTreeMap<String, Stage> = [("2".to_string(), Stage::default())].into();
        let shape: Vec<(usize, String)> = table_rows(&files, &stages).into_iter().map(|r| (r.file, r.name)).collect();
        assert_eq!(shape, [(0, "long".to_string())]);
    }

    #[test]
    fn a_row_name_is_a_function_row_of_the_same_stage_or_a_module_a_root_names() {
        let file = |module: &str, fns| CountedFile { module: module.into(), fns, ..Default::default() };
        let long = || counted("long", "verified", "2", 10, ROW_LINES, true);
        let stages: BTreeMap<String, Stage> = ["2", "3"].map(|s| (s.to_string(), Stage::default())).into();
        let check = |files: &[CountedFile], roots: &[(&str, &str)]| {
            check_row_names(files, &table_rows(files, &stages), &stages, roots)
        };
        // the function row `long`, of the rest's stage
        assert!(check(&[file("", vec![long()]), file("long", vec![counted("short", "trusted", "2", 1, 1, true)])], &[]).is_ok());
        // a module row a root names
        assert!(check(&[file("m", vec![counted("short", "trusted", "2", 1, 1, true)])], &[("m.rs", "m")]).is_ok());
        // a misspelt name, bare or `Type::name`
        assert!(check(&[file("", vec![long()]), file("lnog", vec![counted("short", "trusted", "2", 1, 1, true)])], &[]).is_err());
        assert!(check(&[file("", vec![long()]), file("T::long", vec![counted("short", "trusted", "2", 1, 1, true)])], &[]).is_err());
        // the function row `long` of stage 2, named by a rest of stage 3
        assert!(check(&[file("", vec![long()]), file("long", vec![counted("short", "trusted", "3", 1, 1, true)])], &[]).is_err());
    }

    /// The lines of code and the spec lines of `src`, whose upstream copy has the free functions
    /// `upstream`.
    fn kinds(src: &str, upstream: &[&str]) -> (Vec<usize>, Vec<usize>) {
        let ast: syn::File = syn::parse_str(src).unwrap();
        let mut spans = Spans::default();
        for name in upstream {
            spans.upstream.insert((String::new(), String::new(), name.to_string()), 1);
        }
        for item in &ast.items {
            spans.item(item);
        }
        let text: Vec<&str> = src.lines().collect();
        let (code, spec) = line_kinds(&ast, &text, &spans);
        (code.into_iter().collect(), spec.into_iter().collect())
    }

    #[test]
    fn an_inline_module_holding_only_annotations_is_spec_with_its_header_and_brace() {
        let src = "mod m {\n#[thrust::predicate]\nfn p() -> bool {\ntrue\n}\n}\nfn f() {\n}\n";
        assert_eq!(kinds(src, &["f"]), (vec![7, 8], vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn rustc_assert_is_code_and_assume_ghost_and_closure_contracts_are_spec() {
        let src = "fn f(x: u32) {
assert!(x > 0);
assume!(x < 9);
let g = thrust_macros::ghost!(|| -> u32 { x });
thrust_macros::proof!(lemma(g));
let c = thrust_macros::closure!(
requires(|y: u32| y > 0),
|y: u32| y + 1
);
}
";
        // line 4 holds code (`let g =`) and spec; the closure's body (line 8) is code
        assert_eq!(kinds(src, &["f"]), (vec![1, 2, 4, 6, 8, 9, 10], vec![3, 4, 5, 7]));
    }

    #[test]
    fn added_and_extern_spec_functions_are_spec_with_their_attributes() {
        let src = "#[inline]
fn helper() {
}
#[thrust::extern_spec_fn]
fn spec_of_len(v: &Vec<u32>) -> usize {
Vec::len(v)
}
fn f() {
}
";
        assert_eq!(kinds(src, &["f"]), (vec![8, 9], vec![1, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn an_impl_of_a_local_trait_method_without_a_full_contract_has_its_contract_inferred() {
        let src = "trait T {
#[thrust_macros::ensures(true)]
fn half(self) -> u32;
#[thrust_macros::requires(true)]
#[thrust_macros::ensures(true)]
fn full(self) -> u32;
}
impl T for u32 {
fn half(self) -> u32 { self }
fn full(self) -> u32 { self }
}
impl Clone for S {
fn clone(&self) -> S { S }
}
fn free() {}
#[thrust::callable]
fn called() {}
";
        let ast: syn::File = syn::parse_str(src).unwrap();
        let mut spans = Spans::default();
        for item in &ast.items {
            spans.item(item);
        }
        let traits: HashMap<(String, String), bool> = spans
            .fns
            .iter()
            .filter(|f| f.in_trait)
            .map(|f| ((f.owner.clone(), f.name.clone()), f.full_contract()))
            .collect();
        let inferred: Vec<(&str, bool)> =
            spans.fns.iter().filter(|f| !f.in_trait).map(|f| (f.name.as_str(), f.contract_inferred(&traits))).collect();
        // `half` takes no full contract from `T`; `clone` takes std's `Clone`'s
        assert_eq!(inferred, [("half", true), ("full", false), ("clone", false), ("free", true), ("called", false)]);
    }

    #[test]
    fn counts_assume_calls_and_opaque_attributes() {
        let src = r#"
            #[thrust::opaque]
            #[thrust::predicate]
            fn p(x: i32) -> bool { x > 0 }
            #[thrust::reveal(p)]
            #[thrust::reveal(q)]
            fn f() { assume!(p(1)); thrust_macros::assume!(p(2)); let assumed = 1; }
        "#;
        let ast: syn::File = syn::parse_str(src).unwrap();
        let mut spans = Spans::default();
        spans.upstream.insert((String::new(), String::new(), "f".into()), 1);
        for item in &ast.items {
            spans.item(item);
        }
        let (p, f) = (&spans.fns[0], &spans.fns[1]);
        assert_eq!((p.attr_count("opaque"), p.assumes), (1, 0));
        assert_eq!((f.attr_count("reveal"), f.assumes), (2, 2));
        assert_eq!(f.status(&HashSet::new()), "assumed");
    }

    #[test]
    fn an_added_trusted_helper_is_an_annotation() {
        let ast: syn::File = syn::parse_str("#[thrust::trusted] fn h() {}").unwrap();
        let mut spans = Spans::default();
        for item in &ast.items {
            spans.item(item);
        }
        let h = &spans.fns[0];
        assert_eq!(h.status(&HashSet::new()), "added");
        assert!(h.is_trusted("added"));
        let mut stage = Stage::default();
        stage.add(h, "added", 0, 0, false, 0);
        assert_eq!((stage.trusted, stage.trusted_helpers), (0, 1));

        let ast: syn::File = syn::parse_str("#[thrust::ignored] fn g() {}").unwrap();
        let mut spans = Spans::default();
        for item in &ast.items {
            spans.item(item);
        }
        let g = &spans.fns[0];
        assert_eq!(g.status(&HashSet::new()), "added");
        assert!(!g.is_trusted("added"));
        let mut stage = Stage::default();
        stage.add(g, "added", 0, 0, false, 0);
        assert_eq!((stage.trusted, stage.ignored, stage.trusted_helpers), (0, 0, 1));
    }

    #[test]
    fn a_trusted_function_with_a_contract_is_trusted() {
        let src = "#[thrust::trusted] #[thrust_macros::requires(true)] fn t() {} \
                   #[thrust::trusted] fn c() {} #[thrust::ignored] fn i() {}";
        let ast: syn::File = syn::parse_str(src).unwrap();
        let mut spans = Spans::default();
        for key in ["t", "c", "i"] {
            spans.upstream.insert((String::new(), String::new(), key.into()), 1);
        }
        for item in &ast.items {
            spans.item(item);
        }
        let status = |f: &Function| f.status(&HashSet::new());
        let (t, c, i) = (&spans.fns[0], &spans.fns[1], &spans.fns[2]);
        assert_eq!((status(t), status(c), status(i)), ("trusted", "trusted-callable", "ignored"));
        assert_eq!((t.is_trusted("trusted"), c.is_trusted("trusted-callable")), (true, true));
        assert!(!i.is_trusted("ignored"));
    }
}
