#!/usr/bin/env python3
"""Bottom-up review order for the RQ4 case study (stages 1 to 4).

Usage: review_order.py FORK_CHECKOUT [--stage N ...] [--functions-csv FILE] [--list]

Counts the functions of the stages from FORK_CHECKOUT with `count-annotations coroutine --functions`
(built by cargo, rules in the header of its src/coroutine.rs) unless --functions-csv names a file
made that way, and reads the Rust files of FORK_CHECKOUT/tests/rustc_coroutine/ for everything
else. The functions.csv next to this script is never read: it records the commit it was last
regenerated at, not the checkout under review. A file given with --functions-csv is not checked
against the checkout; a row it holds that the tree does not have is listed at the end and sets the
exit status to 1. --list prints the selected functions, one per line, and stops, to check a
selection without a scan.
It prints a Markdown checklist:
every function with its location, stage, status, contract (the `contract` column of the counted
rows, and `#[thrust::callable]` from the source), panic sites and call sites, in layers.
Layer 0 holds the functions that call no other function of the selection; a function is one layer
above the highest layer it calls; mutually recursive functions form one group. Trusted functions (the `trusted` column of the counted rows: trusted functions and added helpers
marked trusted)
get an item to check the contract against rustc's code and a link to the upstream copy.
The output depends only on the inputs and is not meant to be committed.

This is a static approximation made with regular expressions, not a Rust parser:

- A row of the counted functions is paired with the function of the same file, owner and name nearest to
  its line; a row with none is listed at the end and the exit status is 1.
- Functions are `fn` items found after comments and string literals are blanked; the body is the
  brace-matched block. Nested fns count as part of the enclosing body. The owner of a method is the
  type of the enclosing `impl` or the name of the enclosing `trait`.
- Calls are `name(`, `Type::name(` and `.name(` in a body. `Self::` is the enclosing owner. A call is
  linked to every function of the tree that fits by name (and by owner when written qualified), so
  a callee reached through a trait or a common method name may be linked to more functions than it
  reaches (a method call inside a function of the same name is not linked to that function). A call
  written `Type::name` links only to functions of an impl or trait of that name in the tree (a
  type from outside the tree, or a type parameter, links to none); one written `module::name` or
  bare links to the free functions of that name. For
  the layers, a cycle that exists only through such name matches is broken by keeping the calls
  that fit exactly one function; calls to std or to functions outside the tree are not listed. Macros and calls in
  contracts are not followed.
- Panic sites, per line of a body, of these kinds: `index` (`x[..]`), `unwrap`, `expect`, `panic!`,
  `assert!`/`assert_eq!`/`assert_ne!`, `unreachable!`/`unimplemented!`/`todo!`, `div` (`/`, `%`
  and their assignment forms) and `sub` (a `-` with an operand that is a parameter or `let`
  annotated usize/u32/u64, a method call such as `.len()` or `align.bytes()`, or an
  `as usize/u32/u64` cast; the type of a method call's result is not known, so a signed one is
  listed too). Other overflow, slice
  range checks inside called code and `debug_assert*!` are not detected; a missing site is not
  evidence that there is none, and a listed one may be guarded.
"""

import argparse
import csv
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
TREE = "tests/rustc_coroutine"
ROOTS = "tests/ui/pass/rustc_coroutine"   # the stage roots the counter reads
CHECKED = ("verified", "trusted", "trusted-callable", "ignored", "unchecked", "added")
PRIMITIVES = {"u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128",
              "isize", "f32", "f64", "bool", "char", "str"}
UNSIGNED = ("usize", "u32", "u64")
KEYWORDS = {"if", "while", "for", "match", "fn", "return", "loop", "let", "in", "as", "move",
            "else", "where", "impl", "dyn", "mut", "ref", "Some", "Ok", "Err", "Box", "Vec"}


# ---- source text ----------------------------------------------------------------------------

def blank(text, keep_strings):
    """Replace comments (and string literals unless kept) by spaces, keeping offsets and lines."""
    out, i, n = [], 0, len(text)
    while i < n:
        two = text[i:i + 2]
        if two == "//":
            j = text.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i))
            i = j
        elif two == "/*":
            j = text.find("*/", i + 2)
            j = n if j < 0 else j + 2
            out.append(re.sub(r"[^\n]", " ", text[i:j]))
            i = j
        elif text[i] == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j = min(j + 1, n)
            seg = text[i:j]
            out.append(seg if keep_strings else '"' + re.sub(r"[^\n]", " ", seg[1:-1]) + '"')
            i = j
        elif text[i] == "'" and re.match(r"'(\\.[^']*|[^\\'])'", text[i:i + 8]):
            j = text.find("'", i + 2 if text[i + 1] != "\\" else i + 3) + 1
            out.append("'" + " " * (j - i - 2) + "'")
            i = j
        else:
            out.append(text[i])
            i += 1
    return "".join(out)


def match_close(text, start, open_ch, close_ch):
    depth = 0
    for i in range(start, len(text)):
        if text[i] == open_ch:
            depth += 1
        elif text[i] == close_ch:
            depth -= 1
            if depth == 0:
                return i
    return len(text) - 1


def line_of(text, pos):
    return text.count("\n", 0, pos) + 1


# ---- items ----------------------------------------------------------------------------------

class Fn:
    def __init__(self, path, name, owner, line):
        self.path, self.name, self.owner, self.line = path, name, owner, line
        self.body = ""        # code of the body, strings blanked
        self.body_line = 0    # line of the first body character
        self.params = ""
        self.contract = []    # requires/ensures attributes, from the counted row
        self.callable = False  # `#[thrust::callable]` on the function, from the source
        self.calls = []       # (line, qualifier or None, is_method, name)
        self.stages = set()
        self.status = set()
        self.trusted = False  # the counted row's `trusted` column: the verdict relies on the contract

    @property
    def key(self):
        return (self.path, self.line)

    def label(self):
        return f"{self.owner}::{self.name}" if self.owner else self.name


def impl_owner(header):
    header = re.sub(r"^\s*(unsafe\s+)?impl\s*(<.*?>)?\s*", "", header, flags=re.S)
    header = re.sub(r"\bwhere\b.*", "", header, flags=re.S)
    if " for " in header:
        header = header.split(" for ", 1)[1]
    m = re.search(r"[A-Za-z_]\w*", header.replace("&", " ").replace("mut ", " "))
    return m.group(0) if m else None


def owner_spans(code):
    """(start, end, owner) of every impl and trait block."""
    spans = []
    for m in re.finditer(r"\b(impl|trait)\b", code):
        j, depth = m.end(), 0
        while j < len(code) and not (code[j] == "{" and depth <= 0) and not (
                code[j] == ";" and depth <= 0):
            depth += {"[": 1, "]": -1, "(": 1, ")": -1}.get(code[j], 0)
            j += 1
        if j >= len(code) or code[j] != "{":
            continue
        header = code[m.end():j]
        end = match_close(code, j, "{", "}")
        if m.group(1) == "trait":
            owner = re.match(r"\s*([A-Za-z_]\w*)", header).group(1)
        else:
            owner = impl_owner("impl " + header)
        spans.append((m.start(), end, owner))
    return spans


def attributes_before(code, pos):
    """The attributes `#[..]` directly above the item whose `fn` keyword is at pos, as texts
    without whitespace."""
    head = re.sub(r"(?:(?:pub(?:\s*\([^)]*\))?|const|unsafe|async|default)\s+)*$", "", code[:pos])
    attrs = []
    while head.rstrip().endswith("]"):
        head = head.rstrip()
        depth, i = 0, len(head) - 1
        while i >= 0:
            depth += {"]": 1, "[": -1}.get(head[i], 0)
            if depth == 0:
                break
            i -= 1
        start = head[:i].rstrip()
        if not start.endswith("#"):
            break
        attrs.append(re.sub(r"\s+", "", head[len(start) - 1:]))
        head = start[:-1]
    return attrs


def parse_fns(path, text):
    code = blank(text, keep_strings=False)
    spans = owner_spans(code)
    fns = []
    for m in re.finditer(r"\bfn\s+([A-Za-z_]\w*)", code):
        owners = [s for s in spans if s[0] < m.start() < s[1]]
        owner = min(owners, key=lambda s: s[1] - s[0])[2] if owners else None
        fn = Fn(path, m.group(1), owner, line_of(code, m.start()))
        fn.callable = "#[thrust::callable]" in attributes_before(code, m.start())
        j, depth = m.end(), 0
        while j < len(code):
            c = code[j]
            if c in "([<" and not (c == "<" and code[j - 1] == "-"):
                depth += 1
            elif c in ")]>" and not (c == ">" and code[j - 1] == "-"):
                depth -= 1
            elif depth <= 0 and c in "{;":
                break
            j += 1
        fn.params = code[m.end():j]
        if j < len(code) and code[j] == "{":
            end = match_close(code, j, "{", "}")
            fn.body = code[j + 1:end]
            fn.body_line = line_of(code, j + 1)
        fns.append(fn)
    return fns


def load_tree(fork):
    fns = []
    root = os.path.join(fork, TREE)
    for dirpath, dirs, files in os.walk(root):
        dirs.sort()
        for f in sorted(files):
            if f.endswith(".rs"):
                full = os.path.join(dirpath, f)
                rel = os.path.relpath(full, fork)
                with open(full, encoding="utf-8") as fh:
                    fns.extend(parse_fns(rel, fh.read()))
    return fns


# ---- the checked set ------------------------------------------------------------------------

def count_functions(fork):
    """The rows `count-annotations coroutine --functions` makes of the tree of the checkout."""
    cmd = ["cargo", "run", "-q", "--release", "--manifest-path",
           os.path.join(HERE, "..", "count-annotations", "Cargo.toml"), "--", "coroutine",
           "--functions", fork, os.path.join(HERE, "parts.csv"), os.path.join(HERE, "upstream")]
    done = subprocess.run(cmd, capture_output=True, text=True)
    if done.returncode != 0:
        sys.exit("count-annotations failed:\n" + done.stderr)
    return list(csv.DictReader(done.stdout.splitlines()))


def counted_rows(fork, csv_file):
    if csv_file is None:
        return count_functions(fork)
    with open(csv_file, encoding="utf-8") as fh:
        return list(csv.DictReader(fh))


def read_rows(rows, stages):
    return [r for r in rows if int(r["stage"]) in stages and r["status"] in CHECKED]


def checkout_state(fork):
    """`HEAD` of the checkout, with a note when the tree under review has uncommitted changes."""
    git = ["git", "-C", fork]
    head = subprocess.run(git + ["rev-parse", "--short=8", "HEAD"], capture_output=True, text=True)
    if head.returncode != 0:
        return "a checkout that is not a git repository"
    dirty = subprocess.run(git + ["status", "--porcelain", "--", TREE, ROOTS], capture_output=True, text=True)
    return head.stdout.strip() + (", with uncommitted changes under " + TREE + " or " + ROOTS if dirty.stdout else "")


def csv_path(row):
    return TREE + "/" + row["file"].split("rustc_coroutine/", 1)[1]


def find_fn(fns, row):
    path, name, line = csv_path(row), row["function"], int(row["line"])
    cands = [f for f in fns if f.path == path and f.name == name]
    exact = [f for f in cands if (f.owner or "") == row["type"]]
    return min(exact, key=lambda f: abs(f.line - line)) if exact else None


def split_contract(text):
    return [c.strip() for c in re.split(r"(?=#\[thrust_macros::)", text) if c.strip()]


def select(fns, rows):
    selected, missing = {}, []
    for row in rows:
        fn = find_fn(fns, row)
        if fn is None:
            missing.append(row)
            continue
        fn.stages.add(int(row["stage"]))
        fn.status.add(row["status"])
        fn.trusted = fn.trusted or row["trusted"] == "true"
        fn.contract = split_contract(row["contract"])
        selected[fn.key] = fn
    return selected, missing


# ---- calls ----------------------------------------------------------------------------------

CALL = re.compile(r"(\.\s*)?((?:[A-Za-z_]\w*\s*(?:::\s*<[^;(){}]*?>\s*)?::\s*)*)([A-Za-z_]\w*)\s*(?:::\s*<[^;(){}]*>)?\s*\(")


def find_calls(fn):
    calls = []
    for m in CALL.finditer(fn.body):
        name = m.group(3)
        before = fn.body[:m.start()].rstrip()
        if name in KEYWORDS or before.endswith("fn") or before.endswith("->"):
            continue
        quals = [q.strip() for q in m.group(2).split("::") if q.strip()]
        qual = quals[-1] if quals else None
        if qual is None and before.endswith(">::"):
            qual = "?"
        if qual == "Self":
            qual = fn.owner
        calls.append((fn.body_line + fn.body.count("\n", 0, m.start(3)), qual,
                      bool(m.group(1)), name))
    fn.calls = calls


def resolve(fn, call, by_name, owners):
    _, qual, is_method, name = call
    cands = by_name.get(name, [])
    if is_method:
        return [f for f in cands if f.owner]
    if qual == "?":   # `<T as Trait>::name`: the trait is not known
        return cands
    if qual in owners:
        return [f for f in cands if f.owner == qual]
    if qual in PRIMITIVES:
        return []
    if qual is None or qual[0].islower():   # bare, or `module::name`
        return [f for f in cands if not f.owner]
    return []   # `Type::name` of a type that has no function in the tree


def call_graph(fns):
    by_name, owners = {}, {f.owner for f in fns if f.owner}
    for f in fns:
        by_name.setdefault(f.name, []).append(f)
        find_calls(f)
    callees = {f.key: [] for f in fns}
    sure = {f.key: [] for f in fns}   # edges whose call fits exactly one function
    callers = {f.key: [] for f in fns}
    for f in fns:
        for call in f.calls:
            found = resolve(f, call, by_name, owners)
            for g in found:
                if call[2] and g is f:   # `self.x.name()` inside `name` delegates, it is no recursion
                    continue
                callees[f.key].append(g.key)
                if len(found) == 1:
                    sure[f.key].append(g.key)
                callers[g.key].append((f, call[0]))
    return callees, sure, callers


# ---- layers ---------------------------------------------------------------------------------

def components(keys, edges):
    """Strongly connected components (Tarjan), in a deterministic order."""
    index, low, stack, on, out, counter = {}, {}, [], set(), [], [0]

    def visit(v):
        index[v] = low[v] = counter[0]
        counter[0] += 1
        stack.append(v)
        on.add(v)
        for w in edges[v]:
            if w not in index:
                visit(w)
                low[v] = min(low[v], low[w])
            elif w in on:
                low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop()
                on.discard(w)
                comp.append(w)
                if w == v:
                    break
            out.append(sorted(comp))

    sys.setrecursionlimit(10000)
    for k in sorted(keys):
        if k not in index:
            visit(k)
    return out


def layer_edges(keys, callees, sure):
    """All edges, except that a cycle made of edges that fit several functions by name is
    broken by keeping only the edges that fit exactly one."""
    edges = {k: sorted({c for c in callees[k] if c in keys}) for k in keys}
    final = {k: set(v) for k, v in edges.items()}
    for comp in components(keys, edges):
        if len(comp) < 2:
            continue
        inside = set(comp)
        for k in comp:
            final[k] = {c for c in edges[k] if c not in inside or c in sure[k]}
    return {k: sorted(v) for k, v in final.items()}


def layers(selected, callees, sure):
    keys = set(selected)
    edges = layer_edges(keys, callees, sure)
    comps = components(keys, edges)
    comp_of = {k: i for i, c in enumerate(comps) for k in c}
    height = {}

    def h(i):
        if i not in height:
            below = {comp_of[w] for k in comps[i] for w in edges[k] if comp_of[w] != i}
            height[i] = 1 + max(map(h, below)) if below else 0
        return height[i]

    grouped = {}
    for i, c in enumerate(comps):
        grouped.setdefault(h(i), []).append(c)
    for comps_in_layer in grouped.values():
        comps_in_layer.sort(key=lambda c: min((min(selected[k].stages), k) for k in c))
    return grouped, edges


# ---- panic sites ----------------------------------------------------------------------------

PANIC_PATTERNS = [
    ("index", re.compile(r"[A-Za-z0-9_\)\]]\[")),
    ("unwrap", re.compile(r"\.\s*unwrap\s*\(")),
    ("expect", re.compile(r"\.\s*expect\s*\(")),
    ("panic!", re.compile(r"\bpanic\s*!")),
    ("assert", re.compile(r"\bassert(?:_eq|_ne)?\s*!")),
    ("unreachable", re.compile(r"\b(?:unreachable|unimplemented|todo)\s*!")),
    ("div", re.compile(r"[^/*]/(?![/*])|%")),
]


def unsigned_names(fn):
    names = set()
    for m in re.finditer(r"\b([a-z_]\w*)\s*:\s*(?:%s)\b" % "|".join(UNSIGNED), fn.params):
        names.add(m.group(1))
    for m in re.finditer(r"\blet\s+(?:mut\s+)?([a-z_]\w*)\s*:\s*(?:%s)\b" % "|".join(UNSIGNED),
                         fn.body):
        names.add(m.group(1))
    for m in re.finditer(r"\blet\s+(?:mut\s+)?([a-z_]\w*)\s*=[^;]*\bas\s+(?:%s)\b"
                         % "|".join(UNSIGNED), fn.body):
        names.add(m.group(1))
    return names


def sub_sites(line, names):
    for m in re.finditer(r"(?<![-=<!])-(?![->=])", line):
        left = line[:m.start()].rstrip()
        right = line[m.end():].lstrip()
        if not left or not re.search(r"[\w\)\]]$", left):
            continue
        call_left = re.search(r"\.\s*[A-Za-z_]\w*\s*(?:::\s*<[^;(){}]*>)?\s*\([^()]*\)$", left)
        call_right = re.match(r"(?:[A-Za-z_]\w*\s*\.\s*)+[A-Za-z_]\w*\s*\(", right)
        lm = re.search(r"([A-Za-z_]\w*)$", left)
        rm = re.match(r"([A-Za-z_]\w*)", right)
        hit = (lm and lm.group(1) in names) or (rm and rm.group(1) in names)
        hit = hit or call_left or call_right or re.match(r"[^;]*?\bas\s+(%s)\b" % "|".join(UNSIGNED), right[:30]) \
            or re.search(r"\bas\s+(%s)$" % "|".join(UNSIGNED), left)
        if hit:
            return True
    return False


def panic_sites(fn):
    names, sites = unsigned_names(fn), []
    for off, line in enumerate(fn.body.split("\n")):
        no = fn.body_line + off
        kinds = [k for k, rx in PANIC_PATTERNS if rx.search(line)]
        if sub_sites(line, names):
            kinds.append("sub")
        if kinds:
            sites.append((no, kinds))
    return sites


# ---- output ---------------------------------------------------------------------------------

def upstream_link(fn):
    """`path:line` of the function of the same owner and name in the upstream copy, if exactly one."""
    rel = fn.path.split("tests/rustc_coroutine/", 1)[1]
    up = os.path.join(HERE, "upstream", rel)
    if not os.path.exists(up):
        return None
    with open(up, encoding="utf-8") as fh:
        same = [g for g in parse_fns(rel, fh.read()) if g.name == fn.name and g.owner == fn.owner]
    if len(same) != 1:
        return None
    return os.path.relpath(up, REPO_ROOT) + f":{same[0].line}"


def stage_text(fn):
    return ",".join(str(s) for s in sorted(fn.stages))


def render_fn(fn, callers):
    status = "/".join(sorted(fn.status))
    out = [f"- [ ] `{fn.label()}` at `{fn.path}:{fn.line}`, stage {stage_text(fn)}, {status}"]
    clauses = fn.contract
    if clauses:
        out.append("  - contract as written:")
        out.extend(f"    - `{c}`" for c in clauses)
    if fn.callable:
        out.append("  - callable: `#[thrust::callable]`, an alias of `requires(true)` and `ensures(true)`")
    if not clauses and not fn.callable:
        out.append("  - contract as written: none")
    if fn.trusted:
        link = upstream_link(fn)
        out.append("  - [ ] check: the contract is true of rustc's code; upstream copy: "
                   + (f"`{link}`" if link else "none found for this owner and name"))
    sites = panic_sites(fn)
    if sites:
        out.append("  - panic sites (static scan):")
        out.extend(f"    - [ ] `{fn.path}:{no}`: {', '.join(kinds)}" for no, kinds in sites)
    else:
        out.append("  - panic sites (static scan): none detected")
    calls = sorted({(c.path, line, c.label()) for c, line in callers[fn.key]})
    if calls:
        out.append("  - [ ] contract against its call sites:")
        for path, line, label in calls:
            out.append(f"    - `{path}:{line}` in `{label}`")
    else:
        out.append("  - call sites in the tree: none")
    return out


def render(selected, callees, sure, callers, stages, missing, source):
    grouped, _ = layers(selected, callees, sure)
    out = ["# Review order: rustc coroutine layout, stages "
           + ", ".join(map(str, sorted(stages))), "",
           f"{len(selected)} functions in {sum(len(v) for v in grouped.values())} groups, "
           f"{len(grouped)} layers. Layer 0 calls no other listed function; "
           "each later layer calls only earlier ones, except inside a group of mutually "
           "recursive functions. Static scan, see the header of review_order.py for its limits.",
           "", f"Functions counted {source}.", ""]
    for layer in sorted(grouped):
        out += [f"## Layer {layer}", ""]
        for comp in grouped[layer]:
            if len(comp) > 1:
                out += ["### Mutually recursive group: "
                        + ", ".join(f"`{selected[k].label()}`" for k in comp), ""]
            for k in comp:
                out += render_fn(selected[k], callers) + [""]
    if missing:
        out += ["## Counted rows not found in the tree", ""]
        out += [f"- stage {r['stage']} `{r['type']}::{r['function']}` at `{csv_path(r)}:{r['line']}`"
                for r in missing]
        out.append("")
    return "\n".join(out)


def list_selection(selected, missing):
    for fn in sorted(selected.values(), key=lambda f: (min(f.stages), f.key)):
        print(f"{stage_text(fn)}\t{'/'.join(sorted(fn.status))}\t{fn.label()}\t{fn.path}:{fn.line}")
    for r in missing:
        print(f"{r['stage']}\t{r['status']}\t{r['type']}::{r['function']}\t{csv_path(r)}:{r['line']}"
              "\tnot found in the tree")
    return 1 if missing else 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("fork", help="checkout of the Thrust fork")
    ap.add_argument("--stage", type=int, action="append", choices=[1, 2, 3, 4],
                    help="stage to include, repeatable (default: 1 to 4)")
    ap.add_argument("--functions-csv", help="counted functions of the checkout, as made by "
                    "`count-annotations coroutine --functions` (default: count them now)")
    ap.add_argument("--list", action="store_true", help="print the selected functions and stop")
    args = ap.parse_args()
    stages = set(args.stage or [1, 2, 3, 4])
    fns = load_tree(args.fork)
    selected, missing = select(fns, read_rows(counted_rows(args.fork, args.functions_csv), stages))
    if args.list:
        return list_selection(selected, missing)
    callees, sure, callers = call_graph(fns)
    source = (f"from {args.functions_csv}, not checked against the checkout" if args.functions_csv
              else "now with the parts.csv of this repository (the counter stops when a part's "
                   "`from` matches no line) from the checkout")
    print(render(selected, callees, sure, callers, stages, missing,
                 f"{source} at {checkout_state(args.fork)}"))
    return 1 if missing else 0


if __name__ == "__main__":
    sys.exit(main())
