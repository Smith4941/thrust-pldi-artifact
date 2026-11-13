#!/usr/bin/env python3
"""Checks functions.csv against the runs of the stage roots.

usage: stage-report-check.py [--list] STAGES FUNCTIONS_CSV REPORT_JSONL...

STAGES are the stage digits whose roots the reports are of, such as 1234.

Each REPORT_JSONL is the report of one stage root run with THRUST_REPORT=functions
THRUST_REPORT_FORMAT=json (THRUST_REPORT_FILE). A function of functions.csv with a status of
verified, assumed, trusted, ignored, unchecked or compile-only (kept only so that the code
compiles, whose state must be `unchecked`), or marked `trusted` (an added helper), is looked up by file and line among the
`function` lines and gets, over all roots, the state `verified` (checked with verdict sat in some
root, where its clauses are in a part or it has a written contract), `assumed` (so checked, under
`assume!`s, the report's `assumes` field, D79), `trusted`, `ignored` or `unchecked` (no root
checks anything of it). A function without a contract is checked only under the preconditions its
callers establish: one that no analyzed function calls is reported `checked`, but none of its
clauses is in a part (the report's `parts` empty), since Thrust leaves them out of the query, so it
is `unchecked`. Only a root split by goal (THRUST_SPLIT_GOALS=1, which rq4-function-reports.sh adds
to a root that sets THRUST_SPLIT=1 alone, D125) tells this: without THRUST_SPLIT
the report gives no parts, so a function without a contract that only such roots check is
`unchecked`, and the class split (THRUST_SPLIT alone) gives such a body the parts of its class,
which the pruning leaves without its clauses (fork issue #173), so a root split so must not be
passed. A function has a written
contract when some root reports it `unselected`: Thrust trusts a function outside THRUST_VERIFY_ONLY
only when its contract leaves nothing to infer, and analyzes the others (`if !ty.has_pred_var()`
in src/analyze/crate_.rs:343-353 of fork main 1ea704fb). Every root selects one row, so a function
is outside the selection of the others. A root is named by its report's file name
(`<root>.jsonl`); the check stops unless the reports cover every root that the `root` rows of
parts.csv, next to FUNCTIONS_CSV, name for the given stages.
Prints the count per stage and state and each function whose state differs from its status, and
exits 1 if any; then lists each function that every report has as `unselected`, which no root
selects (its state is then `trusted` or `unchecked`, so it differs unless functions.csv says so).

With --list nothing is compared: it prints the rows of the selected stages, one per function with
its stage, file, line, function, status and the number of `assumes` its reports list (`-` for a
function no report has), so that a change of the selection can be checked without a run.
"""
import csv, json, os, sys
from collections import Counter

STATUS_STATE = {"verified": "verified", "assumed": "assumed", "trusted": "trusted", "trusted-callable": "trusted",
                "ignored": "ignored", "unchecked": "unchecked", "compile-only": "unchecked"}
LEEWAY = 8  # a lemma's span is its attribute line


def load(paths):
    """{file: [(root, start, end, body, verdict, assumes, parts)]}; assumes is the number of
    `assume!`s, parts the list of parts (None without THRUST_SPLIT)."""
    out = {}
    for path in paths:
        root = os.path.basename(path)
        for line in open(path):
            o = json.loads(line)
            if o.get("kind") != "function":
                continue
            s = o["span"]
            f = os.path.normpath(s["file"]).replace("tests/rustc_coroutine/", "")
            out.setdefault(f, []).append((root, s["line"], s["end_line"], o["body"], o.get("verdict"),
                                      len(o.get("assumes", [])), o.get("parts")))
    return out


def innermost(candidates):
    """Each root's innermost function among the candidates."""
    per_root = {}
    for c in candidates:
        best = per_root.get(c[0])
        if best is None or c[2] - c[1] < best[2] - best[1]:
            per_root[c[0]] = c
    return list(per_root.values())


def state_of(candidates):
    """The state over the roots."""
    per_root = innermost(candidates)
    contract = any(c[3] == "unselected" for c in per_root)
    checked = [c for c in per_root if (c[3], c[4]) == ("checked", "sat") and (c[6] or contract)]
    if checked:
        return "assumed" if any(c[5] for c in checked) else "verified"
    bodies = {c[3] for c in per_root}
    return "trusted" if "trusted" in bodies else "ignored" if "ignored" in bodies else "unchecked"


def unselected_everywhere(candidates):
    """Whether every root that reports the function has it outside its selection."""
    return all(c[3] == "unselected" for c in innermost(candidates))


def lookup(reported, file, line):
    f = os.path.normpath("tests/ui/pass/rustc_coroutine/" + file).replace("tests/rustc_coroutine/", "")
    inside = [c for c in reported.get(f, []) if c[1] <= line <= c[2]]
    return inside or [c for c in reported.get(f, []) if c[1] <= line <= c[1] + LEEWAY]


def expected(row):
    """The state a row's function must have in the reports; a `trusted` row (an added helper
    marked trusted too, D79) must be reported as trusted. None for a row with nothing to check."""
    if row["trusted"] == "true":
        return "trusted"
    return STATUS_STATE.get(row["status"])


def selected(stages, functions_csv):
    """The rows of functions_csv of the given stages that have a state to check."""
    return [r for r in csv.DictReader(open(functions_csv))
            if expected(r) and r["stage"] in stages]


def list_rows(stages, functions_csv, reported):
    """Prints the selected rows with the number of assumes the reports list for each."""
    for row in selected(stages, functions_csv):
        found = lookup(reported, row["file"], int(row["line"]))
        assumes = max((c[5] for c in found), default="-")
        print(row["stage"], row["file"], row["line"], row["function"], row["status"], f"assumes={assumes}")


def missing_roots(stages, functions_csv, paths):
    """The roots parts.csv names for the stages that no report is of."""
    parts = os.path.join(os.path.dirname(functions_csv), "parts.csv")
    roots = {r["file"] for r in csv.DictReader(open(parts)) if r["scope"] == "root" and r["part"] in stages}
    reported = {os.path.basename(p).removesuffix(".jsonl") for p in paths}
    return sorted(roots - reported)


def main():
    args = [a for a in sys.argv[1:] if a != "--list"]
    listing = len(args) < len(sys.argv) - 1
    if len(args) < (2 if listing else 3):
        sys.exit(__doc__)
    reported = load(args[2:])
    if listing:
        list_rows(args[0], args[1], reported)
        return
    missing = missing_roots(args[0], args[1], args[2:])
    if missing:
        sys.exit(f"no report of the roots {missing}")
    counts, bad, unselected = Counter(), [], []
    for row in selected(args[0], args[1]):
        want = expected(row)
        found = lookup(reported, row["file"], int(row["line"]))
        got = state_of(found) if found else "not reported"
        counts[(row["stage"], got)] += 1
        where = (row["stage"], os.path.basename(row["file"]), row["line"], row["function"])
        if got != want:
            bad.append(where + (want, got))
        if found and unselected_everywhere(found):
            unselected.append(where)
    for (stage, state), n in sorted(counts.items()):
        print(stage, state, n)
    for b in bad:
        print("differs:", *b)
    for u in unselected:
        print("selected by no root:", *u)
    sys.exit(1 if bad else 0)


main()
