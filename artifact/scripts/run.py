#!/usr/bin/env python3
"""Verify the inputs of a list with Thrust and write one CSV row per run.

usage: run.py [-n] [--dirty] [--check-solution] [--thrust DIR] --thrust-commit T --fptprove-commit F
              (--image IMAGE | --solver CMD) [--runs N] [--jobs N] [--timeout SECS] [--only ID,...]
              [--diagnostic] [--parts-at-once N] [--skip-sizes-past-cap]
              [--keep-verdicts | --reuse-verdicts DIR] LIST OUT

LIST is a CSV with the columns id, setting, side, expect, file, env and printed. run.py runs the
rows whose `printed` is `yes`, the inputs whose result a table or figure of the paper prints
(tables.py stops on one it prints that is not marked), and their fail twins, the `fail` rows of
the same id and setting, which check that the property can fail (decisions.md D27, D107);
--diagnostic also runs the others: the inputs kept for diagnosis and the twins of unprinted inputs. `file` is a path in the
Thrust tree DIR (default thrust), or relative to LIST when it starts with `./`; `env` holds
`K=V` pairs for the row. Each input runs as the ui harness runs it: DIR's thrust-rustc with the
flags and variables of the file's `//@` header and the crate type ui_test infers from its contents
(a library unless the file has a `main`), then the run's settings, then the row's env. The
solver is the tree's tests/thrust-pcsat-wrapper, which starts a container of IMAGE with 4 CPUs and
8 GB per query, or, without an image, the command CMD (made absolute from the caller's
directory). The solver timeout is SECS (default 300; 0 keeps the file's own). A row's env may
change the solver configuration (COAR_CONFIG), the container's limits (THRUST_PCSAT_CPUS,
THRUST_PCSAT_MEMORY), the timeout and Thrust's own settings other than a split, not the other
variables run.py sets.

A split root: an RQ4 root whose header sets exactly THRUST_SPLIT=1, with or without
THRUST_SPLIT_GOALS=1 (D78): with it each part is a goal's cone, and without it each goal of a
function marked #[thrust::split_goals] is a part, its cone, and the other goals are solved in
their classes of clauses. Any other THRUST_SPLIT* setting in a header, a split set by a row's env
or in another list than the RQ4 list (rustc-coroutine-layout/runs.csv), --check-solution, and a
THRUST_CACHE_DIR (other than run.py's own) or THRUST_REPORT* setting in a split row stop the run. Thrust falls back to the
classes of clauses when a clause is not definite, and it splits no goal when no function is
marked; the note "N parts solved on their own" then lacks ", one per goal" and ", each goal of a
`#[thrust::split_goals]` function apart", and the run is an `error`. For a split root SECS
is the limit of each part and replaces the header's (the sweep's 300 s; a header's limit serves
the ui test).

--parts-at-once N (decisions.md D122) solves N parts of a split root at a time, through Thrust's
THRUST_SPLIT=N in place of the header's 1; the parts are the same and time_s is still their sum.
Each part has its container, so a run starts up to --jobs times N containers.

--skip-sizes-past-cap (decisions.md D113, for the RQ3 list) runs the sizes of each series, the
rows whose ids differ only in a final `_k<size>` with the same setting and side, in increasing
order, each size's runs before the next size. When every run of a size reached the cap (timeout
or oom), the larger sizes of the series do not run: their rows are written with the verdict
`skipped`, no time and no query, as no answer. Rows of other ids run as without it. A series
runs in one worker, so --jobs bounds the series at once. The side is part of a series, so under
--diagnostic a fail twin's sizes stop on their own. -n lists the series in the order they run.

--keep-verdicts (decisions.md D124) gives each run Thrust's verdict cache in a directory of its own,
OUT.d/<run>/verdicts/, which does not exist before the run starts, so that no run is answered from
another's verdicts and the times stay those of solving; run-evaluation.sh's `verdicts` step merges
the directories into the cache the artifact ships. Within one run the cache is read too: a split
root whose parts include two with the same query takes the second from the first, and Thrust's
note counts it, which the row records in `cache_hits` and tables.py refuses. --reuse-verdicts DIR
runs every input with DIR as the cache, read and written, as a reviewer re-checks the verdicts with
the shipped cache; the times of such rows are not those of solving, and tables.py refuses them.
Either sets THRUST_CACHE_SALT to the fptprove commit and the row's COAR_CONFIG, what decides a
verdict besides the query and the solver command (the image is checked to be built from F). The
row's `verdict_cache` says which: `off`, `kept` (an empty directory of its own) or `reused`.

--check-solution (with --image): see the artifact README (../README.md).

Before the first run, run.py stops unless the rows can say what ran: DIR is a checkout at T
without uncommitted changes; target/release/thrust-rustc (the release build, `cargo build
--release`, whose wall time the tables print, decisions.md D123) is newer than Cargo.toml,
Cargo.lock and every source file its cargo dep-info names; target/release/deps holds one
thrust_macros library;
this script's directory and LIST's have no uncommitted changes; IMAGE was built from F (its
`fptprove` label, or its tag coar:<commit> when it has no label); no row's env sets a variable
run.py fixes; and neither OUT nor OUT.d exists. --dirty allows uncommitted changes and records
the tree as dirty. A tree that is not a git checkout (the replication package's) is recorded as `nogit` with
the commit given.

The verdict is Thrust's: sat when it exits 0, otherwise the kind of its `verification error`
(timeout also when the solver printed a line `timeout`). A run without one is `error` (Thrust
failed: a compile error, a panic, a solver failure), `oom` (the solver ran out of its container's
memory, which the wrapper reports as status 137), `killed` (by a signal) or `interrupted`
(stopped by a SIGINT, SIGTERM or SIGHUP to run.py, which ends the runs in flight and starts no more).
`time_s` is the solver call's, from Thrust's trace. For a split root it is the sum of the parts' calls:
a part cut at the timeout logs
no call and counts the timeout, and `parts` is the number from Thrust's note "N parts solved on
their own, one per goal" (or ", each goal of a `#[thrust::split_goals]` function apart"). A split run without the note or with the class split, or with more
calls than parts, is `error` with no time. `wall_s` is the wall time of the thrust-rustc command,
from time.monotonic before it starts to after it exits: Thrust's own work and every solver call,
and for a split root its parts as --parts-at-once runs them. tables.py prints `wall_s` (decisions.md
D123) and not `time_s`.
A row also records the input (file, its sha256,
env), the solver command's name (no path, so that rows name no directory), its configuration and
limits, the 1-minute load when the run starts and the containers of other runs when it ends, and
the identity of the run: the Thrust commit and tree state, the sha256 of thrust-rustc, the image
id and fptprove commit, and this repository's commit and state. Rows are written as runs end.
Thrust's log and the emitted query of each run are kept in its own directory under OUT.d/. -n
prints the identity and the runs with their settings instead of starting them, then makes the
same checks (its exit status says whether a run would start).
"""

import argparse
import csv
import datetime
import glob
import hashlib
import json
import os
import re
import shlex
import signal
import subprocess
import sys
import threading
import time
from concurrent.futures import ThreadPoolExecutor

FIELDS = ["id", "setting", "side", "run", "expect", "verdict", "time_s", "wall_s", "timeout_s", "parts", "parts_at_once",
          "verdict_cache", "cache_hits", "invariants",
          "clauses", "unknowns", "dep_unknowns", "check_solution", "check_passed", "unchecked", "check_failed",
          "query_sha256", "load1", "other_containers",
          "thrust", "fptprove", "image", "date", "file", "file_sha256", "env", "solver", "config",
          "cpus", "memory", "image_id", "thrust_tree", "thrust_rustc", "research", "research_tree"]
QUERY_SIZE = {"clauses": "(assert", "unknowns": "(declare-fun", "dep_unknowns": "(declare-dep-exists-fun"}
UNIT = {"ns": 1e-9, "µs": 1e-6, "us": 1e-6, "ms": 1e-3, "s": 1.0}
WRAPPER = "tests/thrust-pcsat-wrapper"
RQ4_DIR = "rustc-coroutine-layout"  # the directory of the only list whose roots may split (D78)
CHECK_CONFIGS = {}  # COAR_CONFIG -> the image's configuration, for --check-solution
SAT_CHECK = r"\(sat-check \(passed (\d+)\) \(unchecked (\d+)\) \(failed (\d+)\)"
# Variables run.py sets that a row's env must not change: they name the solver, the image, where
# the run writes, and how its log reads.
FIXED = ("THRUST_SOLVER", "COAR_IMAGE", "THRUST_OUTPUT_DIR", "THRUST_PCSAT_LABEL", "NO_COLOR",
         "RUST_LOG", "LD_LIBRARY_PATH", "THRUST_CACHE_DIR", "THRUST_CACHE_SALT")
HERE = os.path.dirname(os.path.abspath(__file__))
LOCK = threading.Lock()
STOP = threading.Event()
RUNNING = set()


def pairs(text):
    return dict(kv.split("=", 1) for kv in text.split())


def sha256(path, digits=16):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()[:digits]


def header(path):
    """The ui_test directives of a file: compile flags, rustc env and edition."""
    flags, env, edition = [], {}, "2021"
    for line in open(path):
        if not line.startswith("//@"):
            continue
        key, _, value = (s.strip() for s in line[3:].partition(":"))
        if key == "compile-flags":
            flags += shlex.split(value)
        elif key == "rustc-env":
            env.update(pairs(value))
        elif key == "edition":
            edition = value
    return flags, env, edition


# ui_test 0.30's `CrateType::from_file_contents`, in its order: a proc macro, a test, a binary (its
# `fn main()` is a regex in which `()` is an empty group, so `fn main` anywhere counts), else a library.
CRATE_TYPES = ((re.compile(r"#\[proc_macro(_derive|_attribute)?[\](]"), ["--crate-type=proc-macro"]),
               (re.compile(r"#\[test\]"), ["--test"]),
               (re.compile(r"fn main|#\[start\]"), []))


def crate_type_flags(path):
    """The crate-type flag the ui harness passes for a file, so that a file without `main` is
    compiled as a library there and here alike."""
    text = open(path, encoding="utf-8", errors="replace").read()
    for pattern, flags in CRATE_TYPES:
        if pattern.search(text):
            return flags
    return ["--crate-type=lib"]


def environment(args, test_env, row, rundir):
    """The caller's variables without THRUST_* and COAR_*, then the file's header, the run's
    settings and the row's env."""
    env = {k: v for k, v in os.environ.items() if not k.startswith(("THRUST_", "COAR_"))}
    env.update(test_env)
    env.update(THRUST_SOLVER=args.solver, COAR_IMAGE=args.image_ref, THRUST_OUTPUT_DIR=rundir,
               COAR_CONFIG="./config/solver/pcsat_thrust.json", THRUST_PCSAT_CPUS="4",
               THRUST_PCSAT_MEMORY="8g", THRUST_PCSAT_LABEL=args.label, NO_COLOR="1",
               RUST_LOG="thrust::chc::solver=info", LD_LIBRARY_PATH=args.rustc_lib)
    if args.timeout:
        env["THRUST_SOLVER_TIMEOUT_SECS"] = str(args.timeout)
    if splits(test_env):
        env["THRUST_SPLIT"] = str(args.parts_at_once)  # D122; check_splits made the header's 1
    env.update(pairs(row["env"]))
    env.pop("THRUST_CACHE_DIR", None)  # a header's; check_splits refuses one in a split root
    cache = verdict_cache_dir(args, rundir)
    if cache:
        env.update(THRUST_CACHE_DIR=cache, THRUST_CACHE_SALT=f"fptprove={args.fptprove_commit} config={env['COAR_CONFIG']}")
    return env


def verdict_cache_dir(args, rundir):
    """The cache of a run (D124): its own new directory with --keep-verdicts, the given one with
    --reuse-verdicts, else none."""
    if args.keep_verdicts:
        return os.path.join(rundir, "verdicts")
    return args.reuse_verdicts or ""


def settings(args, env):
    """The solver settings a row records; the container's limits only when there is one."""
    return dict(timeout_s=int(env.get("THRUST_SOLVER_TIMEOUT_SECS", "30")),  # Thrust's default
                parts_at_once=env["THRUST_SPLIT"] if splits(env) else "",
                verdict_cache="kept" if args.keep_verdicts else "reused" if args.reuse_verdicts else "off",
                check_solution="on" if args.check_solution else "off",
                config=env["COAR_CONFIG"], cpus=env["THRUST_PCSAT_CPUS"] if args.image else "",
                memory=env["THRUST_PCSAT_MEMORY"] if args.image else "")


def git(path, *argv):
    """git's output in path, or None when path is not in a git checkout."""
    p = subprocess.run(["git", "-C", path, *argv], capture_output=True, text=True)
    return p.stdout.strip() if p.returncode == 0 else None


def checkout(path, scope):
    """(HEAD, state) of the checkout holding path; state is clean, dirty (uncommitted changes,
    untracked files included, under the paths of scope, or a path of scope outside the checkout)
    or nogit."""
    head = git(path, "rev-parse", "HEAD")
    if head is None:
        return "", "nogit"
    return head, "clean" if git(path, "status", "--porcelain", "--", *scope) == "" else "dirty"


def same_commit(a, b):
    return min(len(a), len(b)) >= 7 and (a.startswith(b) or b.startswith(a))


def image_identity(image):
    """(id, fptprove commit) of a local image: the commit is its `fptprove` label, or the commit
    of its tag coar:<commit>[-...] when it has no label; both empty without the image."""
    try:
        p = subprocess.run(["docker", "image", "inspect", "--format", "{{.Id}} {{json .Config.Labels}}", image],
                           capture_output=True, text=True)
    except FileNotFoundError:
        return "", ""
    if p.returncode != 0:
        return "", ""
    image_id, labels = p.stdout.split(" ", 1)
    tag = re.fullmatch(r"coar:([0-9a-f]{7,40})(-.*)?", image)
    commit = (json.loads(labels) or {}).get("fptprove") or (tag.group(1) if tag else "")
    return image_id.removeprefix("sha256:")[:12], commit


def stale(target, depinfo):
    """The files of target's cargo dep-info, and Cargo.toml and Cargo.lock of the tree, that are
    missing or newer than target."""
    tree = os.path.join(os.path.dirname(depinfo), "..", "..")
    deps = re.split(r"(?<!\\)\s+", open(depinfo).readline().partition(": ")[2].strip())
    deps = [d.replace("\\ ", " ") for d in deps if d]
    deps += [os.path.join(tree, "Cargo.toml"), os.path.join(tree, "Cargo.lock")]
    built = os.path.getmtime(target)
    return [d for d in deps if not os.path.exists(d) or os.path.getmtime(d) > built]


def build_dir(args):
    """The directory of the Thrust build that runs: the release build (D123), whose thrust-rustc
    loads the thrust_macros library of its deps/."""
    return os.path.join(args.thrust, "target", "release")


def identity(args):
    """The columns every row of the run shares, read from the trees and the image."""
    thrust, thrust_tree = checkout(args.thrust, ["."])
    research, research_tree = checkout(HERE, [HERE, os.path.dirname(os.path.abspath(args.list))])
    image_id, fptprove = image_identity(args.image) if args.image else ("", args.fptprove_commit)
    binary = os.path.join(build_dir(args), "thrust-rustc")
    return dict(thrust=thrust[:8] or args.thrust_commit, thrust_tree=thrust_tree,
                thrust_rustc=sha256(binary) if os.path.exists(binary) else "",
                research=research[:7], research_tree=research_tree, image=args.image,
                image_id=image_id, fptprove=fptprove, solver=os.path.basename(args.solver))


def splits(env):
    """Whether Thrust splits a run of this environment: THRUST_SPLIT is a positive number
    (THRUST_SPLIT_GOALS alone chooses what the parts are, not whether there are any)."""
    return env.get("THRUST_SPLIT", "").strip().isdigit() and int(env["THRUST_SPLIT"]) > 0


def sets_split(env):
    """Whether an environment sets THRUST_SPLIT or THRUST_SPLIT_GOALS at all."""
    return any(k.startswith("THRUST_SPLIT") for k in env)


def exact_split(env):
    """Whether the THRUST_SPLIT* settings of an environment are exactly a form of a D78 root."""
    found = {k: v.strip() for k, v in env.items() if k.startswith("THRUST_SPLIT")}
    return found in ({"THRUST_SPLIT": "1", "THRUST_SPLIT_GOALS": "1"}, {"THRUST_SPLIT": "1"})


def split_conflicts(env):
    """The variables of a split run's environment that make its calls not the parts: a cache
    answers a part without a call, and a report solves parts again."""
    return sorted(k for k in env if k == "THRUST_CACHE_DIR" or k.startswith("THRUST_REPORT"))


def check_splits(args, todo):
    """A split is a root's header setting in the RQ4 list (D78): stop on one from a row's env, on
    one in another list, and on one under --check-solution, which reads one solver output per run."""
    from_env = [path for row, path, _ in todo if sets_split(pairs(row["env"]))]
    if from_env:
        sys.exit(f"a split is set in a root's header, not in a row's env (D78): {from_env}")
    odd = [path for _, path, _ in todo if sets_split(header(path)[1]) and not exact_split(header(path)[1])]
    if odd:
        sys.exit(f"a root's header sets THRUST_SPLIT=1, with or without THRUST_SPLIT_GOALS=1, or neither (D78): {odd}")
    split = [path for _, path, _ in todo if splits(header(path)[1])]
    for row, path, _ in todo:
        env = {**header(path)[1], **pairs(row["env"])}
        if splits(env) and split_conflicts(env):
            sys.exit(f"{path}: a split run sets {split_conflicts(env)}, so its calls are not its parts")
    if split and os.path.basename(os.path.dirname(os.path.abspath(args.list))) != RQ4_DIR:
        sys.exit(f"a split is allowed in the RQ4 list ({RQ4_DIR}/runs.csv) only (D78): {split}")
    if split and args.check_solution:
        sys.exit(f"--check-solution reads one solver output per run, not one per part: {split}")


def check(args, ident, todo):
    """Stop on what would make the rows wrong or name something other than what ran."""
    binary = os.path.join(build_dir(args), "thrust-rustc")
    macros = glob.glob(os.path.join(build_dir(args), "deps", "libthrust_macros*.so"))
    missing = [path for _, path, _ in todo if not os.path.exists(path)]
    if os.path.exists(args.out) or os.path.exists(os.path.abspath(args.out) + ".d"):
        sys.exit(f"{args.out} or {args.out}.d exists: give a new OUT")
    fixed = sorted({k for r, _, _ in todo for k in pairs(r["env"]) if k in FIXED})
    if missing:
        sys.exit(f"missing inputs: {missing}")
    if fixed:
        sys.exit(f"a row's env sets what run.py sets: {fixed}")
    check_splits(args, todo)
    if ident["thrust_tree"] != "nogit" and not same_commit(ident["thrust"], args.thrust_commit):
        sys.exit(f"the tree {args.thrust} is at {ident['thrust']}, not at {args.thrust_commit}")
    if not args.dirty and "dirty" in (ident["thrust_tree"], ident["research_tree"]):
        sys.exit(f"uncommitted changes (Thrust tree {ident['thrust_tree']}, scripts and list "
                 f"{ident['research_tree']}); commit them or pass --dirty")
    if not os.path.exists(binary + ".d"):
        sys.exit(f"{binary} is not built: run cargo build --release")
    newer = stale(binary, binary + ".d")
    if newer:
        sys.exit(f"{binary} is older than {newer[:3]}: run cargo build --release")
    if len(macros) != 1:
        sys.exit(f"{len(macros)} thrust_macros libraries in the tree, not 1: {macros}")
    if args.image and not ident["image_id"]:
        sys.exit(f"no image {args.image}")
    if not same_commit(ident["fptprove"], args.fptprove_commit):
        sys.exit(f"the image {args.image} names fptprove {ident['fptprove'] or 'nothing'}, "
                 f"not {args.fptprove_commit}")
    if args.check_solution:
        read_check_configs(args, todo)


# The wrapper's message when the solver's container exits with status 137, its process killed by
# SIGKILL: in a run that nobody stops, that is the kernel's OOM killer at the container's memory
# limit (a stop by Thrust or run.py ends the wrapper with 143 instead).
OUT_OF_MEMORY = "the solver exited with status 137"


def verdict(code, log, stopped):
    """Thrust's verdict, or how the run ended without one: `oom` when the solver ran out of the
    container's memory (OUT_OF_MEMORY)."""
    if stopped and code != 0:
        return "interrupted"
    if code == 0:
        return "sat"
    if code < 0:
        return "killed"
    m = re.search(r"verification error: (\w+)", log)
    kind = m.group(1).lower() if m else "error"
    if kind == "noverdict" and "timeout" in solver_lines(log[m.end():]):
        return "timeout"
    if kind in ("unsat", "unknown", "timeout"):
        return kind
    return "oom" if OUT_OF_MEMORY in log else "error"


def solver_lines(rest):
    """The solver's output lines in Thrust's `NoVerdict { stdout: "..." }`."""
    m = re.match(r' \{ stdout: "(.*)" \}', rest.split("\n", 1)[0])
    return [line.strip() for line in m.group(1).split("\\n")] if m else []


def solver_calls(log):
    """The seconds of each solver call in Thrust's trace, in order."""
    found = re.findall(r"exited.*?elapsed=([0-9.]+)(ns|µs|us|ms|s)", log)
    return [float(n) * UNIT[unit] for n, unit in found]


def part_count(log):
    """The number of parts of a split run, from Thrust's note "N parts solved on their own"; None
    without the note."""
    m = re.search(r"thrust: (\d+) parts? solved on (?:their|its) own", log)
    return int(m.group(1)) if m else None


def by_goals(log):
    """Whether Thrust's note says the parts are the goals' cones (all goals, or the goals of the
    functions marked #[thrust::split_goals]), not the classes of clauses it falls back to when a
    clause is not definite or no function is marked."""
    return ("one per goal with the clauses it depends on" in log
            or "each goal of a `#[thrust::split_goals]` function apart" in log)


def cache_hits(log):
    """The parts Thrust's note says it took from THRUST_CACHE_DIR; 0 without such a note."""
    m = re.search(r"solved on (?:their|its) own, (?:(\d+) of them from earlier runs|from an earlier run)", log)
    return 0 if not m else int(m.group(1)) if m.group(1) else 1


def split_run(log, result, limit):
    """(verdict, seconds, parts) of a split run: the sum of the parts' calls, a part cut at its
    limit (THRUST_SOLVER_TIMEOUT_SECS applies to each part) logging no call and counting the
    limit, a part taken from the cache counting nothing. Without Thrust's part count, with the
    class split, or with more calls than parts, the run did not solve the parts it names: `error`
    and no time, unless it was stopped."""
    parts, calls = part_count(log), solver_calls(log)
    if parts is None or not by_goals(log) or len(calls) + cache_hits(log) > parts:
        return (result if result in ("interrupted", "killed", "oom") else "error"), None, parts
    return result, sum(calls) + (parts - cache_hits(log) - len(calls)) * limit, parts


def check_counts(path):
    """The counts of the solver's `(sat-check ...)` line in its output file; empty when the file or
    the line is missing."""
    if not os.path.exists(path):
        return {}
    m = re.search(SAT_CHECK, open(path).read())
    if not m:
        return {}
    return dict(zip(("check_passed", "unchecked", "check_failed"), m.groups()))


def read_check_configs(args, todo):
    """The image's solver configuration of each row's COAR_CONFIG, read once; stops when one is
    unreadable."""
    for row, path, _ in todo:
        config = environment(args, header(path)[1], row, "OUT.d")["COAR_CONFIG"]
        if config in CHECK_CONFIGS:
            continue
        p = subprocess.run(["docker", "run", "--rm", "-w", "/root/coar", args.image_ref, "cat", config],
                           capture_output=True, text=True)
        if p.returncode != 0:
            sys.exit(f"cannot read {config} from {args.image}: {p.stderr.strip()}")
        CHECK_CONFIGS[config] = json.loads(p.stdout)


def write_check_config(env, rundir):
    """OUT.d/<run>/solver-config.json: the image's configuration with `check_solution` on."""
    name, options = CHECK_CONFIGS[env["COAR_CONFIG"]]
    with open(os.path.join(rundir, "solver-config.json"), "w") as f:
        json.dump([name, dict(options, check_solution=True)], f, indent=2)


def query_facts(path):
    """Size and hash of the emitted query; empty when the run emitted none."""
    if not os.path.exists(path):
        return {}
    text = open(path, "rb").read()
    heads = [line.split(b" ", 1)[0].decode() for line in text.splitlines() if line.startswith(b"(")]
    facts = {k: heads.count(head) for k, head in QUERY_SIZE.items()}
    return dict(facts, query_sha256=hashlib.sha256(text).hexdigest()[:16])


def other_containers(label):
    """The running containers without this run's label; empty without Docker."""
    try:
        ps = [subprocess.run(["docker", "ps", "-q", *f], capture_output=True, text=True).stdout.split()
              for f in ([], ["--filter", f"label={label}"])]
    except FileNotFoundError:
        return ""
    return len(ps[0]) - len(ps[1])


def terminate(p):
    """SIGTERM to the process group of a run: thrust-rustc and the solver wrapper it started."""
    try:
        os.killpg(p.pid, signal.SIGTERM)
    except ProcessLookupError:
        pass


def solve(cmd, cwd, env):
    """(exit status, output) of thrust-rustc, in a process group of its own so that run.py alone
    decides when it is stopped."""
    p = subprocess.Popen(cmd, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                         text=True, start_new_session=True)
    with LOCK:
        RUNNING.add(p)
    if STOP.is_set():
        terminate(p)
    log = p.communicate()[0]
    with LOCK:
        RUNNING.discard(p)
    return p.returncode, log


def stop_runs():
    STOP.set()
    with LOCK:
        for p in RUNNING:
            terminate(p)


def run_name(row, n):
    return ".".join(filter(None, [row["id"], row["setting"], row["side"], str(n)]))


def run_one(args, ident, write, row, path, n):
    """Run one input, write its row and return its verdict; nothing (None) once the run is stopped."""
    if STOP.is_set():
        return None
    name = run_name(row, n)
    rundir = os.path.join(os.path.abspath(args.out) + ".d", name)
    os.makedirs(rundir)  # OUT.d is new, so the directory holds this run's files only, and no verdicts
    flags, test_env, edition = header(path)
    env = environment(args, test_env, row, rundir)
    solver_env = env
    if args.check_solution:
        write_check_config(env, rundir)
        solver_env = dict(env, COAR_CONFIG_DIR=rundir, COAR_CONFIG="solver-config.json",
                          THRUST_PCSAT_OUTPUT=os.path.join(rundir, "solver.out"))
    cmd = [os.path.join(build_dir(args), "thrust-rustc"), path, "--edition", edition,
           *crate_type_flags(path), *flags, "--out-dir", rundir]
    started = dict(load1=f"{os.getloadavg()[0]:.1f}",
                   date=datetime.datetime.now().astimezone().isoformat(timespec="seconds"))
    start = time.monotonic()
    code, log = solve(cmd, args.thrust, solver_env)
    wall_s = f"{time.monotonic() - start:.2f}"
    with open(os.path.join(rundir, "thrust.log"), "w") as f:
        f.write(shlex.join(cmd) + "\n" + log)
    listed = {k: v for k, v in row.items() if k != "printed"}  # the rows record what ran, not the selection
    record = dict(listed, **ident, **started, **ended(args, env, path, rundir, code, log), wall_s=wall_s, run=n)
    write(record, name)
    return record["verdict"]


def write_skipped(args, ident, write, row, path, n):
    """The row of a run --skip-sizes-past-cap does not start: `skipped`, with the settings and the
    input it would have run, and no time, load or query."""
    flags, test_env, _ = header(path)
    env = environment(args, test_env, row, "")
    listed = {k: v for k, v in row.items() if k != "printed"}
    write(dict(listed, **ident, **settings(args, env), verdict="skipped", time_s="", run=n, file_sha256=sha256(path),
               date=datetime.datetime.now().astimezone().isoformat(timespec="seconds")), run_name(row, n))


SIZE = re.compile(r"(.+)_k(\d+)")


def series(todo):
    """The tasks grouped for --skip-sizes-past-cap: per series (the id without its `_k<size>`, the
    setting and the side) a list of sizes in increasing order, each the tasks of its runs; an id
    without a size is a series of its own. In the order of each series' first task."""
    groups = {}
    for task in todo:
        row = task[0]
        m = SIZE.fullmatch(row["id"])
        key = (m.group(1) if m else row["id"], row["setting"], row["side"])
        size = int(m.group(2)) if m else 0
        groups.setdefault(key, {}).setdefault(size, []).append(task)
    return [[sizes[k] for k in sorted(sizes)] for sizes in groups.values()]


def run_series(args, ident, write, sizes):
    """Run a series' sizes in order; after a size every run of which reached the cap, write the
    later sizes' rows as `skipped`. Stops writing once the run is stopped."""
    for i, tasks_of_size in enumerate(sizes):
        verdicts = [run_one(args, ident, write, *task) for task in tasks_of_size]
        if STOP.is_set():
            return
        if all(v in ("timeout", "oom") for v in verdicts):
            for task in (t for later in sizes[i + 1:] for t in later):
                if STOP.is_set():
                    return
                write_skipped(args, ident, write, *task)
            return


def checked_verdict(args, result, counts):
    """With the check on, a `sat` without the check's line is an `error` (the check did not run or
    its output was not kept), and one whose check failed is `check-failed`."""
    if not args.check_solution or result != "sat":
        return result
    if not counts:
        return "error"
    return "check-failed" if int(counts["check_failed"]) > 0 else result


def ended(args, env, path, rundir, code, log):
    """The columns of a run that ended: its verdict and time, its settings, the input's hash and
    invariants, the emitted query's facts and the containers of other runs."""
    counts = check_counts(os.path.join(rundir, "solver.out"))
    result = checked_verdict(args, verdict(code, log, STOP.is_set()), counts)
    record = settings(args, env)
    calls = solver_calls(log)
    seconds, parts = (calls[-1] if calls else None), None
    if splits(env):
        result, seconds, parts = split_run(log, result, record["timeout_s"])
    elif seconds is None and result == "timeout":
        seconds = record["timeout_s"]
    invariants = len(re.findall(r"^\s*(thrust_macros::)?invariant!", open(path).read(), re.M))
    return dict(record, verdict=result, time_s="" if seconds is None else f"{seconds:.2f}",
                cache_hits=cache_hits(log), invariants=invariants, parts="" if parts is None else parts,
                other_containers=other_containers(args.label),
                file_sha256=sha256(path), **counts,
                **query_facts(os.path.join(rundir, "thrust_output.smt2")))


def tasks(args):
    """(row, file, run number) of every run, in the order of the list and the runs: the printed rows
    and their fail twins, or with --diagnostic every row; stops on an id of --only that has no such
    row, so that a selection cannot shrink unseen."""
    base = os.path.dirname(os.path.abspath(args.list))
    reader = csv.DictReader(open(args.list))
    if "printed" not in reader.fieldnames and not args.diagnostic:
        sys.exit(f"{args.list} has no `printed` column; run every row with --diagnostic")
    rows = list(reader)
    listed = rows if args.diagnostic else measured(rows)
    only = set(filter(None, (args.only or "").split(",")))
    unknown = sorted(only - {r["id"] for r in listed})
    if unknown:
        sys.exit(f"{args.list} has no {'' if args.diagnostic else 'printed '}row for {unknown}")
    rows = [r for r in listed if not only or r["id"] in only]
    return [(r, os.path.join(base if r["file"].startswith("./") else args.thrust, r["file"]), n)
            for n in range(1, args.runs + 1) for r in rows]


def measured(rows):
    """The rows a run takes without --diagnostic: the printed ones and the fail twins of their id
    and setting."""
    printed = {(r["id"], r["setting"]) for r in rows if r["printed"] == "yes"}
    return [r for r in rows if r["printed"] == "yes" or (r["side"] == "fail" and (r["id"], r["setting"]) in printed)]


def show_series(todo):
    """--skip-sizes-past-cap under -n: each series and its sizes, in the order they run."""
    for sizes in series(todo):
        row = sizes[0][0][0]
        ids = [tasks_of_size[0][0]["id"] for tasks_of_size in sizes]
        print("series", row["setting"] or "-", row["side"], " < ".join(ids))


def show(args, ident, todo):
    """What a run would start: its identity, then each run with its solver settings."""
    for key in ("thrust", "thrust_tree", "thrust_rustc", "solver", "image", "image_id", "fptprove",
                "research", "research_tree"):
        print(f"{key}: {ident[key]}")
    for row, path, n in todo:
        if not os.path.exists(path):
            print(run_name(row, n), row["expect"], row["file"], "missing")
            continue
        flags, test_env, _ = header(path)
        s = settings(args, environment(args, test_env, row, "OUT.d"))
        print(run_name(row, n), row["expect"], row["file"], f"crate={' '.join(crate_type_flags(path)) or 'bin'}", f"timeout={s['timeout_s']}", *([f"split({args.parts_at_once} at once, timeout per part, solver time summed in time_s)"] if splits(environment(args, test_env, row, "OUT.d")) else []),
              f"config={s['config']}", f"check_solution={s['check_solution']}", f"cpus={s['cpus'] or '-'}", f"memory={s['memory'] or '-'}",
              f"verdict_cache={s['verdict_cache']}" + (f"({verdict_cache_dir(args, os.path.join('OUT.d', run_name(row, n)))})" if s['verdict_cache'] != 'off' else ""),
              *(["env:", row["env"]] if row["env"] else []))


def rows_writer(f):
    """A function that writes one row and its progress line, and flushes it."""
    writer = csv.DictWriter(f, FIELDS, lineterminator="\n")
    writer.writeheader()
    f.flush()

    def write(row, name):
        with LOCK:
            writer.writerow(row)
            f.flush()
            print(name, row["verdict"], row["time_s"], file=sys.stderr, flush=True)
    return write


def run_all(args, ident, todo):
    """Start the runs, at most --jobs at once; on SIGINT, SIGTERM or SIGHUP, end the runs in flight and
    start no more."""
    for sig in (signal.SIGTERM, signal.SIGHUP):
        signal.signal(sig, signal.default_int_handler)
    with open(args.out, "x", newline="") as f:
        write = rows_writer(f)
        pool = ThreadPoolExecutor(args.jobs)
        try:
            jobs = ([pool.submit(run_series, args, ident, write, sizes) for sizes in series(todo)]
                    if args.skip_sizes_past_cap else [pool.submit(run_one, args, ident, write, *task) for task in todo])
            for future in jobs:
                future.result()
        except BaseException:
            for sig in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
                signal.signal(sig, signal.SIG_IGN)  # the rows in flight are written before the file closes
            stop_runs()
            raise
        finally:
            pool.shutdown(cancel_futures=True)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("-n", action="store_true", help="check and print the runs, start none")
    ap.add_argument("--dirty", action="store_true", help="allow uncommitted changes, and record them")
    ap.add_argument("--thrust", default="thrust", help="the Thrust tree, built (default thrust)")
    ap.add_argument("--thrust-commit", required=True, help="the commit the tree must be at")
    ap.add_argument("--fptprove-commit", required=True, help="the commit the solver was built from")
    solver = ap.add_mutually_exclusive_group(required=True)
    solver.add_argument("--image", help="the solver image, for the tree's " + WRAPPER)
    solver.add_argument("--solver", help="a solver command instead of the wrapper and an image")
    ap.add_argument("--check-solution", action="store_true", help="run CoAR with check_solution on (needs --image)")
    ap.add_argument("--runs", type=int, default=1)
    ap.add_argument("--jobs", type=int, default=1, help="runs at once")
    ap.add_argument("--timeout", type=int, default=300)
    ap.add_argument("--only", help="the ids to run, comma-separated")
    ap.add_argument("--diagnostic", action="store_true",
                    help="also run the inputs kept for diagnosis and the twins of unprinted inputs")
    ap.add_argument("--parts-at-once", type=int, default=1,
                    help="parts of a split root solved at a time (D122; THRUST_SPLIT=N)")
    ap.add_argument("--skip-sizes-past-cap", action="store_true",
                    help="run each series' sizes in order and skip those after a size no run answered (D113)")
    cache = ap.add_mutually_exclusive_group()
    cache.add_argument("--keep-verdicts", action="store_true",
                       help="keep each run's verdicts in OUT.d/<run>/verdicts/, new and empty when it starts (D124)")
    cache.add_argument("--reuse-verdicts", metavar="DIR",
                       help="answer from and add to the verdict cache DIR, for checking verdicts; the times are not measured")
    ap.add_argument("list")
    ap.add_argument("out")
    args = ap.parse_args()
    args.thrust = os.path.abspath(args.thrust)
    if args.check_solution and not args.image:
        ap.error("--check-solution needs --image")
    if args.check_solution and (args.keep_verdicts or args.reuse_verdicts):
        ap.error("--check-solution solves every query with the check; it keeps no verdicts")
    args.reuse_verdicts = os.path.abspath(args.reuse_verdicts) if args.reuse_verdicts else ""
    if args.parts_at_once < 1:
        ap.error("--parts-at-once needs a positive number")
    args.solver = os.path.abspath(args.solver) if args.solver else WRAPPER
    args.image = args.image or ""
    sysroot = subprocess.run(["rustc", "--print", "sysroot"], cwd=args.thrust, capture_output=True, text=True)
    args.rustc_lib = os.path.join(sysroot.stdout.strip(), "lib")
    args.label = f"thrust-run={os.getpid()}"
    todo = tasks(args)
    ident = identity(args)
    args.image_ref = ident["image_id"] or args.image  # the wrapper starts the image checked, not what the tag names later
    if args.n:
        show(args, ident, todo)
        if args.skip_sizes_past_cap:
            show_series(todo)
        check(args, ident, todo)
        return
    check(args, ident, todo)
    try:
        run_all(args, ident, todo)
    except KeyboardInterrupt:
        sys.exit("stopped: the runs in flight are recorded as interrupted")


if __name__ == "__main__":
    main()
