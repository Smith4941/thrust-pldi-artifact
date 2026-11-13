#!/usr/bin/env python3
"""Write the programs of the scalability evaluation (RQ3) and their manifest (see README.md).

usage: generate.py [-n]

Every variant uses one trait `Step` and the generic function `repeat<T: Step>` with a loop:

  inferred_k<k>      k unrelated types S_0 .. S_{k-1}; `main` calls `repeat` once per type; the
                     loop invariant of `repeat` is left to inference
  written_k<k>       the same with the loop invariant written
  nested_k<k>        k generic wrappers W_0 .. W_{k-1}, each around the previous one and the
                     innermost around a base type; `main` calls `repeat` once on the outermost;
                     the loop invariant is left to inference
  perinstance_t<j>   S_j alone with a copy `repeat_s<j>` of `repeat` specialised by hand to S_j
                     (the contract instantiated at S_j, the invariant inferred); the variant at
                     size k is the k programs t0 .. t(k-1), each verified in its own run

The fail twin of every program is the program with the loop of `repeat` (`repeat_s<j>`) stopping
one step early, so the body breaks the postcondition `result >= x + n` at n = 1 and the query must
be unsat; the call sites are those of the pass side.

The programs go to programs/{pass,fail}/ next to this script, with programs/manifest.json. -n
writes nothing and prints each file with `same`, `changed` or `new` against the files there.
"""

import argparse
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SIZES = [1, 2, 4, 8, 16, 32]
TIMEOUT_SECS = 300
REPEAT_COUNT = 3  # the iteration count every call passes
LOOP = "    while i < n {\n"
LOOP_SHORT = "    while i + 1 < n {\n"  # the fail twin's loop, one step short
TWIN = " Fail twin: the loop of `{}` stops one step early, so its body breaks its postcondition."


def header(expect_pass, env=""):
    """The ui_test header; `env` is further `K=V` pairs of the rustc env, after the solver's."""
    first = "//@check-pass" if expect_pass else "//@error-in-other-file: Unsat"
    extra = f" {env}" if env else ""
    return (
        f"{first}\n"
        "//@compile-flags: -C debug-assertions=off\n"
        f"//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper{extra} THRUST_SOLVER_TIMEOUT_SECS={TIMEOUT_SECS}\n"
    )


TRAIT = """\
#[thrust_macros::context]
trait Step {
    #[thrust_macros::requires(Self::inv(x))]
    #[thrust_macros::ensures(Self::inv(result) && result > x)]
    fn step(&self, x: i64) -> i64;

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool;
}
"""

REPEAT_INFERRED = """\
#[thrust_macros::requires(T::inv(x) && n >= 0)]
#[thrust_macros::ensures(T::inv(result) && result >= x + n)]
fn repeat<T: Step>(a: &T, x: i64, n: i64) -> i64 {
    let mut v = x;
    let mut i = 0;
    while i < n {
        v = a.step(v);
        i += 1;
    }
    v
}
"""

REPEAT_WRITTEN = """\
#[thrust_macros::context]
#[thrust_macros::requires(T::inv(x) && n >= 0)]
#[thrust_macros::ensures(T::inv(result) && result >= x + n)]
fn repeat<T: Step>(a: &T, x: i64, n: i64) -> i64 {
    let mut v = x;
    let mut i = 0;
    while i < n {
        thrust_macros::invariant!(
            |v: i64, i: i64, x: thrust_models::FnParam<i64>| T::inv(v) && v >= x.at_entry() + i
        );
        v = a.step(v);
        i += 1;
    }
    v
}
"""

BASE = """\
#[derive(PartialEq)]
struct Base {}

impl thrust_models::Model for Base {
    type Ty = Base;
}

#[thrust_macros::context]
impl Step for Base {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 0
    }
}
"""


def loop(text, expect_pass):
    """The generic function's text, its loop one step short on the fail side."""
    assert text.count(LOOP) == 1
    return text if expect_pass else text.replace(LOOP, LOOP_SHORT)


def repeat_specialised(j):
    return f"""\
#[thrust_macros::requires(S{j}::inv(x) && n >= 0)]
#[thrust_macros::ensures(S{j}::inv(result) && result >= x + n)]
fn repeat_s{j}(a: &S{j}, x: i64, n: i64) -> i64 {{
    let mut v = x;
    let mut i = 0;
    while i < n {{
        v = a.step(v);
        i += 1;
    }}
    v
}}
"""


def instance(j):
    """Type S_j: its predicate is `x >= j` and its step adds (j mod 3) + 1."""
    return f"""\
#[derive(PartialEq)]
struct S{j} {{}}

impl thrust_models::Model for S{j} {{
    type Ty = S{j};
}}

#[thrust_macros::context]
impl Step for S{j} {{
    fn step(&self, x: i64) -> i64 {{
        x + {j % 3 + 1}
    }}

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {{
        x >= {j}
    }}
}}
"""


def wrapper(j):
    """Wrapper W_j around any T: Step: its step is T's and its predicate is T's and `x >= j + 1`."""
    return f"""\
#[derive(PartialEq)]
struct W{j}<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W{j}<T> {{
    type Ty = W{j}<<T as thrust_models::Model>::Ty>;
}}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W{j}<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{{
    fn step(&self, x: i64) -> i64 {{
        self.0.step(x)
    }}

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {{
        T::inv(x) && x >= {j + 1}
    }}
}}
"""


def call(name, callee, arg, start):
    return (
        f"    let r_{name} = {callee}(&{arg}, {start}, {REPEAT_COUNT});\n"
        f"    assert!(r_{name} >= {start + REPEAT_COUNT});\n"
    )


def instance_call(j, callee):
    """The call at S_j with the least start value S_j's predicate allows."""
    return call(f"s{j}", callee, f"S{j} {{}}", j)


def program(expect_pass, what, callee, items, main_body):
    comment = f"// Scalability evaluation, {what}." + ("" if expect_pass else TWIN.format(callee))
    parts = [header(expect_pass), "\n", comment, "\n\n", TRAIT]
    for item in items:
        parts += ["\n", item]
    parts += ["\nfn main() {\n", main_body, "}\n"]
    return "".join(parts)


def flat_program(variant, k, expect_pass):
    """inferred_k<k> or written_k<k>: k unrelated types, one call per type."""
    items = [loop(REPEAT_WRITTEN if variant == "written" else REPEAT_INFERRED, expect_pass)]
    items += [instance(j) for j in range(k)]
    calls = [instance_call(j, "repeat") for j in range(k)]
    return program(expect_pass, f"variant {variant}, k = {k}", "repeat", items, "".join(calls))


def nested_program(k, expect_pass):
    """nested_k<k>: W_{k-1}(... W_0(Base)), one call. The outermost predicate is `x >= k`."""
    items = [loop(REPEAT_INFERRED, expect_pass), BASE] + [wrapper(j) for j in range(k)]
    lines = ["    let c0 = W0(Base {});\n"]
    lines += [f"    let c{j} = W{j}(c{j - 1});\n" for j in range(1, k)]
    lines.append(call("all", "repeat", f"c{k - 1}", k))
    return program(expect_pass, f"variant nested, k = {k}", "repeat", items, "".join(lines))


def perinstance_program(j, expect_pass):
    items = [instance(j), loop(repeat_specialised(j), expect_pass)]
    what = f"variant perinstance, the instantiation at S{j} alone; the variant at size k is t0 .. t(k-1)"
    return program(expect_pass, what, f"repeat_s{j}", items, instance_call(j, f"repeat_s{j}"))


def annotation_items(text):
    """Annotation items as the evaluation counts them: contract clauses, predicate
    declarations and definitions, loop invariants."""
    contracts = len(re.findall(r"#\[thrust_macros::(requires|ensures)\(", text))
    predicates = len(re.findall(r"#\[thrust_macros::predicate\]", text))
    invariants = len(re.findall(r"thrust_macros::invariant!", text))
    return contracts + predicates + invariants


def skip_block(lines, i):
    """The index after the block that starts at lines[i - 1] and ends at the next `}`."""
    while i < len(lines) and lines[i] != "}":
        i += 1
    return i + 1


def code_lines(text):
    """Lines of Rust code under verification, without annotations: blank lines, comments,
    headers, attributes, loop invariants, predicate declarations and definitions, the `Model`
    impls (which only name the logical model of a type) and the `where` clause that bounds a
    wrapper's model are not counted."""
    lines = [l.strip() for l in text.splitlines()]
    count = 0
    i = 0
    while i < len(lines):
        s = lines[i]
        i += 1
        if not s or s.startswith("//") or s.startswith("#["):
            continue
        if s.startswith("thrust_macros::invariant!"):
            while not lines[i - 1].endswith(");"):
                i += 1
            continue
        if s.startswith("impl") and "thrust_models::Model for" in s:
            i = skip_block(lines, i)
            continue
        if s == "where" or s.startswith("<T as thrust_models::Model>::Ty"):
            continue
        if s.startswith("fn inv(") and s.endswith("{"):
            i = skip_block(lines, i)
            continue
        if s.startswith("fn inv("):
            continue  # the declaration in the trait
        count += 1
    return count


def generate():
    """The files, {relative path: text}, the manifest among them."""
    texts = {}
    manifest = {"sizes": SIZES, "programs": {}}

    def add(name, variant, k, members):
        pass_texts = [texts[(m, True)] for m in members]
        manifest["programs"][name] = {
            "variant": variant, "k": k, "files": members,
            "annotation_items": sum(annotation_items(t) for t in pass_texts),
            "code_lines": sum(code_lines(t) for t in pass_texts),
        }

    for k in SIZES:
        for variant in ("inferred", "written"):
            for expect_pass in (True, False):
                texts[(f"{variant}_k{k}", expect_pass)] = flat_program(variant, k, expect_pass)
            add(f"{variant}_k{k}", variant, k, [f"{variant}_k{k}"])
        for expect_pass in (True, False):
            texts[(f"nested_k{k}", expect_pass)] = nested_program(k, expect_pass)
        add(f"nested_k{k}", "nested", k, [f"nested_k{k}"])
    for j in range(max(SIZES)):
        for expect_pass in (True, False):
            texts[(f"perinstance_t{j}", expect_pass)] = perinstance_program(j, expect_pass)
    for k in SIZES:
        add(f"perinstance_k{k}", "perinstance", k, [f"perinstance_t{j}" for j in range(k)])

    files = {os.path.join("programs", "pass" if p else "fail", f"{name}.rs"): text
             for (name, p), text in texts.items()}
    files[os.path.join("programs", "manifest.json")] = json.dumps(manifest, indent=2) + "\n"
    return files


def status(path, text):
    """same, changed or new: the file at path against text."""
    if not os.path.exists(path):
        return "new"
    with open(path) as f:
        return "same" if f.read() == text else "changed"


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(text)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("-n", action="store_true", help="print each file against the tree, write none; exit 1 if any file is changed or new")
    args = ap.parse_args()

    files = generate()
    stale = False
    for rel, text in sorted(files.items()):
        path = os.path.join(HERE, rel)
        if args.n:
            st = status(path, text)
            stale = stale or st != "same"
            print(f"{st} {rel}")
        else:
            write(path, text)
    if not args.n:
        print(f"{len(files)} files")
    return 1 if stale else 0


if __name__ == "__main__":
    sys.exit(main())
