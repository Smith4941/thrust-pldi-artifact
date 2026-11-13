#!/usr/bin/env python3
"""Time Creusot's side of RQ1 per case, from the case's Rust file to the proof search's result.

usage: creusot-time.py [-n] --creusot DIR [--creusot-std-build STD] [--runs N] [--only CASE,...]
                       [--write-creusot-csv] OUT

The time Table tab:creusot prints for Creusot (decisions.md D123), as Thrust's is the wall time of
its command from the Rust file to the verdict. For each case of comparison-creusot/creusot.csv
with a proof (every row but `common`), each run makes a new directory OUT.d/run<k>/<case>/ with
the case's file and `common.rs` of comparison-creusot/creusot-port/ and DIR's `why3find.json`, and
takes one clock over two commands:

- the translation: DIR's release `target/release/creusot-rustc` on the file against creusot-std
  built at DIR's commit (STD, default DIR/target/creusot/debug, where Creusot's `./t ui` builds
  it: `libcreusot_std.rlib`, `libcreusot_std.cmeta` and `deps/`), with the flags of
  Creusot's ui test, as experiments/2026-11-07-creusot-search-with-hints/compile.sh runs it;
- the proof search on the `.coma` it wrote: `why3find prove --no-cache` from scratch (no
  certificate, no Why3 session), Creusot's `why3.conf` capped at two provers at a time, as
  search-one.sh of the same record runs it (experiments/2026-11-11-creusot-per-case-search.md).

The runs go run by run, each over the cases in creusot.csv's order, one at a time. OUT is a CSV
with one row per case and run: the run, the case, the file and its sha256, each command's exit
status and seconds (`translate_s`, `search_s`), `wall_s` (both, from before the translation starts
to after the search exits), the goals the search left open (`unproved`), the 1-minute load before
the case and the containers running after it (the protocol quotes times from an idle machine;
docs/evaluation/README.md), DIR's commit and this repository's commit. After the runs it
prints per case the value of creusot.csv's `wall`: the median `wall_s` over the runs in which both
commands exited 0 and no goal stayed open, with the number of such runs, or `unproved` when there
is none; with --write-creusot-csv it also writes those values into the `wall` column of
comparison-creusot/creusot.csv, for the cases it ran, which is what run-evaluation.sh's `creusot`
step does. -n prints the commands of every run and checks that the tools exist, without starting
any. OUT and OUT.d must not exist. The protocol quotes these times only from an idle machine:
run-evaluation.sh runs the step after its solver steps, and the rows record the load.
"""

import argparse
import csv
import datetime
import hashlib
import os
import re
import shutil
import statistics
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
PORT = os.path.join(HERE, "..", "comparison-creusot")
TOOLS = os.path.expanduser("~/.local/share/creusot/bin")  # the installer's why3find and provers
WHY3_CONF = os.path.expanduser("~/.config/creusot/why3.conf")
FIELDS = ["run", "case", "file", "file_sha256", "translate_exit", "translate_s", "search_exit", "search_s",
          "wall_s", "unproved", "load1", "other_containers", "date", "creusot", "research"]
UNPROVED = re.compile(r"^Goal (\S+): ✘", re.M)


def git_head(path):
    p = subprocess.run(["git", "-C", path, "rev-parse", "--short=9", "HEAD"], capture_output=True, text=True)
    return p.stdout.strip() if p.returncode == 0 else ""


def sha256(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()[:16]


def cases(only):
    """(case, file) of creusot.csv's rows with a proof, in its order; stops on a case of --only it lacks."""
    rows = [(r["case"], r["file"]) for r in csv.DictReader(open(os.path.join(PORT, "creusot.csv")))
            if r["case"] != "common"]
    wanted = set(filter(None, (only or "").split(",")))
    unknown = sorted(wanted - {c for c, _ in rows})
    if unknown:
        sys.exit(f"creusot.csv has no case {unknown}")
    return [(c, f) for c, f in rows if not wanted or c in wanted]


def translate_cmd(args, name):
    """creusot-rustc on <name>.rs, as compile.sh of the hints record runs it; it prints the .coma."""
    std = args.creusot_std_build
    return [os.path.join(args.creusot, "target", "release", "creusot-rustc"), "--diagnostic-width=100",
            "-Zwrite-long-types-to-disk=no", "--edition=2024", "-Zno-codegen", "--crate-type=lib",
            "--extern", f"creusot_std={std}/libcreusot_std.rlib", f"-Ldependency={std}/deps/",
            f"{name}.rs", "--", "--stdout", "--export-metadata=false", "--spans-relative-to=.",
            "--span-mode=off", "--creusot-extern", f"creusot_std={std}/libcreusot_std.cmeta"]


def search_cmd(name):
    return [os.path.join(TOOLS, "why3find"), "prove", f"examples/iterators/{name}.coma", "--no-cache"]


def environments(args, out_d):
    """The translation's environment (the libraries of DIR's toolchain) and the search's (two
    provers at a time, DIR's Why3 prelude)."""
    sysroot = subprocess.run(["rustc", "--print", "sysroot"], cwd=args.creusot, capture_output=True, text=True)
    translate = dict(os.environ, LD_LIBRARY_PATH=os.path.join(sysroot.stdout.strip(), "lib"))
    conf = os.path.join(out_d, "why3.conf")
    search = dict(os.environ, WHY3CONFIG=conf,
                  DUNE_DIR_LOCATIONS=f"why3find:lib:{os.path.join(args.creusot, 'target', 'creusot')}")
    return translate, search, conf


def two_provers(conf):
    """Creusot's why3.conf with `running_provers_max = 2`, as search-one.sh writes it."""
    text = open(WHY3_CONF).read()
    with open(conf, "w") as f:
        f.write(re.sub(r"^running_provers_max = .*$", "running_provers_max = 2", text, flags=re.M))


def prepare(args, case, file, rundir):
    """rundir/examples/iterators/ with the case's file and common.rs, and DIR's why3find.json."""
    src = os.path.join(rundir, "examples", "iterators")
    os.makedirs(src)
    name = os.path.basename(file)[:-3]
    shutil.copy(os.path.join(PORT, file), os.path.join(src, f"{name}.rs"))
    shutil.copy(os.path.join(PORT, "creusot-port", "common.rs"), src)
    shutil.copy(os.path.join(args.creusot, "why3find.json"), rundir)
    return src, name


def other_containers():
    p = subprocess.run(["docker", "ps", "-q"], capture_output=True, text=True)
    return len(p.stdout.split()) if p.returncode == 0 else ""


def time_case(args, envs, run, case, file, out_d):
    """One run of one case: the translation and the search under one clock."""
    rundir = os.path.join(out_d, f"run{run}", case)
    src, name = prepare(args, case, file, rundir)
    load = f"{os.getloadavg()[0]:.2f}"
    date = datetime.datetime.now().astimezone().isoformat(timespec="seconds")
    start = time.monotonic()
    with open(os.path.join(src, f"{name}.coma"), "w") as coma, open(os.path.join(rundir, "translate.log"), "w") as log:
        translated = subprocess.run(translate_cmd(args, name), cwd=src, env=envs[0], stdout=coma, stderr=log)
    middle = time.monotonic()
    search_exit, unproved = "", ""
    if translated.returncode == 0:
        searched = subprocess.run(search_cmd(name), cwd=rundir, env=envs[1], capture_output=True, text=True)
        open(os.path.join(rundir, "search.log"), "w").write(searched.stdout + searched.stderr)
        search_exit, unproved = searched.returncode, " ".join(UNPROVED.findall(searched.stdout + searched.stderr))
    end = time.monotonic()
    return dict(run=run, case=case, file=file, file_sha256=sha256(os.path.join(PORT, file)),
                translate_exit=translated.returncode, translate_s=f"{middle - start:.2f}",
                search_exit=search_exit, search_s=f"{end - middle:.2f}" if search_exit != "" else "",
                wall_s=f"{end - start:.2f}", unproved=unproved, load1=load,
                other_containers=other_containers(), date=date)


def summary(rows):
    """Per case, creusot.csv's `wall`: the median wall_s of its proved runs, or `unproved`; printed
    with the number of proved runs."""
    by_case = {}
    for r in rows:
        by_case.setdefault(r["case"], []).append(r)
    walls = {}
    for case, runs in by_case.items():
        proved = [float(r["wall_s"]) for r in runs if r["translate_exit"] == 0 and r["search_exit"] == 0
                  and not r["unproved"]]
        walls[case] = f"{statistics.median(proved):.2f}" if proved else "unproved"
        print(f"{case}\twall={walls[case]}\tproved {len(proved)}/{len(runs)}")
    return walls


def write_creusot_csv(walls):
    """comparison-creusot/creusot.csv with `wall` replaced for the cases in walls."""
    path = os.path.join(PORT, "creusot.csv")
    reader = csv.DictReader(open(path))
    rows = [dict(r, wall=walls.get(r["case"], r["wall"])) for r in reader]
    with open(path, "w", newline="") as f:
        writer = csv.DictWriter(f, reader.fieldnames, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def missing_tools(args):
    paths = [os.path.join(args.creusot, "target", "release", "creusot-rustc"),
             os.path.join(args.creusot, "why3find.json"), os.path.join(args.creusot, "target", "creusot"),
             os.path.join(args.creusot_std_build, "libcreusot_std.rlib"),
             os.path.join(args.creusot_std_build, "libcreusot_std.cmeta"),
             os.path.join(TOOLS, "why3find"), WHY3_CONF]
    return [p for p in paths if not os.path.exists(p)]


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("-n", action="store_true", help="print the commands of every run and check the tools, run none")
    ap.add_argument("--creusot", required=True, help="the Creusot checkout (3620de437), built with --release")
    ap.add_argument("--creusot-std-build", help="creusot-std built at that commit (default DIR/target/creusot/debug)")
    ap.add_argument("--write-creusot-csv", action="store_true", help="write the medians into creusot.csv's `wall`")
    ap.add_argument("--runs", type=int, default=3)
    ap.add_argument("--only", help="the cases to run, comma-separated")
    ap.add_argument("out")
    args = ap.parse_args()
    args.creusot = os.path.abspath(args.creusot)
    args.creusot_std_build = os.path.abspath(args.creusot_std_build or os.path.join(args.creusot, "target", "creusot", "debug"))
    out_d = os.path.abspath(args.out) + ".d"
    todo = [(run, case, file) for run in range(1, args.runs + 1) for case, file in cases(args.only)]
    if args.n:
        for run, case, file in todo:
            name = os.path.basename(file)[:-3]
            print(f"run {run} {case}: {' '.join(translate_cmd(args, name))} > {name}.coma; "
                  f"{' '.join(search_cmd(name))}  (in {os.path.join(out_d, f'run{run}', case)})")
    missing = missing_tools(args)
    if missing:
        sys.exit(f"missing: {missing}")
    if os.path.exists(args.out) or os.path.exists(out_d):
        sys.exit(f"{args.out} or {out_d} exists: give a new OUT")
    if args.n:
        return
    os.makedirs(out_d)
    envs = environments(args, out_d)
    two_provers(envs[2])
    ident = dict(creusot=git_head(args.creusot), research=git_head(HERE))
    rows = []
    with open(args.out, "x", newline="") as f:
        writer = csv.DictWriter(f, FIELDS, lineterminator="\n")
        writer.writeheader()
        for run, case, file in todo:
            row = dict(time_case(args, envs, run, case, file, out_d), **ident)
            writer.writerow(row)
            f.flush()
            rows.append(row)
            print(run, case, row["wall_s"], row["unproved"] or "proved", file=sys.stderr, flush=True)
    walls = summary(rows)
    if args.write_creusot_csv:
        write_creusot_csv(walls)


if __name__ == "__main__":
    main()
