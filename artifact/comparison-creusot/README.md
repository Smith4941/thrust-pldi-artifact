# Creusot comparison (RQ1, RQ2)

The iterator benchmark of Denis and Jourdan, "Specifying and Verifying Higher-order Rust
Iterators" (TACAS 2023), and the Thrust tests that state the same properties. Table `tab:creusot`
compares the two tools on it, against the current Creusot (creusot-rs/creusot 3620de437, its own
copy of the benchmark, decisions.md D14); Table `tab:creusot-inferred`
verifies the loops of RQ2 with their invariants written and removed; which loops are its rows, and
why, is in status.md (sections "Where it
stands" and "S5").

## Inputs

- `runs.csv`: the inputs of `../scripts/run.py`, one row per Thrust test: the pass test of each
  case, its fail twin (expected `unsat`), and for the loops of RQ2 the pass test without its
  `invariant!` calls (side `inferred`). A test the ui harness ignores on the host is not a row:
  the pass-only `skip_step` (`tests/ui/pass/traits/skip_step.rs`) is ignored, as PCSat gives it
  `unknown` after 60.8 s alone and `timeout` in a parallel run (fptprove develop c3b7d8e4a,
  record).
- `noinv/`: the files of the `inferred` rows, each a pass test of its case with every
  `invariant!(...)` and `partial_invariant!(...)` statement removed and nothing else (the pass row's
  own test; for extend, as `noinv.py` names it, the `for` loop `extend_for.rs` beside it), written by `noinv.py` from fork cc7689ac (the merge of fork PR 297, D121);
  a module a test loads by
  `#[path = ...]` (`auxiliary/common.rs`) is copied beside it under `noinv/`;
  `noinv.py --check <commit>` fails when a file differs from the tests at that commit, and the
  `rq1` step of `../scripts/run-evaluation.sh` runs it at the measured commit. Each keeps
  its test's file name, which is the crate name the queries use. The ghost code an invariant is
  stated over and the calls of laws and lemmas stay: fold and try_fold keep the chain `gs`,
  `gaccs`, `gfs` with its updates and the call of `fold_chain_push` or `try_fold_chain_push`, and
  the files that call `produces_refl` keep the calls. They are written annotations,
  counted as spec lines by `../count-annotations/` (a `ghost!` call, a call of a law or a lemma);
  only the invariant over them is inferred. collect and extend are `for` loops over std.rs's
  `IteratorSpec` as Creusot writes them (`collect_for.rs`, `extend_for.rs`,
  decisions.md D92), and their `inferred` rows set
  `THRUST_FOR_LOOP_HISTORY=1` in the row's env, which gives a `for` loop without a written
  invariant the ghost history `produced` that Creusot gives every `for` loop.
- `cases.csv`: one row per case id of `runs.csv`: the table row it belongs to (`case`), the
  `block` (adapter, client, extra), whether it states Creusot's property in Creusot's form or a
  weaker one (`variant`, decisions.md D1), the `form`, the row label of
  `tab:creusot-inferred` (`s5_label`, empty for a case that is not a row; the rows of block `extra`
  come after the benchmark's), a note, and
  `weaker`: when non-empty, the way the reported form is weaker than Creusot's property
  (spec-audit.md); `tables.py` marks the
  row of `tab:creusot` that reports such a case with a dagger, which the caption explains, and names
  this text in the comment above the row.

## Counts and Creusot's side

- `creusot-port/`: the Creusot side as one file per case (decisions.md D121): the 2022
  artifact's executable code with the specifications of creusot-rs/creusot 3620de437, the case's
  items only, beside the shared `common.rs`, translated against creusot-std at 3620de437.
  `map.rs`, `map_ext.rs`, `skip.rs` and `fuse.rs` carry the hints written in the source that let
  the proof search prove them, as their shipped proofs use manual transformations
  (record). decuple_range, which
  3620de437 does not carry, is the artifact's file ported to 3620de437
  (record); how the others were
  made, and the search on them: record.
- `creusot.csv`: the Creusot columns of `tab:creusot` as recorded: per case, in the order of Fig. 3
  of the Creusot paper, its name there (`name`, the label `tab:creusot` prints, which
  `tab:creusot-inferred`'s `s5_label` follows; `concat_vec` is the artifact's `extend.rs`), the
  lines of code and of specification of its file in `creusot-port/` (`file`), `wall`,
  `tab:creusot`'s Creusot time: the median wall seconds from the translation of that file to the
  proof search's result (`unproved` when goals stay open; decisions.md D123), written from
  `../scripts/creusot-time.py` in the final run and empty until then, and `search`, the median
  wall seconds of the proof search alone on that file from scratch, without certificates, two
  provers at a time on an idle machine, not printed
  (record; decuple_range's from its
  port record). `auto` (the shipped proof uses no manual tactic) and `time` (the median of
  three replays of the shipped proof sessions,
  idle-machine replay) describe
  the 3620de437 file that holds the case (`examples/iterators/`, a client sharing the time of its
  file) and are not printed. `common` has neither, as it has no proof of its own, and decuple_range,
  which 3620de437 does not carry, has no `time`. The lines are counted by the rule of Thrust's side
  (`count-annotations creusot --creusot creusot-port creusot-case-items.csv`, a line holding code
  and spec counting in both, decisions.md D84,
  recount), in the items
  `creusot-case-items.csv` names, so they differ from Fig. 3 of the Creusot paper in version and
  counting; the hints count as specification.
- `case-items.csv`: the items of each Thrust test that form its case part; `annotations.csv`:
  the code and specification lines of that part, written by `../count-annotations/`
  (`creusot`, rules in the header of its `src/creusot.rs`) in the tables step of
  `../scripts/run-evaluation.sh`.

## Not part of the evaluation

`creusot/` holds the 2022 artifact's crate (`creusot-spec/`, which sets no licence) and the same
examples in Thrust's specification form (`thrust-spec/`), with their Why3 sessions; both build
against creusot-rs/creusot b311d543c checked out at `creusot/creusot/` (not tracked);
`creusot-runs.csv` is the replay of those sessions
(record). `thrust/` holds the Thrust
tests as they were on 2026-09-05. `results/` holds the rows of development sweeps, each cited by
its record. The package leaves all of them out.
