#!/usr/bin/env python3
"""Write the tables and figures of the paper from the rows of a run.

usage: tables.py [-n] RUN [OUT]

RUN holds the rows run.py wrote: rq1.csv, rq3.csv, families.csv and rq4.csv. The counts come
from comparison-creusot/annotations.csv, rustc-coroutine-layout/rows.csv and
scalability-evaluation/code-lines.csv, which count-annotations writes in the tables step of
run-evaluation.sh. The files go to OUT/tables/ and OUT/figures/ (default paper/sections/ beside
the artifact's directory); with -n they are printed instead, each after a `==> path` line, and
nothing is written.

When every result cell of a table is `sat` (with or without a mark), the table leaves out all
its result columns, and a cell's mark moves to the time cell of its row (decisions.md D81);
`results` implements it for every table.

A cell reports the pass side only (decisions.md D90): its result and time, as
docs/evaluation/README.md (Verdicts) defines them; `input_runs` and `verdict_cell`
implement it, and the script stops where that definition says it must. The fail twins are a check
and no cell shows their results: a twin that answered `sat` stops the script, whether or not a
table reads its input (`stop_on_sat_twin`), and every twin row that is not `unsat` is listed on
stderr (`unrefuted_twins`), for the run's record. The cells of the
families figure take one run per size. The time of a cell is in seconds.

A time is the wall time of the Thrust command (decisions.md D123): run.py's `wall_s`, from
before thrust-rustc starts to after it exits, so Thrust's analysis and constraint generation and
every solver call with the wrapper's container start and teardown; for a split root (RQ4) the
root's parts as they were solved, four at a time (D126). Nothing is subtracted, and a perinstance
cell carries a Thrust command per program. Rows without `wall_s` (run.py before D122) stop the
script, and so does a `sat` row with an empty one (`wall_time`); run.py's `time_s`, the solver
calls alone, is recorded and not printed.
The rows' `load1` and `other_containers` are not read (decisions.md D71). A row answered from a
verdict cache (run.py --reuse-verdicts, or `cache_hits` above 0) stops the script
(`check_solved`, decisions.md D124): no time it prints is a cached answer.

The Creusot table prints, per case, the LoC of Creusot's code, which Thrust's side verifies in the
same form, so one count serves both (decisions.md D109; D121: the 2022 artifact's code, counted in
the per-case files of comparison-creusot/creusot-port/), Creusot's spec lines and time
(creusot.csv `wall`: the median wall seconds from the translation of the case's file to the proof
search's result, as creusot-time.py measures it, D123; `$\\times$` for `unproved`, the search
left goals open), then Thrust's spec lines, time and result. A case Thrust's side reports
without a `wall` stops the script; the shared trait `common`, with no proof of its own, prints
`--`.
creusot.csv's `auto`, replay `time` and `search` (the search alone, before D123) are recorded and
not printed.

A Creusot-table row whose Thrust side is weaker than Creusot's (the `weaker` column of
comparison-creusot/cases.csv) is marked with a dagger, which the caption explains; the comment
above the row names the way.

Table tab:coroutine has one row per row of rustc-coroutine-layout/rows.csv (decisions.md D82): a
rustc function of at least 40 lines of code, or the rest of a source file, or of the files that
parts.csv groups into one module (`rustc_index`); the rules are in the header of
count-annotations' src/coroutine.rs. A row's result and time are the
cell of its runs: the roots rows.csv names in `files` (the root whose THRUST_VERIFY_ONLY selects
the row, by parts.csv, when the row has a verified or assumed function), whose pass rows of
rq4.csv give the verdict and whose times are summed. A row whose roots are all roots of an
earlier row shows no cell of its own but `same run as` that row, so a run is printed once; a row
with roots of an earlier row and others stops tables.py, since its time would count that run
again. A row without roots shows what its functions are instead (`body not verified` for
unchecked ones, `trusted`, `not verified` for ignored ones; tables.py stops when they are of more
than one kind). Its columns (D83): verification target, LoC (which
includes the lines of functions kept only so that the code compiles, rows.csv `compile_only`), spec
lines (rows.csv `spec_lines`, counted as RQ1's), extracted (the functions a rewrite extracted from
rustc's functions, other than lemmas and law proofs, rows.csv `functions_extracted`), trusted (the rustc functions whose contract is
taken without a body a listed root checks: trusted and unchecked; an ignored function, which no
verified code may run, is assumed nothing and is not counted, decisions.md D111), assumed (the
`assume!` calls of the row, rows.csv `assume_stmts`), invariants inferred
(the loops whose invariant is inferred / the loops of the verified functions), contracts inferred
(the non-generic functions whose contract is inferred / the non-generic functions whose bodies a run
checks, verified or assumed, rows.csv `functions_checked` less `generic_functions_checked`), and the
same for the generic functions, whose contract Thrust infers per instance (`generic_contracts_inferred`
/ `generic_functions_checked`), result, time. An inferred count is the row's when its runs verified it in
some run (its result is `sat`, with or without a share) and 0 otherwise, since a run that does not
verify infers nothing it can be shown to hold; a row without loops or checked functions shows `--`.
The other counts of rows.csv are not printed, and the table has no note
rows (the text says what they said). The printed text uses no term of the method (root, fail
twin, part), and a fail twin that answered `sat` still stops tables.py.

The rows are joined to the lists of this checkout by id, so tables.py stops on a row that ran
another file than its list names now (another path, or for a file of this repository other
content), or that ran from a tree with uncommitted changes, or that is `interrupted`, or lacks a row its list
names for a run number (a row marked `printed` or the fail twin of one, as run.py runs by
default, or any row once the run holds another, as a run with run.py --diagnostic does; with or without a `file` column, an input with twin rows
missing a pass or twin row for a run number the other side has is refused), or whose fail twin answered `sat`: such rows are regenerated with this
repository at the commit they record (`research`). An input a table or figure reads must be marked
`printed` in its list, which run.py runs by default with the twins; tables.py stops on one that is not. Rows written before run.py recorded the file
carry none of these columns and are not checked.
Every check runs before the first file is written.
"""

import collections
import csv
import hashlib
import itertools
import math
import os
import re
import statistics
import sys

ART = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
# The directory that holds artifact/ and paper/: the paper's directory in this repository, the
# package's root.
PAPER_ROOT = os.path.join(ART, "..")
SIZES = [1, 2, 4, 8, 16, 32]
# The benchmarks of scalability-evaluation/generate_chains.py that fig:scalability-iterators draws
# and tab:scalability-iterators prints, in the order of the panels (two per row), each with the name
# the paper gives it (k-types-<adapter>: count called at k distinct types; k-chain-<adapter>: at a
# chain of k adapters).
# Every benchmark grows with one size k, as its programs' ids `<benchmark>_..._k<k>` do: the number
# of distinct types for the k-types benchmarks, the depth of the chain for the k-chain ones.
SCALABILITY_FIGURE_BENCHMARKS = (("tagged_take", "\\texttt{k-types-take}"),
                                 ("tagged_skip", "\\texttt{k-types-skip}"),
                                 ("take_depth", "\\texttt{k-chain-take}"),
                                 ("skip_depth", "\\texttt{k-chain-skip}"))
# The lowest time the logarithmic axes of the RQ3 figures show, in seconds.
LOG_YMIN = 0.1
SETTINGS = {"generic": ("generic", "solid", "0072B2"), "per_instance": ("per instance", "dashed", "D55E00")}
Marker = collections.namedtuple("Marker", "verified unverified rotate")
# The marker of each setting in the RQ3 figures, so that shape tells the setting and the kind of
# cell also in black and white: a square for generic and the same square turned by 45 degrees per
# instance; a filled square for a verified cell, an unfilled square at the time limit for a cell
# whose runs timed out or ran out of memory. A size that was not run (decisions.md D113) has no
# marker. Where the two settings' points coincide the squares form an eight-pointed star, and both
# stay visible.
MARKS = {"generic": Marker("square*", "square", 0), "per_instance": Marker("square*", "square", 45)}
MARK_SIZE = "2pt"
# The kinds of a cell in the RQ3 figures, in legend order.
KINDS = ("verified", "unverified")
# The verdicts of an unverified cell, which share its marker: its runs timed out, or ran out of
# memory (run.py's OUT_OF_MEMORY, verdict `oom`); and the legend's words for it.
UNVERIFIED_VERDICTS = ("timeout", "oom")
UNVERIFIED = "timeout/out of memory"
# The verdict of a cell that was not run: run.py's --skip-sizes-past-cap row.
NOT_RUN = "skipped"
# The top of the RQ3 figures' axes, as a multiple of the time limit, so that the markers of the
# unverified cells at the limit lie inside the axes, on the dotted line that marks it.
LIMIT_HEADROOM = 2
# The legend of an RQ3 figure on one row, its entries apart; the iterators figure centres it under its
# two columns of panels (x 1.03 of the first panel's width), the families figure under its middle one.
LEGEND_ROW = "legend columns=4, legend cell align=left, /tikz/every even column/.append style={column sep=8pt}"
# The order the settings are drawn in: per instance first, under generic.
DRAWN = ("per_instance", "generic")
LISTS = {"rq1.csv": "comparison-creusot", "rq3.csv": "scalability-evaluation",
         "families.csv": "scalability-input-families", "rq4.csv": "rustc-coroutine-layout"}
FAMILIES = [("base", "loop, inferred"), ("base_parity", "loop, parity inv."),
            ("twovar", "two values, inferred"), ("twovarw", "two values, written"),
            ("nestloop", "nested loops, inferred"), ("vecfill", "\\lstinline{Vec}, inferred"),
            ("long8", "8 calls, no loop"), ("long12", "12 calls, no loop"), ("satint8", "8 saturating adds")]


def mark(setting, kind="verified"):
    """The pgfplots options of a setting's marker for a kind of cell (KINDS)."""
    m = MARKS[setting]
    turn = f", mark options={{rotate={m.rotate}}}" if m.rotate else ""
    return f"mark={getattr(m, kind)}, mark size={MARK_SIZE}{turn}"


def legend_lines(names, kinds):
    """The legend of an RQ3 figure: for each setting, under the name `names` gives it, its line with
    its verified marker and, when the figure draws an unverified cell (`kinds`), its unverified
    marker, so that every marker the figure shows is named."""
    lines = []
    for s, (_, dash, _) in SETTINGS.items():
        lines += [f"\\addlegendimage{{color=c{s}, {mark(s)}, {dash}}}", f"\\addlegendentry{{{names[s]}, verified}}"]
        if "unverified" in kinds:
            lines += [f"\\addlegendimage{{color=c{s}, {unverified(s)}}}", f"\\addlegendentry{{{names[s]}, {UNVERIFIED}}}"]
    return lines


def limit_line(cap):
    """A dotted line across the axes at the time limit."""
    return f"\\draw[densely dotted, gray] ({{rel axis cs:0,0}} |- {{axis cs:1,{cap}}}) -- ({{rel axis cs:1,0}} |- {{axis cs:1,{cap}}});"


def unverified(setting):
    """The pgfplots options of a setting's mark for a cell verified in no run: its unverified
    marker alone, thick."""
    return f"only marks, {mark(setting, 'unverified')}, thick"


def plot_line(setting, kind, coords):
    """The \\addplot line of consecutive cells of one kind: verified cells joined by the setting's
    line, the others as markers alone."""
    options = f"{mark(setting)}, {SETTINGS[setting][1]}" if kind == "verified" else unverified(setting)
    return f"\\addplot[forget plot, color=c{setting}, {options}] coordinates {{{coords}}};"


def runs_label(setting, k, cell):
    """A node beside the marker of a cell verified in some runs but not all, giving n/N."""
    return (f"\\node[font=\\tiny, color=c{setting}, anchor=south west, inner sep=1pt] "
            f"at (axis cs:{k},{round(cell.time, 1)}) {{{cell.ok}/{len(cell.runs)}}};")

def read(*path):
    return list(csv.DictReader(open(os.path.join(*path))))


def digest(source, file):
    """The sha256 prefix run.py records of a file of this repository, relative to source's list."""
    return hashlib.sha256(open(os.path.join(ART, LISTS[source], file), "rb").read()).hexdigest()[:16]


class Rows(list):
    """The rows of a run, with the (id, setting) pairs their list declares with a fail twin and the
    (id, setting, side) inputs it lists without marking them printed."""

    def __init__(self, rows, twins, unprinted=frozenset()):
        super().__init__(rows)
        self.twins = twins
        self.unprinted = unprinted


def checked_rows(run, source):
    """The rows of RUN/source, after checking each against today's list of its question."""
    rows = read(run, source)
    check_wall_times(source, rows)
    check_solved(source, rows)
    listed = read(ART, LISTS[source], "runs.csv")
    files = {(r["id"], r["setting"], r["side"]): r["file"] for r in listed}
    printed = {(r["id"], r["setting"], r["side"]) for r in listed if r["printed"] == "yes"}
    for r in rows:
        name = " ".join(filter(None, [r["id"], r["setting"], r["side"]]))
        listed = files.get((r["id"], r["setting"], r["side"]))
        if "file" in r and r["file"] != listed:
            sys.exit(f"{source}: {name} ran {r['file']}, the list names {listed}; "
                     f"regenerate at this repository's commit {r['research']}")
        if "file" in r and r["file"].startswith("./") and r["file_sha256"] != digest(source, r["file"]):
            sys.exit(f"{source}: {name} ran another {r['file']} than this checkout holds; "
                     f"regenerate at this repository's commit {r['research']}")
        if "dirty" in (r.get("thrust_tree"), r.get("research_tree")):
            sys.exit(f"{source}: {name} ran with uncommitted changes")
    if rows and "file" in rows[0]:
        check_complete(source, rows, files, printed)
    return Rows(rows, {(i, st) for i, st, sd in files if sd == "fail"}, set(files) - printed)


def check_wall_times(source, rows):
    """Stop on rows without run.py's `wall_s` column, the time every cell prints (D123), which rows
    written before D122 lack."""
    if rows and "wall_s" not in rows[0]:
        sys.exit(f"{source}: the rows have no wall_s, the time the tables print (decisions.md D123); "
                 f"they predate run.py's wall_s (D122): take a new run")


def check_solved(source, rows):
    """Stop on a row answered from a verdict cache (D124): run with --reuse-verdicts, or with a
    part taken from its own run's cache (`cache_hits`). Rows before D124 have neither column."""
    for r in rows:
        name = " ".join(filter(None, [r["id"], r["setting"], r["side"]]))
        if r.get("verdict_cache") == "reused":
            sys.exit(f"{source}: {name} run {r['run']} answered from a shared verdict cache; its time is not measured")
        if int(r.get("cache_hits") or 0) > 0:
            sys.exit(f"{source}: {name} run {r['run']} took {r['cache_hits']} parts from the cache; "
                     f"run it again without --keep-verdicts")


def wall_time(row):
    """The seconds of a row's Thrust command (`wall_s`, D123); stops on a row without them."""
    if not row.get("wall_s"):
        sys.exit(f"{' '.join(filter(None, [row['id'], row['setting'], row['side']]))} run {row['run']}: "
                 f"{row['verdict']} without wall_s, the time the tables print (D123)")
    return float(row["wall_s"])


def check_complete(source, rows, files, printed):
    """Stop on an interrupted row, and on a run number that lacks a row its list names: a printed
    row or the fail twin of one (what run.py runs by default), or any row once the run holds
    another (a run with run.py --diagnostic)."""
    if any(r["verdict"] == "interrupted" for r in rows):
        sys.exit(f"{source}: a run was interrupted")
    if any(r.get("check_solution", "off") != "off" for r in rows):
        sys.exit(f"{source}: rows taken with the solution check on carry its time; use a run with it off")
    have = {(r["id"], r["setting"], r["side"], r["run"]) for r in rows}
    measured = printed | {(i, st, sd) for i, st, sd in files if sd == "fail" and any(p[:2] == (i, st) for p in printed)}
    diagnostic = any(k[:3] not in measured for k in have)
    for n in sorted({r["run"] for r in rows}, key=int):
        missing = [k for k in (files if diagnostic else measured) if k + (n,) not in have]
        if missing:
            sys.exit(f"{source}: run {n} has no row for {missing[0]}; the run did not finish")


def comment(path, rows):
    return (f"% Generated by artifact/scripts/tables.py from {os.path.relpath(path, PAPER_ROOT)} "
            f"(Thrust {rows[0]['thrust']}, fptprove {rows[0]['fptprove']}); do not edit.")


DAGGER = r"\textsuperscript{\dag}"
CROSS = r"$\times$"


def is_sat(cell):
    """Whether a result cell is `sat`, with or without a mark (a superscript)."""
    return cell == "sat" or cell.startswith("sat\\textsuperscript")


SAME_RUN = "same run as"


def joined(cells, i):
    """Whether cell i is joined with a neighbour into one `\\multicolumn`: None itself (joined with
    the cell before), or followed by None (the start of a joined cell)."""
    return cells[i] is None or (i + 1 < len(cells) and cells[i + 1] is None)


def refers(cells, i):
    """Whether cell i is no result of its own: joined with the cell before, or a joined reference to
    an earlier row's run (`same run as`). A joined cell of a row without a run (`body not
    verified`) is a result that is not sat."""
    return cells[i] is None or (joined(cells, i) and cells[i].startswith(SAME_RUN))


def results(body, columns):
    """The lines of a table body; when every result cell of the table is sat, its
    result columns are left out (D81). body is a list of (comments, cells); a cell None joins the
    cell before it into one `\\multicolumn`, and a row may be shorter than the others. columns are
    (result, time) pairs of indices; a left-out result's mark moves to the time cell of its row. A
    joined cell over a result column (`same run as`) is no result: the sat test skips it, and leaving
    the column out narrows it by one; a joined cell of a row without a run is a result other than
    sat. Stops on a table without rows."""
    if not body:
        sys.exit("tables: a table without rows")
    body = [(comments, list(cells)) for comments, cells in body]
    cells = [c[result] for result, _ in columns for _, c in body
             if len(c) > result and c[result] and not refers(c, result)]
    if not cells or not all(is_sat(c) for c in cells):
        columns = []
    for result, time in sorted(columns, reverse=True):
        for _, c in body:
            if len(c) <= result:
                continue
            if joined(c, result):
                del c[result + 1 if c[result] is not None else result]
                continue
            mark = c[result][3:] if c[result] else ""
            if mark:
                c[time] += mark
            del c[result]
    return [line for comments, cells in body for line in comments + [render(cells)]]


def render(cells):
    """A row: the cells joined by `&`, a cell followed by k None cells as `\\multicolumn{k+1}`."""
    out = []
    for c in cells:
        if c is not None:
            out.append([c, 1])
        else:
            out[-1][1] += 1
    return " & ".join(c if n == 1 else f"\\multicolumn{{{n}}}{{c}}{{{c}}}" for c, n in out) + r" \\"


def tex(s):
    return s.replace("_", r"\_")


def seconds(t):
    return "--" if t is None else f"{t:.1f}" if t < 100 else f"{t:.0f}"


def share(word, k, n):
    return word if k == n else f"{word} {k}/{n}"


# The words a cell prints for a verdict other than its own name: `oom`, the solver out of its
# container's memory (run.py's OUT_OF_MEMORY), is printed OOM.
VERDICT_WORDS = {"oom": "OOM"}


Input = collections.namedtuple("Input", "passes fails runs ok")
# ok: the number of runs the pass side is sat; reason: of a cell with none, its most frequent verdict
Cell = collections.namedtuple("Cell", "result time ok runs reason", defaults=(None,))


def input_runs(rows, id, setting="", side="pass"):
    """One input: its pass rows and fail-twin rows by run number, the runs, and the runs whose pass
    side answered sat (D90: the twin's verdict does not enter the cell). Stops on an input its list
    does not mark printed, on a twin that answered sat, and on a run number that one side has and
    the other lacks; a run without twin rows (run.py without --diagnostic) checks no twin."""
    if (id, setting, side) in rows.unprinted:
        sys.exit(f"{id} {setting} {side}: a table prints it, and its list does not mark it printed")
    sides = {s: {r["run"]: r for r in rows if (r["id"], r["setting"], r["side"]) == (id, setting, s)}
             for s in (side, "fail")}
    passes, fails = sides[side], sides["fail"]
    twin = (id, setting) in rows.twins
    if not twin and fails:
        sys.exit(f"{id} {setting}: has fail rows but its list declares no twin")
    if twin and fails and set(passes) != set(fails):
        sys.exit(f"{id} {setting}: pass runs {sorted(passes, key=int)} and twin runs "
                 f"{sorted(fails, key=int)} differ; the run did not finish")
    for n, r in fails.items():
        if r["verdict"] == "sat":
            sys.exit(f"{id} {setting}: the fail twin answered sat in run {n}; a soundness event to investigate")
    runs = sorted(passes, key=int)
    ok = [n for n in runs if passes[n]["verdict"] == "sat"]
    return Input(passes, fails, runs, ok)


def failure_outcome(inputs, n):
    """Why run n of inputs is not sat: the first pass verdict other than sat."""
    return next(i.passes[n]["verdict"] for i in inputs if i.passes[n]["verdict"] != "sat")


def failure_cell(inputs, runs):
    """The cell of inputs none of whose runs is sat: the most frequent pass verdict, over the runs
    (`timeout 1/2`)."""
    reasons = collections.Counter(failure_outcome(inputs, n) for n in runs)
    reason, k = max(reasons.items(), key=lambda kv: (kv[1], kv[0]))
    return Cell(share(VERDICT_WORDS.get(reason, reason), k, len(runs)), None, 0, runs, reason)


def verdict_cell(rows, ids, setting="", side="pass"):
    """The Cell (result, time, verified runs, runs) of the inputs ids, as Tables tab:creusot and
    tab:coroutine print them; None when no input has a pass row. Stops when the inputs ran
    different run numbers."""
    inputs = [input_runs(rows, i, setting, side) for i in ids]
    if not any(i.passes for i in inputs):
        return None
    if any(i.runs != inputs[0].runs for i in inputs):
        sys.exit(f"{', '.join(ids)}: the inputs have different run numbers; the run did not finish")
    runs = inputs[0].runs
    ok = [n for n in runs if all(n in i.ok for i in inputs)]
    return cell_of(inputs, ok, runs) if ok else failure_cell(inputs, runs)


def cell_of(inputs, ok, runs):
    """The Cell of inputs whose pass sides are all sat in the runs ok: the sum of the inputs' median
    wall times (D123) over those runs."""
    time = math.fsum(statistics.median(wall_time(i.passes[n]) for n in ok) for i in inputs)
    return Cell(share("sat", len(ok), len(runs)), time, len(ok), runs)


Row = collections.namedtuple("Row", "comment cells")


def choose_variant(cells):
    """Thrust's variant of a case as decisions.md D1 says: Creusot's form unless it was measured
    and verified in no run while the weaker form verified in some; None when there is neither.
    Verified is the pass side's `sat`, as for the cells (D90); a chosen form whose twin is not
    refuted is on the `unrefuted_twins` list like any other."""
    def verified(v):
        return bool(cells.get(v)) and cells[v].time is not None
    if verified("creusot"):
        return "creusot"
    if cells.get("creusot") and verified("weaker"):
        return "weaker"
    if "creusot" in cells:
        return "creusot"
    return "weaker" if "weaker" in cells else None


def creusot_time(case):
    """Creusot's time of a creusot.csv row (`wall`, D123): seconds, or a cross for `unproved`; stops
    on a row without one."""
    if not case["wall"]:
        sys.exit(f"creusot.csv: {case['case']} has no `wall`, Creusot's time from translation to the "
                 f"search's result (D123): measure it with creusot-time.py")
    return {"unproved": CROSS}.get(case["wall"], case["wall"])


def creusot_rows(rows):
    """The Rows of tab:creusot per block: the Creusot side as recorded, and Thrust's variant. When
    Thrust's reported form is weaker (cases.csv `weaker`), the row's comment names the way and its
    result carries the dagger. A row without a Thrust test (`common`, the iterator trait shared by
    the files) prints Thrust's spec lines when annotations.csv counts it, and no time or result."""
    cases = read(ART, "comparison-creusot", "cases.csv")
    counts = {r["id"]: r for r in read(ART, "comparison-creusot", "annotations.csv")}
    blocks = {"adapter": [], "client": []}
    for c in read(ART, "comparison-creusot", "creusot.csv"):
        variants = {v: [r["id"] for r in cases if r["case"] == c["case"] and r["variant"] == v]
                    for v in ("creusot", "weaker")}
        cells = {v: verdict_cell(rows, ids) for v, ids in variants.items() if ids}
        chosen = choose_variant(cells)
        creusot = [tex(c["name"]), c["loc"], c["spec"]]
        if chosen is None:
            shared = counts.get(c["case"], {}).get("spec_lines", "--")
            blocks[c["block"]].append(Row(None, creusot + ["--", shared, "--", ""]))
            continue
        creusot.append(creusot_time(c))
        cell = cells[chosen] or Cell("--", None, 0, [])
        first = counts[variants[chosen][0]]
        way = next((r["weaker"] for r in cases if r["id"] in variants[chosen] and r["weaker"]), None)
        if chosen == "weaker" and not way:
            sys.exit(f"tables: {c['case']} is reported in its weaker variant, whose cases.csv rows have no `weaker` text")
        comment = f"% {c['case']}: Thrust's {chosen} variant, {' '.join(variants[chosen])}"
        blocks[c["block"]].append(Row(comment + (f"; weaker: {way}" if way else ""),
                                      creusot + [first["spec_lines"], seconds(cell.time),
                                                 cell.result + (DAGGER if way else "")]))
    return blocks


def creusot_table(rows):
    """Adapters on the left, clients on the right of the same lines; comments above their row."""
    blocks = creusot_rows(rows)
    body = []
    for left, right in itertools.zip_longest(blocks["adapter"], blocks["client"]):
        left = left or Row(None, [""] * 7)
        halves = [left] + ([right] if right else [])
        body.append(([h.comment for h in halves if h.comment], [c for h in halves for c in h.cells]))
    return results(body, [(6, 5), (13, 12)])


def inferred_row(rows, case, files):
    """The (comments, cells) of one case of tab:creusot-inferred: its label, and the result and time
    with its invariants written and with every `invariant!` removed."""
    cells = []
    for side in ("pass", "inferred"):
        cell = verdict_cell(rows, [case["id"]], side=side) or Cell("--", None, 0, [])
        cells += [cell.result, seconds(cell.time)]
    comment = f"% {case['id']}: written {files[(case['id'], 'pass')]}; inferred {files[(case['id'], 'inferred')]}"
    return [comment], [tex(case["s5_label"]), *cells]


def inferred_table(rows):
    """tab:creusot-inferred: the labelled cases (cases.csv `s5_label`), the cases of tab:creusot first in
    its order (creusot.csv) and then the others (block `extra`, std's loops, decisions.md D120), two to
    a line; each with its written invariants and with every `invariant!` removed."""
    files = {(r["id"], r["side"]): r["file"] for r in read(ART, "comparison-creusot", "runs.csv")}
    order = [c["case"] for c in read(ART, "comparison-creusot", "creusot.csv")]
    cases = [c for c in read(ART, "comparison-creusot", "cases.csv") if c["s5_label"]]
    ordered = (sorted((c for c in cases if c["case"] in order), key=lambda c: order.index(c["case"]))
               + [c for c in cases if c["case"] not in order])
    halves = [inferred_row(rows, c, files) for c in ordered]
    body = []
    for l, r in itertools.zip_longest(halves[0::2], halves[1::2]):
        line = [l] + ([r] if r else [])
        body.append(([c for comments, _ in line for c in comments], [x for _, cells in line for x in cells]))
    return results(body, [(1, 2), (3, 4), (6, 7), (8, 9)])


def scalability_cell(rows, variant, k, setting):
    """The Cell of one variant at size k (None without rows). perinstance is the k programs
    t0 .. t(k-1); its time the sum of each program's median over the verified runs."""
    ids = [f"perinstance_t{j}" for j in range(k)] if variant == "perinstance" else [f"{variant}_k{k}"]
    return verdict_cell(rows, ids, setting)


def scalability_text(cell):
    """The table's cell: the time with the verified runs as a superscript when fewer than all,
    else the result."""
    if not cell:
        return "--"
    if cell.reason == NOT_RUN:  # a size --skip-sizes-past-cap did not run is no answer (D113)
        return "--"
    if cell.time is None:
        return cell.result.replace("timeout", "t/o")
    mark = f"$^{{\\mathrm{{{cell.ok}/{len(cell.runs)}}}}}$" if cell.ok < len(cell.runs) else ""
    return f"{cell.time:.1f}{mark}"


def scalability_time(rows, variant, k, setting):
    """The time the figure plots; stops when the cell has none."""
    cell = scalability_cell(rows, variant, k, setting)
    if not cell or cell.time is None:
        sys.exit(f"rq3.csv: {variant} k={k} {setting} has no verified time ({cell.result if cell else 'no rows'}); "
                 f"the figure plots only verified times")
    return round(cell.time, 1)


def scalability_table(rows):
    lines = {r["id"]: r["code_lines"] for r in read(ART, "scalability-evaluation", "code-lines.csv")}
    body = [" & ".join([str(k), lines[f"inferred_k{k}"]]
                       + [scalability_text(scalability_cell(rows, v, k, s))
                          for v in ("inferred", "written", "nested", "perinstance") for s in SETTINGS]) + r" \\"
            for k in SIZES]
    return body


def iterator_variant(benchmark, inferred):
    """The id prefix of a benchmark's program in runs.csv, as generate_chains.py's variant() names it:
    the asserting form, whose callers assert the count, so that `count`'s contract is used
    (decisions.md D98)."""
    return benchmark + ("_inferred" if inferred else "") + "_assert"


def iterator_rows(rows, benchmark, lines):
    """One benchmark's rows of tab:scalability-iterators: per k, k, the lines of code and the cells of
    `count` with its loop invariant written and inferred, in each setting."""
    return [[str(k), lines[f"{iterator_variant(benchmark, False)}_k{k}"]]
            + [scalability_text(scalability_cell(rows, iterator_variant(benchmark, inferred), k, s))
               for inferred in (False, True) for s in SETTINGS]
            for k in SIZES]


def iterators_table(rows):
    """tab:scalability-iterators: the benchmarks of SCALABILITY_FIGURE_BENCHMARKS, two side by side as
    the figure's panels. A `\\midrule` separates the pairs; each pair has its title row unstriped and
    its rows striped from the first (the paper's `\\stripedbody`)."""
    lines = {r["id"]: r["code_lines"] for r in read(ART, "scalability-evaluation", "code-lines.csv")}
    pairs = zip(SCALABILITY_FIGURE_BENCHMARKS[0::2], SCALABILITY_FIGURE_BENCHMARKS[1::2])
    body = []
    for i, ((left, left_title), (right, right_title)) in enumerate(pairs):
        if i > 0:
            body.append("\\midrule")
        body.append(f"\\hiderowcolors\\multicolumn{{6}}{{l}}{{{left_title}}} & "
                    f"\\multicolumn{{6}}{{l}}{{{right_title}}} \\\\")
        body.append("\\stripedbody")
        body += [" & ".join(l + r) + " \\\\"
                 for l, r in zip(iterator_rows(rows, left, lines), iterator_rows(rows, right, lines))]
    return body


def scalability_figure(rows):
    """fig:scalability, the supplementary material's figure of the `repeat` series (decisions.md D88):
    the inferred and written cells of tab:scalability against k."""
    lines = [r"\definecolor{cinferred}{HTML}{0072B2}", r"\definecolor{cwritten}{HTML}{D55E00}",
             r"\begin{tikzpicture}",
             r"\begin{groupplot}[group style={group size=2 by 1, horizontal sep=1cm},",
             r"  width=0.52\linewidth, height=4.7cm, ymode=log, ymin=" + str(LOG_YMIN) + r", xlabel={$k$},",
             r"  legend pos=north west, legend style={font=\scriptsize, draw=none, fill=none},",
             r"  legend cell align=left,",
             r"  xtick={1,4,8,16,32}, tick label style={font=\scriptsize}, label style={font=\small},",
             r"  title style={font=\small, yshift=-1ex}]"]
    for variant, axis in (("inferred", ", ylabel={wall time (s)}"), ("written", "")):
        lines.append(f"\\nextgroupplot[title={{{variant}}}{axis}]")
        for setting in DRAWN:
            points = " ".join(f"({k},{scalability_time(rows, variant, k, setting)})" for k in SIZES)
            lines.append(f"\\addplot[forget plot, color=c{variant}, {mark(setting)}, {SETTINGS[setting][1]}] coordinates {{{points}}};")
        for setting, (name, dash, _) in SETTINGS.items():
            lines += [f"\\addlegendimage{{color=c{variant}, {mark(setting)}, {dash}}}", f"\\addlegendentry{{{name}}}"]
    return lines + [r"\end{groupplot}", r"\end{tikzpicture}"]


def time_limit(rows):
    """The one time limit of the rows; stops when they have several."""
    limits = {r["timeout_s"] for r in rows}
    if len(limits) != 1:
        sys.exit(f"rq3.csv: the rows have the time limits {sorted(limits)}, the figure draws one")
    return int(float(limits.pop()))


def cell_kind(cell):
    """The kind (KINDS) of a cell of an RQ3 figure, None for a size that was not run (D113); stops on
    a cell verified in no run whose verdict is neither a timeout nor out of memory, which the figure
    has no marker for."""
    if cell.time is not None:
        return "verified"
    if cell.reason == NOT_RUN:
        return None
    if cell.reason not in UNVERIFIED_VERDICTS:
        sys.exit(f"rq3.csv: a cell answered {cell.result}; the figure has no marker for it")
    return "unverified"


def iterators_figure(rows):
    """fig:scalability-iterators, the RQ3 figure of the paper's body (decisions.md D88): one panel per
    benchmark of SCALABILITY_FIGURE_BENCHMARKS, its size k on its x axis, the cell of `count` with its loop invariant inferred (the variant
    `<benchmark>_inferred_assert`, run with the loop history, D88, D98) against the parameter, generic solid with
    squares and per instance dashed with squares turned by 45 degrees (MARKS). A cell verified in
    some runs but not all has its n/N beside its marker; a cell without a verified time is the
    setting's marker for its verdict (timeout or out of memory) at the time limit, which no line
    joins; a size without rows or that was not run (D113) has no mark."""
    cap = time_limit(rows)
    series = {(benchmark, setting): [(k, cell, cell_kind(cell)) for k in SIZES
                                     if (cell := scalability_cell(rows, iterator_variant(benchmark, True), k, setting))]
              for benchmark, _ in SCALABILITY_FIGURE_BENCHMARKS for setting in DRAWN}
    series = {key: [(k, cell, kind) for k, cell, kind in cells if kind] for key, cells in series.items()}
    drawn = {kind for cells in series.values() for _, _, kind in cells}
    lines = [f"\\definecolor{{c{s}}}{{HTML}}{{{c}}}" for s, (_, _, c) in SETTINGS.items()]
    lines += [r"\begin{tikzpicture}",
              r"\begin{groupplot}[group style={group size=2 by 2, horizontal sep=0.6cm, vertical sep=1.4cm, yticklabels at=edge left},",
              f"  width=0.53\\linewidth, height=3.8cm, xmin=1, xmax=32, ymode=log, ymin={LOG_YMIN}, ymax={cap * LIMIT_HEADROOM}, clip=false,",
              r"  xtick={1,2,4,8,16,32}, tick label style={font=\scriptsize}, label style={font=\small},",
              r"  title style={font=\footnotesize, yshift=-1ex}]"]
    for n, (benchmark, title) in enumerate(SCALABILITY_FIGURE_BENCHMARKS):
        axis = ", xlabel={$k$}" + (", ylabel={wall time (s)}" if n % 2 == 0 else "")  # the left panel of each row
        axis += f", legend style={{at={{(1.03,-2.2)}}, anchor=north, {LEGEND_ROW}, font=\\scriptsize, draw=none}}" if n == 0 else ""
        lines += [f"\\nextgroupplot[title={{{title}}}{axis}]", limit_line(cap)]
        for setting in DRAWN:
            cells = series[benchmark, setting]
            for kind, run in itertools.groupby(cells, lambda c: c[2]):
                coords = " ".join(f"({k},{round(cell.time, 1) if kind == 'verified' else cap})" for k, cell, _ in run)
                lines.append(plot_line(setting, kind, coords))
            lines += [runs_label(setting, k, cell) for k, cell, kind in cells
                      if kind == "verified" and cell.ok < len(cell.runs)]
        if n == 0:
            lines += legend_lines({s: name for s, (name, _, _) in SETTINGS.items()}, drawn)
    return lines + [r"\end{groupplot}", r"\end{tikzpicture}"]


def family_points(rows, family, setting):
    """(k, row) of the family's pass sides in one setting, one run per k; stops on a verdict
    other than sat, timeout and oom, which the figure has no mark for."""
    points = {}
    for r in rows:
        m = re.fullmatch(family + r"_k(\d+)", r["id"])
        if not m or r["setting"] != setting or r["side"] != "pass":
            continue
        if (r["id"], setting, "pass") in rows.unprinted:
            sys.exit(f"families.csv: the figure prints {r['id']} {setting}, and its list does not mark it printed")
        if int(m.group(1)) in points:
            sys.exit(f"families.csv: more than one run of {r['id']} {setting}")
        if r["verdict"] not in ("sat",) + UNVERIFIED_VERDICTS:
            sys.exit(f"families.csv: {r['id']} {setting} answered {r['verdict']}")
        points[int(m.group(1))] = r
    return sorted(points.items())


def point_kind(row):
    """The kind (KINDS) of a point of fig:scalability-families: `verified` for a `sat` row, else
    `unverified` (family_points allows only timeout and oom besides sat)."""
    return "verified" if row["verdict"] == "sat" else "unverified"


def families_figure(rows):
    """Pass-side wall time against k per family, once and per instance; a timeout or an out-of-memory
    run is the setting's marker for it at the time limit and breaks the line. One run per size, read from the rows directly: outside the
    cells of docs/evaluation/README.md (Verdicts)."""
    cap = int(rows[0]["timeout_s"])
    lines = [f"\\definecolor{{c{s}}}{{HTML}}{{{c}}}" for s, (_, _, c) in SETTINGS.items()]
    lines += [r"\begin{tikzpicture}",
              r"\begin{groupplot}[group style={group size=3 by 3, horizontal sep=30pt, vertical sep=34pt,",
              r"    xlabels at=edge bottom, ylabels at=edge left},",
              r"  width=4.4cm, height=3.4cm, xmode=log, log basis x=2, xtick={1,4,8}, xticklabels={1,4,8},",
              f"  ymode=log, ymin={LOG_YMIN}, ymax={cap * LIMIT_HEADROOM}, clip=false,",
              r"  xlabel={$k$}, ylabel={wall time (s)},",
              r"  title style={font=\footnotesize}, tick label style={font=\scriptsize},",
              r"  label style={font=\footnotesize}, legend style={font=\scriptsize}]"]
    points = {(family, setting): family_points(rows, family, setting) for family, _ in FAMILIES for setting in DRAWN}
    drawn = {point_kind(r) for ps in points.values() for _, r in ps}
    for family, title in FAMILIES:
        legend = f", legend style={{at={{(0.5,-0.42)}}, anchor=north, {LEGEND_ROW}}}" if family == "long12" else ""
        lines += [f"\\nextgroupplot[title={{{title}}}{legend}]", limit_line(cap)]
        for setting in DRAWN:
            for kind, run in itertools.groupby(points[family, setting], lambda p: point_kind(p[1])):
                coords = " ".join(f"({k},{wall_time(r) if kind == 'verified' else cap})" for k, r in run)
                lines.append(plot_line(setting, kind, coords))
        if legend:
            lines += legend_lines({"generic": "once", "per_instance": "per inst."}, drawn)
    return lines + [r"\end{groupplot}", r"\end{tikzpicture}"]


CoroutineRow = collections.namedtuple("CoroutineRow", "label ids counts")


def coroutine_label(r):
    """A function row by its name, a module row by its `module` (`rustc_index`), the rest of a file
    by its module path (`rustc_abi`)."""
    if r["function"]:
        return f"\\texttt{{{tex(r['function'])}}}"
    if r["module"]:
        return f"\\texttt{{{tex(r['module'])}}}"
    path = re.sub(r"^.*/compiler/", "", r["file"]).removesuffix(".rs").replace("/src/", "/").split("/")
    return f"\\texttt{{{tex('::'.join(p for p in path if p not in ('lib', 'mod')))}}}"


def coroutine_rows():
    """The rows of rows.csv with their labels and the run ids of their roots."""
    return [CoroutineRow(coroutine_label(r), [f[:-3] for f in r["files"].split()], r)
            for r in read(ART, "rustc-coroutine-layout", "rows.csv")]


def shared_row(row, earlier):
    """The first earlier row whose roots include all of row's, else None; stops on an earlier row
    that has some of row's roots but not all, since row's time would count that run again."""
    for e in earlier:
        if set(row.ids) <= set(e.ids):
            return e
        if set(row.ids) & set(e.ids):
            sys.exit(f"rows.csv: {row.label} has the root(s) {sorted(set(row.ids) & set(e.ids))} of the earlier "
                     f"row {e.label} and others; its time would count that run again")
    return None


def unrun(row):
    """What the functions of a row without roots are, in the table's words: `body not verified`
    (unchecked), `trusted` or `not verified` (ignored); stops when they are of none or of more
    than one kind."""
    c = row.counts
    kinds = {"body not verified": int(c["unchecked_functions"]), "not verified": int(c["ignored_functions"]),
             "trusted": int(c["trusted_functions"]) - int(c["ignored_functions"])}
    present = [k for k, n in kinds.items() if n]
    if len(present) != 1:
        sys.exit(f"rows.csv: {row.label} has no root and its functions are {present or 'none'}")
    return present[0]


def coroutine_run(rows, row, earlier):
    """The result and time cells of a row (None: the cell is joined with the one before it), and
    whether the row's runs verified it in some run; a row that shares an earlier row's runs takes
    that row's (earlier maps each earlier row's label to it and that flag)."""
    if not row.ids:
        return [unrun(row), None], False
    shared = shared_row(row, [e for e, _ in earlier.values()])
    if shared:
        return [f"{SAME_RUN} {shared.label}", None], earlier[shared.label][1]
    cell = verdict_cell(rows, row.ids) or Cell("--", None, 0, [])
    return [cell.result, seconds(cell.time)], cell.ok > 0


def inferred_cell(inferred, total, verified):
    """`k/n`: of the n loops or functions of a row, the k whose invariant or contract the row's
    runs inferred, counted only when the runs verified the row in some run (0 otherwise); `--`
    when the row has none."""
    if int(total) == 0:
        return "--"
    return f"{inferred if verified else 0}/{total}"


def coroutine_table(rows):
    """tab:coroutine (D82, D83): verification target, LoC, spec lines, functions extracted (rows.csv
    `functions_extracted`, the functions a rewrite extracted from rustc's functions, other than
    lemmas and law proofs), trusted (trusted and unchecked functions; ignored ones are not, D111), assumed (rows.csv
    `assume_stmts`, the `assume!` calls), invariants inferred (`invariants_inferred` / `loops`),
    contracts inferred of the non-generic functions checked and of the generic ones, whose
    contract Thrust infers per instance (`contracts_inferred` / `functions_checked`, each less
    its `generic_` count), result, time."""
    body, earlier = [], {}
    for row in coroutine_rows():
        c = row.counts
        (result, time), verified = coroutine_run(rows, row, earlier)
        earlier[row.label] = (row, verified)
        trusted = int(c["trusted_functions"]) - int(c["ignored_functions"]) + int(c["unchecked_functions"])
        invariants = inferred_cell(c["invariants_inferred"], c["loops"], verified)
        contracts = inferred_cell(int(c["contracts_inferred"]) - int(c["generic_contracts_inferred"]),
                                  int(c["functions_checked"]) - int(c["generic_functions_checked"]), verified)
        generic_contracts = inferred_cell(c["generic_contracts_inferred"], c["generic_functions_checked"], verified)
        body.append(([], [row.label, c["loc"], c["spec_lines"], c["functions_extracted"], str(trusted),
                          c["assume_stmts"], invariants, contracts, generic_contracts, result, time]))
    return results(body, [(9, 10)])


def stop_on_sat_twin(source, rows):
    """Stop on a fail-twin row of source that answered sat (D27), also of an input no table reads."""
    for r in rows:
        if r["side"] == "fail" and r["verdict"] == "sat":
            sys.exit(f"{source}: {' '.join(filter(None, [r['id'], r['setting']]))}: the fail twin answered sat "
                     f"in run {r['run']}; a soundness event to investigate")


def unrefuted_twins(source, rows):
    """One line per fail-twin row of source that is not `unsat`, with the verdicts of the sides it
    guards (pass, and inferred where RQ2 runs one) in the same run: the check the record reports,
    since no cell shows it (D90)."""
    guarded = collections.defaultdict(list)
    for r in rows:
        if r["side"] != "fail":
            guarded[r["id"], r["setting"], r["run"]].append(f"{r['side']} side {r['verdict']}")
    return [f"{source}: {' '.join(filter(None, [r['id'], r['setting']]))} run {r['run']}: fail twin "
            f"{r['verdict']}, {', '.join(guarded.get((r['id'], r['setting'], r['run']), ['pass side not run']))}"
            for r in rows if r["side"] == "fail" and r["verdict"] != "unsat"]


def main():
    args = sys.argv[1:]
    dry = args[:1] == ["-n"]
    args = args[1:] if dry else args
    if len(args) not in (1, 2):
        sys.exit(__doc__.split("\n\n")[1])
    run, out = args[0], args[1] if len(args) == 2 else os.path.join(PAPER_ROOT, "paper", "sections")
    outputs = [("rq1.csv", "tables/creusot.tex", creusot_table), ("rq1.csv", "tables/creusot-inferred.tex", inferred_table),
               ("rq3.csv", "tables/scalability.tex", scalability_table), ("rq3.csv", "figures/scalability.tex", scalability_figure),
               ("rq3.csv", "tables/scalability-iterators.tex", iterators_table),
               ("rq3.csv", "figures/scalability-iterators.tex", iterators_figure),
               ("families.csv", "figures/scalability-families.tex", families_figure),
               ("rq4.csv", "tables/coroutine.tex", coroutine_table)]
    checked = {source: checked_rows(run, source) for source, _, _ in outputs}
    for source, rows in checked.items():
        stop_on_sat_twin(source, rows)
        for line in unrefuted_twins(source, rows):
            print(line, file=sys.stderr)
    texts = [(target, "\n".join([comment(os.path.join(run, source), checked[source])] + write(checked[source])) + "\n")
             for source, target, write in outputs]
    if dry:
        print("".join(f"==> {os.path.join(out, target)}\n{text}" for target, text in texts), end="")
        return
    for target, text in texts:
        os.makedirs(os.path.dirname(os.path.join(out, target)), exist_ok=True)
        with open(os.path.join(out, target), "w") as f:
            f.write(text)


if __name__ == "__main__":
    main()
