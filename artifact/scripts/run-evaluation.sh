#!/bin/bash
# Run the evaluation of the paper and write its tables and figures from the rows. The tables step
# first counts the tree and the RQ3 programs (count-annotations) into the CSVs the tables read.
#
# usage: artifact/scripts/run-evaluation.sh [-n] [--thrust DIR] [--image IMAGE]
#            [--runs N] [--jobs N] [--diagnostic] [--reuse-verdicts DIR]
#            [--creusot DIR [--creusot-std-build STD]] OUT STEP...
#
#   -n          print the commands, and the runs each step would start, without running them
#   --thrust    the Thrust tree, built with `cargo build --release` (default thrust/); run.py runs
#               target/release/thrust-rustc, whose wall time the tables print (decisions.md D123)
#   --image     the solver image (default coar:F)
#   --runs N    runs per input of rq1 and rq3 (default 3); families and rq4 run once
#   --jobs N    solver containers at once in rq3 and families (default 4); each has 4 CPUs and
#               8 GB. rq1 ignores it and runs two at once (decisions.md D127); rq4 ignores it: it
#               runs one input at a time and solves a split root's parts four at a time
#               (decisions.md D126), so at most four containers of 8 GB
#   --diagnostic  also run the inputs no table prints: the inputs kept for diagnosis and the twins
#               of unprinted inputs (run.py --diagnostic); without it a step runs the printed rows
#               of its list and their fail twins
#   --reuse-verdicts DIR  run every input but RQ4's with the verdict cache DIR (the shipped one,
#               OUT/verdicts/ of the reported run), to check the verdicts quickly; such rows
#               measure no time and tables.py refuses them. Without it each run keeps its verdicts
#               in a new, empty directory of its own (run.py --keep-verdicts, decisions.md D124);
#               RQ4 runs without a cache
#   --creusot   the Creusot checkout at 3620de437 for the creusot step, and STD its creusot-std
#               build (creusot-time.py's options)
#   OUT         the directory of the run; OUT/versions.env names the Thrust commit T and the
#               fptprove commit F the image was built from, which run.py checks against the tree
#               and the image; a step stops when its rows exist
#   STEP        smoke, rq1, rq3, families, rq4, creusot, tables, verdicts, or all (each of them
#               in this order); creusot times Creusot's side of tab:creusot after the solver
#               steps, on the idle machine (creusot-time.py), and writes creusot.csv's `wall`;
#               verdicts merges the runs' verdicts into OUT/verdicts/, the shipped cache;
#               smoke runs ten ui tests to check the build and the image, the full ui suite
#               being a development check, not a step of the evaluation
set -euo pipefail

ART=$(cd "$(dirname "$0")/.." && pwd)
THRUST=thrust
IMAGE=
RUNS=3
JOBS=4
DRY=
DIAGNOSTIC=
VERDICTS=(--keep-verdicts)
CREUSOT=()

runs() {  # LIST NAME RUNS TIMEOUT JOBS [RUN.PY OPTIONS...]: the inputs of LIST into OUT/NAME.csv,
  # JOBS runs at once; with -n, list them
  local cmd=(python3 "$ART/scripts/run.py" $DRY $DIAGNOSTIC "${VERDICTS[@]}" --thrust "$THRUST" --thrust-commit "$T" --fptprove-commit "$F"
    --image "$IMAGE" --runs "$3" --jobs "$5" --timeout "$4" "${@:6}" "$ART/$1" "$OUT/$2.csv")
  echo "+ ${cmd[*]}"
  "${cmd[@]}" || [ -n "$DRY" ]  # under -n a failed check does not end the listing of the later steps
}

# Five pass/fail pairs of different kinds, each answered in about a second: Z3 alone, a generic
# adapter with traits, a closure contract, a loop invariant, an FnMut closure with a history.
SMOKE_TESTS=(
  tests/ui/pass/add.rs tests/ui/fail/add.rs
  tests/ui/pass/creusot/fuse.rs tests/ui/fail/creusot/fuse.rs
  tests/ui/pass/creusot/weaker/map_call.rs tests/ui/fail/creusot/weaker/map_call.rs
  tests/ui/pass/iterators/loop_invariant_range_loop.rs tests/ui/fail/iterators/loop_invariant_range_loop.rs
  tests/ui/pass/creusot/counter.rs tests/ui/fail/creusot/counter.rs
)

step_smoke() {  # the build and the image on SMOKE_TESTS; a failure stops the run
  echo "+ (cd $THRUST && COAR_IMAGE=$IMAGE cargo test --test ui -- --exact ${SMOKE_TESTS[*]}) > $OUT/smoke.log"
  [ -n "$DRY" ] && return
  (cd "$THRUST" && COAR_IMAGE=$IMAGE cargo test --test ui -- --exact "${SMOKE_TESTS[@]}") > "$OUT/smoke.log" 2>&1 \
    || { tail -20 "$OUT/smoke.log" >&2; exit 1; }
  tail -1 "$OUT/smoke.log"
}

check_noinv() {  # in a git checkout, RQ2's inferred inputs must be the tests of T without invariants
  [ -e "$THRUST/.git" ] || return 0
  echo "+ comparison-creusot/noinv.py --check --thrust $THRUST $T"
  python3 "$ART/comparison-creusot/noinv.py" --check --thrust "$THRUST" "$T" || [ -n "$DRY" ]  # under -n a failed check does not end the listing
}

# RQ1 runs two containers at once: its times are the table's comparison with Creusot (decisions.md D127)
step_rq1() { check_noinv; runs comparison-creusot/runs.csv rq1 "$RUNS" 300 2; }
# RQ3 skips the sizes of a series after one no run answered (decisions.md D113)
step_rq3() { runs scalability-evaluation/runs.csv rq3 "$RUNS" 300 "$JOBS" --skip-sizes-past-cap; }
step_families() { runs scalability-input-families/runs.csv families 1 60 "$JOBS"; }
# RQ4 runs its inputs one after another and a split root's parts four at once (decisions.md D126)
step_rq4() {  # without a verdict cache: a split root has parts with a byte-identical query, which the
  # cache would answer from an earlier part (D124)
  local VERDICTS=()
  runs rustc-coroutine-layout/runs.csv rq4 1 300 1 --parts-at-once 4
}
step_creusot() {  # Creusot's time per case into OUT/creusot-time.csv and creusot.csv's `wall` (D123),
  # after the solver steps so that nothing else runs
  [ ${#CREUSOT[@]} -gt 0 ] || { echo "the creusot step needs --creusot DIR" >&2; [ -n "$DRY" ] && return; exit 1; }
  local cmd=(python3 "$ART/scripts/creusot-time.py" $DRY "${CREUSOT[@]}" --write-creusot-csv "$OUT/creusot-time.csv")
  echo "+ ${cmd[*]}"
  "${cmd[@]}" || [ -n "$DRY" ]
}

count() {  # OUT ARGS...: count-annotations ARGS into OUT
  local out=$1
  shift
  echo "+ count-annotations $* > $out"
  [ -n "$DRY" ] && return
  cargo run -q --release --manifest-path "$ART/count-annotations/Cargo.toml" -- "$@" > "$out.tmp"
  mv "$out.tmp" "$out"
}

check_tree() {  # a git checkout as the tree must be at the run's Thrust commit T, without changes
  [ -e "$THRUST/.git" ] || return 0
  local head
  head=$(git -C "$THRUST" rev-parse HEAD)
  [[ $head == "$T"* ]] || tree_fails "the tree is at $head, the run at $T"
  [ -z "$(git -C "$THRUST" status --porcelain)" ] || tree_fails "the tree has uncommitted changes"
}

tree_fails() {  # MESSAGE: stop, or under -n report it and go on listing
  echo "$1" >&2
  [ -n "$DRY" ] || exit 1
}

step_tables() {  # the counts of the tree and of the RQ3 programs, then the tables and figures;
  # tables.log keeps the unrefuted fail twins tables.py lists, for the run's record
  local cc=$ART/comparison-creusot rc=$ART/rustc-coroutine-layout se=$ART/scalability-evaluation
  check_tree
  count "$cc/annotations.csv" creusot "$THRUST" "$cc/case-items.csv"
  count "$rc/stages.csv" coroutine "$THRUST" "$rc/parts.csv" "$rc/upstream"
  count "$rc/functions.csv" coroutine --functions "$THRUST" "$rc/parts.csv" "$rc/upstream"
  count "$rc/rows.csv" coroutine --rows "$THRUST" "$rc/parts.csv" "$rc/upstream"
  count "$se/code-lines.csv" scalability "$se/programs/pass"
  echo "+ python3 $ART/scripts/tables.py $OUT 2> $OUT/tables.log"
  [ -n "$DRY" ] && return
  local status=0
  python3 "$ART/scripts/tables.py" "$OUT" 2> "$OUT/tables.log" || status=$?
  cat "$OUT/tables.log" >&2
  return "$status"
}

step_verdicts() {  # the verdicts every run kept (OUT/<step>.csv.d/<run>/verdicts/) into
  # OUT/verdicts/, the cache the artifact ships; an entry two runs wrote differently stops it
  echo "+ merge $OUT/*.csv.d/*/verdicts/* into $OUT/verdicts/"
  [ -n "$DRY" ] && return
  [ ! -e "$OUT/verdicts" ] || { echo "$OUT/verdicts exists" >&2; exit 1; }
  mkdir "$OUT/verdicts"
  local entry target
  for entry in "$OUT"/*.csv.d/*/verdicts/*; do
    [ -e "$entry" ] || continue
    target=$OUT/verdicts/$(basename "$entry")
    if [ -e "$target" ]; then
      cmp -s "$entry" "$target" || { echo "$entry differs from an earlier run's $target" >&2; exit 1; }
      continue
    fi
    cp "$entry" "$target"
  done
  echo "$(ls "$OUT/verdicts" | wc -l) verdicts in $OUT/verdicts"
}

while [[ "${1:-}" == -* ]]; do
  case $1 in
    -n) DRY=-n; shift ;;
    --thrust) THRUST=$2; shift 2 ;;
    --image) IMAGE=$2; shift 2 ;;
    --runs) RUNS=$2; shift 2 ;;
    --jobs) JOBS=$2; shift 2 ;;
    --diagnostic) DIAGNOSTIC=--diagnostic; shift ;;
    --reuse-verdicts) VERDICTS=(--reuse-verdicts "$2"); shift 2 ;;
    --creusot) CREUSOT+=(--creusot "$2"); shift 2 ;;
    --creusot-std-build) CREUSOT+=(--creusot-std-build "$2"); shift 2 ;;
    *) sed -n 2,34p "$0" >&2; exit 2 ;;
  esac
done
[ $# -ge 2 ] || { sed -n 2,34p "$0" >&2; exit 2; }
OUT=$1
shift
source "$OUT/versions.env"
IMAGE=${IMAGE:-coar:$F}
[ "$1" = all ] && set -- smoke rq1 rq3 families rq4 creusot tables verdicts
for step in "$@"; do
  declare -F "step_$step" >/dev/null || { echo "unknown step: $step" >&2; exit 2; }
  echo "== $step ($(date +%T))"
  "step_$step"
done
