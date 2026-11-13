# Replication package

This package reproduces the evaluation of the paper: the verdicts and times behind every table and
figure of Section 6, and the tables and figures themselves. It contains the extended Thrust, the
solver it calls (CoAR, built from one fixed commit), the inputs of each research question, the
scripts that run them and the rows of the reported run.

## Contents

| Path | What it is |
| --- | --- |
| `thrust/` | the extended Thrust at the evaluated commit, with its test suite (`thrust/tests/ui/`) |
| `solver/coar.zip` | the solver with its loader and every library it needs; it runs on x86-64 Linux as it is |
| `solver/Dockerfile` | the solver image the scripts use, built from the unpacked zip |
| `solver/thrust-coar-native-wrapper` | Thrust's solver command for the unpacked zip, without Docker |
| `artifact/` | the inputs of each research question, the scripts and the rows of the reported run; its `README.md` describes them |
| `paper/sections/{tables,figures}/` | the tables and figures as generated from those rows |
| `LICENSES/` | the licences of Thrust; those of the solver's libraries are inside `solver/coar.zip` |

The inputs are listed in one `runs.csv` per research question, with the verdict each must give:
`comparison-creusot/` (RQ1, RQ2), `scalability-evaluation/` and `scalability-input-families/`
(RQ3) and `rustc-coroutine-layout/` (RQ4). The reported run is `artifact/results/<run>/`;
its `versions.env` names the Thrust commit `T` and the solver's commit `F`.

## Requirements

- x86-64 Linux, Docker, Python 3, and the Rust toolchain through `rustup` (Thrust pins its
  nightly in `thrust/rust-toolchain.toml`).
- The reported run used 32 hardware threads and 94 GiB of RAM with four solver containers at
  once, each with 4 CPUs and 8 GB (RQ1: two at once; RQ4: four at once, the parts of one input).

## Setting up

Run every command from the root of this package; `<F>` is `F` of `versions.env`.

```bash
(cd solver && unzip -q coar.zip && docker build -t coar:<F> .)
(cd thrust && cargo build --release)
```

## Getting started (target: 30 minutes)

1. Set up as above: unpack and build the solver image (a few minutes) and build Thrust with
   `cargo build --release` (a build with the crates and toolchain already downloaded took 1 minute on 32
   threads; a fresh one also downloads them, which was not measured).
2. Create a run directory with the reported run's commits and run the quick run, which verifies
   six inputs and their fail twins in 16 runs, each solver call capped at 60 s (four containers at
   once, 4 CPUs and 8 GB each), and prints each verdict next to the expected one. In the reported
   run every one of them answered as expected (`results/<run>/rq1.csv`, `rq3.csv`, `rq4.csv`): the
   RQ1 and RQ3 inputs within 3 s, `lemmas` in 36 s and its twin, whose 60 parts the quick run solves
   one at a time, in about 72 s of solver time:

```bash
mkdir -p quick && cp artifact/results/<run>/versions.env quick/
artifact/reproduce.sh -n quick quick   # what it runs, without running it
artifact/reproduce.sh quick quick
```

   Each line is expected to end in `ok` (`sat` for a pass test, `unsat` for its fail twin); the
   run fails otherwise.
   To check one input alone:

```bash
artifact/scripts/run.py --thrust-commit <T> --fptprove-commit <F> --image coar:<F> \
    --only range artifact/comparison-creusot/runs.csv /tmp/range.csv
```

   Replace `--image coar:<F>` by `--solver $PWD/solver/thrust-coar-native-wrapper` to run without
   Docker. Before it starts, `run.py` checks what identifies the run and stops on a mismatch (the checks
   are listed in its docstring).
3. Regenerate every table and figure from the reported rows, which needs no solver:

```bash
artifact/reproduce.sh tables
```

## The whole evaluation

```bash
mkdir -p OUT && cp artifact/results/<run>/versions.env OUT/
artifact/reproduce.sh -n full OUT     # the commands and every run
artifact/reproduce.sh full OUT
```

The steps run four solver containers at once (`--jobs N`; with fewer than 16 hardware threads or
64 GB of RAM, lower it), RQ1 two whatever `--jobs` is and write their rows under `OUT`; the last step, `tables`, counts the lines
and annotations of `thrust/` and of the RQ3 programs (the crate `artifact/count-annotations/`)
into the CSVs the tables read and writes `paper/sections/tables/` and `figures/` from them
and the rows. Which table or figure each step writes, with its inputs and time, is the table in
`artifact/README.md`, Reproducing the paper.

A time is the wall time of the Thrust command, from the input to the verdict, its solver calls
with their container start-up included (D123 of the authors' decision record), taken on an
otherwise idle machine; another machine reproduces the verdicts, and the shape of the times
(ledger rows C7 and C8), not the seconds. The reported run took 47 minutes, 8 of them in the
whole ui suite, which it ran as its first step; `full` now runs the `smoke` step instead, ten ui
tests that check the build and the solver image and stop the run on a failure. The whole suite in
`thrust/tests/ui/` is a development check.

To check the verdicts within minutes, run the steps with the verdicts the reported run kept
(`artifact/results/<run>/verdicts/`, D124): Thrust then calls the solver only for a query
the cache lacks. The times of such a run are not measurements, and `tables.py` refuses its rows.
Without `--reuse-verdicts`, every run starts from an empty cache of its own, and its times are
measurements.

```bash
artifact/scripts/run-evaluation.sh --reuse-verdicts artifact/results/<run>/verdicts OUT rq1 rq3 families rq4
```

Not reproduced: the Creusot columns of Table `tab:creusot`, recorded in
`artifact/comparison-creusot/creusot.csv` (Creusot 3620de437, from the translation of each
case's file to the proof search's result; `artifact/scripts/creusot-time.py` times it again
with Creusot installed), and `sat` re-verification (`run.py --check-solution`). The ports of the
benchmark to Verus, Flux and Prusti are not part of this package.

## Licences

Thrust is available under MIT or Apache-2.0 (`LICENSES/thrust-LICENSE-*`). The code under
`artifact/rustc-coroutine-layout/` is from rustc, under MIT or Apache-2.0 (its own
`LICENSE-*`). The solver's libraries are under their own licences: `coar/THIRD_PARTY_LICENSE/` of
`solver/coar.zip` holds them, and its `LIBRARIES` names the file of each library; the image
carries those of its system under `/usr/share/doc`.
