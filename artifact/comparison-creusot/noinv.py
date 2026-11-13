#!/usr/bin/env python3
"""Write the inputs of the `inferred` rows of runs.csv: each is the case's pass test with its
`invariant!` and `partial_invariant!` statements removed.

usage: noinv.py [--check] [--thrust DIR] COMMIT

For each id with an `inferred` row, the pass row's test is read at COMMIT of the Thrust git tree
DIR (default the repository's submodule thrust/), every `invariant!(...);` and
`partial_invariant!(...);` statement is removed with the lines it spans, and the rest is written
to the inferred row's file, which must be `./noinv/` and the test's file name (the crate name the
queries use). The ids in OTHER_FORM are the exception: their inferred row is another pass test of
the case, in the directory of the pass row's, named there. A test with no such statement, or with `invariant!(` left on
another line, stops it. A module a test loads with `#[path = "P"] mod NAME;` (as the Creusot cases
load `auxiliary/common.rs`) is copied unchanged to `./noinv/P`, so that the path still resolves.
Copying is one level: a module loaded by `#[path]` inside the copied module is not copied.
`#[path]` must directly precede `mod`.
--check writes nothing and exits 1 when a file differs from what it would write.
"""

import argparse
import csv
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SUBMODULE = os.path.join(HERE, "..", "..", "..", "thrust")
START = re.compile(r"^\s*(thrust_macros::)?(partial_)?invariant!\(")
PATH_MODULE = re.compile(r'^#\[path = "([^"]+)"\]\s*\n\s*(?:pub\s+)?mod \w+;', re.M)
# id: the test of its inferred row, where that is not its pass row's. extend's printed row is the
# 2022 artifact's call of Vec::extend (concat_vec.rs, D121), its inferred row the for loop of the
# current Creusot's extend (D92).
OTHER_FORM = {"extend": "extend_for.rs"}


def statement_end(lines, start):
    """The index of the line that closes the `invariant!(` statement opened at lines[start]."""
    depth = 0
    for i in range(start, len(lines)):
        depth += lines[i].count("(") - lines[i].count(")")
        if depth == 0:
            if not lines[i].rstrip().endswith(");"):
                sys.exit(f"line {i + 1}: an invariant! statement does not end the line with `);`")
            return i
    sys.exit(f"line {start + 1}: an invariant! statement is not closed")


def without_invariants(test, text):
    """text without its invariant! statements; stops when it has none, or one it cannot remove."""
    lines = text.splitlines(keepends=True)
    kept, removed, i = [], 0, 0
    while i < len(lines):
        if START.match(lines[i]):
            i = statement_end(lines, i) + 1
            removed += 1
            continue
        kept.append(lines[i])
        i += 1
    rest = [n + 1 for n, line in enumerate(kept) if "invariant!(" in line]
    if not removed or rest:
        sys.exit(f"{test}: {removed} invariant! statements removed; kept lines with invariant!( {rest}")
    return "".join(kept)


def inputs(rows):
    """(pass test, inferred file) of each id with an inferred row."""
    tests = {r["id"]: r["file"] for r in rows if r["side"] == "pass"}
    pairs = []
    for r in rows:
        if r["side"] != "inferred":
            continue
        name = OTHER_FORM.get(r["id"], os.path.basename(tests[r["id"]]))
        if r["file"] != "./noinv/" + name:
            sys.exit(f"{r['id']}: the inferred file is {r['file']}, not ./noinv/{name}")
        test = os.path.join(os.path.dirname(tests[r["id"]]), name)
        pairs.append((test, os.path.join(HERE, r["file"])))
    return pairs


def show(args, path):
    """The file at path in the Thrust tree at the commit."""
    return subprocess.run(["git", "-C", args.thrust, "show", f"{args.commit}:{path}"],
                          capture_output=True, text=True, check=True).stdout


def outputs(args, rows):
    """{file written: its text} for the inferred rows and the modules their tests load by path."""
    texts = {}
    for test, out in inputs(rows):
        source = show(args, test)
        texts[out] = without_invariants(test, source)
        for module in PATH_MODULE.findall(source):
            module_file = os.path.normpath(os.path.join(os.path.dirname(test), module))
            texts[os.path.normpath(os.path.join(HERE, "noinv", module))] = show(args, module_file)
    return texts


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--check", action="store_true", help="compare, write nothing")
    ap.add_argument("--thrust", default=SUBMODULE, help="the Thrust git tree (default the submodule)")
    ap.add_argument("commit")
    args = ap.parse_args()
    rows = list(csv.DictReader(open(os.path.join(HERE, "runs.csv"))))
    differ = []
    for out, text in outputs(args, rows).items():
        current = open(out).read() if os.path.exists(out) else None
        if text == current:
            continue
        differ.append(os.path.relpath(out, HERE))
        if not args.check:
            os.makedirs(os.path.dirname(out), exist_ok=True)
            with open(out, "w") as f:
                f.write(text)
    verb = "differ" if args.check else "written"
    for path in differ:
        print(f"{verb}: {path}")
    if args.check and differ:
        sys.exit(1)


if __name__ == "__main__":
    main()
