#!/bin/bash
# usage: artifact/scripts/rq4-function-reports.sh [-n] THRUST_TREE IMAGE OUTDIR
#
# Writes the THRUST_REPORT=functions report of each RQ4 pass root that
# rustc-coroutine-layout/runs.csv lists, for stage-report-check.py: OUTDIR/<root>.jsonl, with the
# root's queries in OUTDIR/<root>/. Each root is compiled as its ui test compiles it (its header's
# environment and solver, and the `env` of its runs.csv row) with THRUST_TREE's debug thrust-rustc
# and COAR_IMAGE=IMAGE, one root at a time and without THRUST_CACHE_DIR, at the sweep's limit of
# 300 s per query or part (THRUST_SOLVER_TIMEOUT_SECS, over the header's). Every pass root runs,
# also a draft such as layout.rs, which does not split and runs at the memory its runs.csv row
# gives (16 GB). THRUST_TREE must be a git checkout without changes; its commit goes to
# OUTDIR/commit, and `cargo build --bins` runs in it first, so that the thrust-rustc used is the
# commit's. A root's exit status, wall time and load go to OUTDIR/<root>/time; a non-zero status
# is also a root that does not verify, whose report stands. It stops when OUTDIR already holds a
# root's report or folder, when a root's report has no `verdict` line (Thrust stopped before it
# reported), and when THRUST_SPLIT_GOALS (added to a root that sets THRUST_SPLIT=1 alone, whose
# marked functions alone split by goal in the sweep, D125) fell back to the class split (Thrust's note "the parts
# are the classes"), whose parts stage-report-check.py cannot read, after removing that report.
# -n prints the commit and the roots and what it would run, and runs nothing.
set -eu
dry=0
if [ "${1:-}" = "-n" ]; then dry=1; shift; fi
[ $# -eq 3 ] || { sed -n 2,18p "$0" >&2; exit 2; }
tree=$(cd "$1" && pwd); image=$2; out=$3
art=$(cd "$(dirname "$0")/.." && pwd)
commit=$(git -C "$tree" rev-parse HEAD)
if [ -n "$(git -C "$tree" status --porcelain --untracked-files=no)" ]; then
  echo "$tree has uncommitted changes" >&2
  exit 1
fi
echo "Thrust $commit"
if [ "$dry" -eq 0 ]; then
  mkdir -p "$out"
  out=$(cd "$out" && pwd)
  [ ! -e "$out/commit" ] || { echo "$out already holds reports" >&2; exit 1; }
  (cd "$tree" && cargo build -q --bins)
  echo "$commit" > "$out/commit"
fi

roots() {  # the pass rows of runs.csv: file and env
  awk -F, 'NR > 1 && $3 == "pass" { print $5, $6 }' "$art/rustc-coroutine-layout/runs.csv"
}

report() {  # FILE [ENV]: compiles one root into OUTDIR
  local f=$1 extra=${2:-} name dir flags ed envs lib rc=0 l0 s
  name=$(basename "$f")
  dir=$out/$name
  flags=$(grep -m1 '^//@compile-flags:' "$tree/$f" | sed 's#^//@compile-flags:##')
  ed=$(grep -m1 '^//@edition:' "$tree/$f" | sed 's#^//@edition:##' | tr -d ' ')
  envs=$(grep '^//@rustc-env:' "$tree/$f" | sed 's#^//@rustc-env:##' | tr '\n' ' ')
  # A root that splits only its marked functions' goals (THRUST_SPLIT=1 alone, D125) is reported
  # with every goal split: the class split gives a body the parts of its class even when the
  # pruning leaves none of its clauses there (fork issue #173), and both splits decide the same
  # constraints (D78).
  if [[ " $envs " == *" THRUST_SPLIT=1 "* && " $envs " != *THRUST_SPLIT_GOALS=* ]]; then
    envs="$envs THRUST_SPLIT_GOALS=1"
  fi
  if [ "$dry" -eq 1 ]; then
    echo "$name: $envs $extra THRUST_SOLVER_TIMEOUT_SECS=300 THRUST_REPORT=functions -> $out/$name.jsonl"
    return
  fi
  if [ -e "$dir" ] || [ -e "$out/$name.jsonl" ]; then
    echo "$out already holds a report of $name" >&2
    exit 1
  fi
  mkdir -p "$dir"
  lib=$(rustc --print sysroot)/lib
  l0=$(cut -d" " -f1 /proc/loadavg)
  s=$(date +%s.%N)
  (cd "$tree" && env -i PATH="$PATH" HOME="$HOME" LD_LIBRARY_PATH="$lib" COAR_IMAGE="$image" $envs $extra THRUST_SOLVER_TIMEOUT_SECS=300 \
    THRUST_OUTPUT_DIR="$dir" THRUST_REPORT=functions THRUST_REPORT_FORMAT=json \
    THRUST_REPORT_FILE="$out/$name.jsonl" \
    "$tree/target/debug/thrust-rustc" --error-format=json --out-dir "$dir" "$f" $flags --edition "${ed:-2021}" \
    > "$dir/stdout" 2> "$dir/stderr") || rc=$?
  echo "$f rc=$rc wall=$(echo "$(date +%s.%N) - $s" | bc) load=$l0..$(cut -d" " -f1 /proc/loadavg) image=$image" > "$dir/time"
  if ! grep -q '"kind":"verdict"' "$out/$name.jsonl" 2>/dev/null; then
    rm -f "$out/$name.jsonl"
    echo "$name: no verdict in its report (rc=$rc, $dir/stderr)" >&2
    exit 1
  fi
  if grep -q "the parts are the classes" "$dir/stderr"; then
    rm -f "$out/$name.jsonl"
    echo "$name: THRUST_SPLIT_GOALS fell back to the class split ($dir/stderr)" >&2
    exit 1
  fi
}

roots | while read -r f extra; do
  report "$f" "$extra"
done
