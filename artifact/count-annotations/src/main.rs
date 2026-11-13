//! Counts the lines and the annotations of the verified programs of the evaluation, the inputs of
//! the tables that `../scripts/tables.py` writes.
//!
//! usage: count-annotations creusot [--creusot] TREE CASE_ITEMS_CSV
//!        count-annotations coroutine [--functions | --rows] THRUST_TREE PARTS_CSV UPSTREAM_DIR
//!        count-annotations scalability PROGRAMS_DIR
//!        count-annotations prusti [--lines ID] DIR CASES_CSV
//!
//! Each command prints one CSV; its rules are in the header of its module.

mod coroutine;
mod creusot;
mod prusti;
mod scalability;

use coroutine::Output::{Functions, Rows, Stages};

use std::error::Error;
use std::ops::RangeInclusive;

use proc_macro2::{Delimiter, LineColumn, Span, TokenStream, TokenTree};

const USAGE: &str = "usage: count-annotations creusot [--creusot] TREE CASE_ITEMS_CSV
       count-annotations coroutine [--functions | --rows] THRUST_TREE PARTS_CSV UPSTREAM_DIR
       count-annotations scalability PROGRAMS_DIR
       count-annotations prusti [--lines ID] DIR CASES_CSV";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = match args[..] {
        ["creusot", tree, cases] => creusot::run(tree, cases, false),
        ["creusot", "--creusot", tree, cases] => creusot::run(tree, cases, true),
        ["coroutine", "--functions", tree, parts, up] => coroutine::run(tree, parts, up, Functions),
        ["coroutine", "--rows", tree, parts, up] => coroutine::run(tree, parts, up, Rows),
        ["coroutine", tree, parts, up] => coroutine::run(tree, parts, up, Stages),
        ["scalability", dir] => scalability::run(dir),
        ["prusti", "--lines", id, dir, cases] => prusti::run(dir, cases, Some(id)),
        ["prusti", dir, cases] => prusti::run(dir, cases, None),
        _ => Err(USAGE.into()),
    };
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// A position in the source, (line, column).
pub(crate) type Pos = (usize, usize);

/// A span of the source, [start, end).
pub(crate) type Range = (Pos, Pos);

pub(crate) fn pos(lc: LineColumn) -> Pos {
    (lc.line, lc.column)
}

pub(crate) fn range(span: Span) -> Range {
    (pos(span.start()), pos(span.end()))
}

pub(crate) fn within(r: &Range, p: Pos) -> bool {
    r.0 <= p && p < r.1
}

/// A token of the source; an opening delimiter has no text and carries its group's delimiter and
/// end.
pub(crate) struct Token {
    pub(crate) text: String,
    pub(crate) range: Range,
    group: Option<(Delimiter, Pos)>,
}

/// Whether a token is a separator, `,` or `;`, which alone does not make a line code: the one
/// after a where-predicate or a macro statement lies outside their span.
pub(crate) fn separator(t: &Token) -> bool {
    t.text == "," || t.text == ";"
}

/// The tokens of `stream` in source order, doc comments left out.
pub(crate) fn tokens(stream: TokenStream) -> Vec<Token> {
    let mut out = Vec::new();
    flatten(stream, &mut out);
    out
}

fn flatten(stream: TokenStream, out: &mut Vec<Token>) {
    let trees: Vec<TokenTree> = stream.into_iter().collect();
    let mut i = 0;
    while i < trees.len() {
        if doc_at(&trees[i..]) {
            i += 2;
            continue;
        }
        let r = range(trees[i].span());
        if let TokenTree::Group(g) = &trees[i] {
            let group = Some((g.delimiter(), r.1));
            out.push(Token { text: String::new(), range: range(g.span_open()), group });
            flatten(g.stream(), out);
            out.push(Token { text: String::new(), range: range(g.span_close()), group: None });
        } else {
            out.push(Token { text: trees[i].to_string(), range: r, group: None });
        }
        i += 1;
    }
}

/// Whether the tokens start with a doc comment, `#` and `[doc = ..]`.
fn doc_at(trees: &[TokenTree]) -> bool {
    let [TokenTree::Punct(p), TokenTree::Group(g), ..] = trees else { return false };
    p.as_char() == '#'
        && g.delimiter() == Delimiter::Bracket
        && matches!(g.stream().into_iter().next(), Some(TokenTree::Ident(i)) if i == "doc")
}

/// The source of a file and its syntax tree.
fn parse(path: &str) -> Result<(String, syn::File), Box<dyn Error>> {
    let source = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let file = syn::parse_file(&source).map_err(|e| format!("{path}: {e}"))?;
    Ok((source, file))
}

/// The rows of a CSV file after its header; no field holds a comma or a quote.
fn read_csv(path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    Ok(text.lines().skip(1).map(|l| l.split(',').map(String::from).collect()).collect())
}

/// The lines a span covers.
fn lines(span: Span) -> RangeInclusive<usize> {
    span.start().line..=span.end().line
}

/// Whether a line of the source is blank or a `//` comment.
fn blank(text: &str) -> bool {
    let t = text.trim();
    t.is_empty() || t.starts_with("//")
}

fn last_segment(path: &syn::Path) -> String {
    path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
}

/// The name of a `#[thrust_macros::..]` or `#[thrust::..]` attribute.
fn thrust_attr(a: &syn::Attribute) -> Option<String> {
    let first = a.path().segments.first()?.ident.to_string();
    (first == "thrust_macros" || first == "thrust").then(|| last_segment(a.path()))
}

fn thrust_attrs(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs.iter().filter_map(thrust_attr).collect()
}

fn is_model_impl(i: &syn::ItemImpl) -> bool {
    matches!(&i.trait_, Some((_, path, _)) if last_segment(path) == "Model")
}
