#!/bin/bash
# The entry point that reproduces the paper's tables and figures; every step delegates to
# scripts/run.py and scripts/run-evaluation.sh and starts nothing else.
#
# usage: artifact/reproduce.sh [-n] [--thrust DIR] [--image IMAGE] [--jobs N]
#            quick OUT | full OUT | tables [RUN]
#
#   quick OUT   the quick run: a handful of inputs of RQ1, RQ3 and RQ4 with their fail
#               twins, one run each with a 60 s cap; prints each verdict next to its expected one
#               and fails on a difference or a missing row. Writes quick-rq1.csv, quick-rq3.csv, quick-rq4.csv to OUT.
#   full OUT    the full run: every step of the evaluation (smoke, rq1, rq3, families, rq4, tables) into OUT,
#               on the inputs the paper prints, three runs per input and 300 s per solver call (times: README.md, Reproducing the
#               paper)
#   tables RUN  regenerate every table and figure from the rows of RUN (default: the reported
#               run) and the counts of the Thrust tree DIR, without a solver; DIR must be at the
#               Thrust commit of RUN/versions.env, and this checkout at the commit the rows record
#   -n          print the commands each step would run, and the runs behind them, and run nothing;
#               quick then exits 1 when a run would stop at one of run.py's checks
#   --thrust    the Thrust tree, built (default thrust/)
#   --image     the solver image (default coar:F)
#   --jobs      solver containers at once (default 4; full's rq1 runs two, run-evaluation.sh)
#   OUT         an existing directory whose versions.env names the fptprove commit F and the Thrust
#               commit T (copy the reported run's); the rows of a step are written there, and a
#               step stops when its rows exist
set -euo pipefail

ART=$(cd "$(dirname "$0")" && pwd)
REPORTED=$ART/results/2026-11-13-ab598199
QUICK_RQ1=range,iter_mut,map_call
QUICK_RQ3=inferred_k1,inferred_k2
QUICK_RQ4=lemmas
THRUST=thrust
IMAGE=
JOBS=4
DRY=
WOULD_FAIL=

usage() {
  sed -n 2,24p "$0" >&2
  exit 2
}

quick_run() {  # LIST IDS NAME: the inputs IDS of LIST, once, 60 s, into OUT/quick-NAME.csv
  local cmd=(python3 "$ART/scripts/run.py" $DRY --diagnostic --thrust "$THRUST" --thrust-commit "$T"
    --fptprove-commit "$F" --image "$IMAGE" --runs 1 --jobs "$JOBS" --timeout 60 --only "$2"
    "$ART/$1" "$OUT/quick-$3.csv")
  echo "+ ${cmd[*]}"
  "${cmd[@]}" && return
  [ -n "$DRY" ] || exit 1
  WOULD_FAIL=1  # under -n a failed check does not end the listing of the later runs
}

quick_report() {  # each quick row against its expected verdict, and every row of the chosen ids present
  python3 - "$ART" "$OUT" "$QUICK_RQ1" "$QUICK_RQ3" "$QUICK_RQ4" <<'PY'
import csv, sys
art, out, *ids = sys.argv[1:]
lists = {"rq1": "comparison-creusot", "rq3": "scalability-evaluation", "rq4": "rustc-coroutine-layout"}
bad = 0
for (name, directory), chosen in zip(lists.items(), ids):
    chosen = chosen.split(",")
    expected = sum(r["id"] in chosen for r in csv.DictReader(open(f"{art}/{directory}/runs.csv")))
    rows = list(csv.DictReader(open(f"{out}/quick-{name}.csv")))
    if len(rows) != expected:
        print(f"{name}: {len(rows)} rows, the list has {expected} for {','.join(chosen)}  DIFFERS")
        bad += 1
    for r in rows:
        ok = r["verdict"] == r["expect"]
        bad += not ok
        print(f'{r["id"]:<14}{r["setting"]:<14}{r["side"]:<6}{r["verdict"]:<9}'
              f'expected {r["expect"]:<7}{r["time_s"]:>7} s  load {r["load1"]}  {"ok" if ok else "DIFFERS"}')
sys.exit(1 if bad else 0)
PY
}

step_quick() {
  source "$OUT/versions.env"
  IMAGE=${IMAGE:-coar:$F}
  quick_run comparison-creusot/runs.csv "$QUICK_RQ1" rq1
  quick_run scalability-evaluation/runs.csv "$QUICK_RQ3" rq3
  quick_run rustc-coroutine-layout/runs.csv "$QUICK_RQ4" rq4
  [ -n "$DRY" ] || quick_report
  [ -z "$WOULD_FAIL" ] || { echo "the quick run would stop at a check above" >&2; exit 1; }
}

step_full() {
  local args=(--thrust "$THRUST" --jobs "$JOBS")
  [ -z "$IMAGE" ] || args+=(--image "$IMAGE")
  echo "+ run-evaluation.sh $DRY ${args[*]} $OUT all"
  "$ART/scripts/run-evaluation.sh" $DRY "${args[@]}" "$OUT" all
}

step_tables() {
  echo "+ run-evaluation.sh $DRY --thrust $THRUST $OUT tables"
  "$ART/scripts/run-evaluation.sh" $DRY --thrust "$THRUST" "$OUT" tables
}

while [[ "${1:-}" == -* ]]; do
  case $1 in
    -n) DRY=-n; shift ;;
    --thrust) THRUST=$2; shift 2 ;;
    --image) IMAGE=$2; shift 2 ;;
    --jobs) JOBS=$2; shift 2 ;;
    *) usage ;;
  esac
done
STEP=${1:-}
case $STEP in
  quick | full) [ $# = 2 ] || usage ;;
  tables) [ $# -le 2 ] || usage ;;
  *) usage ;;
esac
OUT=$(realpath "${2:-$REPORTED}")
[ -f "$OUT/versions.env" ] || { echo "$OUT/versions.env is missing" >&2; exit 1; }
[ -d "$THRUST" ] || { echo "the Thrust tree $THRUST does not exist (--thrust DIR)" >&2; exit 1; }
"step_$STEP"
