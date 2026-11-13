# Coroutine layout case study (RQ4)

A panic-safety verification of `coroutine::layout()` from the Rust compiler with Thrust. The
extraction is the smallest part of rustc's code that compiles with `layout()`: a function
nothing needs in order to compile is dropped, and the items kept stay as rustc writes them
(rewrites.md E0, E3, E7; decisions.md D74, D86). Every line that remains counts as
code, also that of a function kept only so that the code compiles (a trait impl a type must
have, or a function only unverified code calls), which no root's run checks: such a function is
`compile-only` in `functions.csv`, verified by none and trusted by none, since nothing assumes a
contract of it, and counted in `stages.csv` and `rows.csv` as `compile_only_functions` with its
lines `compile_only_loc`, which `loc` includes. The
original source is rust-lang/rust at 89a99936d9e76a50e8df622e7242190841fd871b, licensed under
MIT OR Apache-2.0 (LICENSE-MIT, LICENSE-APACHE). The relevant definitions were extracted into one
file, `src/main.rs`, from which the fork's single-file stage files were made; since fork c2a57182
the verified code is rustc's module layout under `tests/rustc_coroutine/` with one stage root per
stage under `tests/ui/{pass,fail}/rustc_coroutine/`
(evaluation README). This directory records
how the verified files were reached from the original, so that every change can be read, and the
verified files rebuilt from the compiler's sources and checked.

## Layout

- `upstream/`: the nine files of rust-lang/rust the target was extracted from, copied at the
  commit above; `https://raw.githubusercontent.com/rust-lang/rust/<commit>/<path>` serves each
  original for comparison.
- `src/main.rs`: the extraction, one file with `// //== <upstream file>` markers at the start
  of each part. `Cargo.toml` builds it as a crate with the features of `rustc_abi`.
- `rewrites.md`: every change made to the code, from the extraction to the stage files, with
  its commit, reason, equivalence argument and patch.
- `provenance/`: the line selections and patches that rebuild the verified files (below).
  `summary.csv` counts, per patch, the lines kept from the source, added and changed.
- `runs.csv`: the inputs of `../scripts/run.py`, the pass file of each root and its fail
  twin where it has one; stage 7 runs with 16 GB.
- `parts.csv`: the root files (`root` rows: the stage each serves, the run ids, and the row of
  Table `tab:coroutine` its `THRUST_VERIFY_ONLY` selects, one row per root, decisions.md D82;
  `lemmas.rs`, the root of `thrust/lemmas.rs`, is a second root of the `layout` row), which stage
  each part of the module tree under `tests/rustc_coroutine/` belongs to, by the line a section
  starts at, or by a function's first line, the `unchecked` functions whose body no root's run
  checks and the `compile-only` functions, both by `Type::name`, and the `row` rows that name the
  module rows (the four files of `rustc_index` one row, `rustc_hashes` and the lib.rs of
  `rustc_abi` one each). An unchecked function has a contract the other roots assume: no root that
  runs selects it (`univariant_biased`, whose body only the draft root `univariant_biased.rs`
  checks), or nothing its root analyzes calls it (`IndexSlice::index`). A `row` row may name a
  function row of the same stage instead of a module (the counter stops on a name that is
  neither, or on a function row of another stage): layout/simple.rs, which the tree needs for
  `LayoutData::scalar_pair`, counts in `univariant_biased`'s row (decisions.md D86). A `helpers` row (`file,helpers,from,name`) puts the lines of its file from the first line that begins with `from` on, outside every function row, into the function row `name`, as decisions.md D82 counts the helpers a rewrite added for a function (`univariant_biased`'s: layout.rs's lemmas of the field placement and its helpers from `univariant_biased_post` on, and thrust/iter.rs's `iter_enumerated_trusted`, whose section is stage 6). A function without a
  contract that no run checks, since no function the analysis reaches calls it and Thrust leaves
  its clauses out of the query, is kept only if the code needs it to compile, as `compile-only`
  (above; decisions.md D74). `../scripts/stage-report-check.py`, run on the reports of the roots
  (`THRUST_REPORT=functions`), checks both lists and lists the functions no root selects. A
  section of part `std` is a stand-in for std that the case study writes and is counted nowhere
  (D83): `thrust/iter.rs` and the own iterators `IdxRange`, `IterEnumerated` and
  `WordIter`, which the `upstream/` copies lack; the fallback wrappers at the end of
  `thrust/iter.rs` (from `// Fallback: `Map::new``), whose `assume!`s stage 7's `layout` relies on,
  are a section of stage 7; `thrust/lemmas.rs` and `thrust/counts.rs` are sections of stage 7,
  whose `layout` their lemmas serve; layout/coroutine.rs is stage 5 from its start to the lemmas that
  `layout()` uses, stage 7 from them through `bit_iter_yields_below`, stage 5 from `// The
  specification of stage 5` through `set_assignment`, and stage 7 from there to its end;
  `thrust/models.rs` is a section of part `model`, each of its `Model`
  impls counting with the type it models. A file `parts.csv` does not name
  (`thrust/std_specs.rs`) counts nowhere.
- `stages.csv`: the counts per stage (lines of code, functions under
  verification and those with a type parameter under a trait bound, loops, contract clauses of
  the verified functions, predicates, trusted functions with the ignored ones among them, invariants
  written and inferred, unchecked functions, functions under assumptions (a function that would
  be verified but whose body holds an `assume!`, D79), and the `#[thrust::opaque]`,
  `#[thrust::reveal]` and `assume!` annotations (D78, D79), the added helpers marked trusted, which are annotations
  and not trusted functions (`trusted_helpers`, D79), the spec lines, counted as RQ1's
  (`spec_lines`, D83), the analyzed functions whose contract is inferred
  (`contracts_inferred`, D83), and the functions whose bodies a run checks against a contract,
  the verified and the assumed ones (`functions_checked`, of which `contracts_inferred` is a part), and the generic ones of these two
  (`generic_functions_checked`, `generic_contracts_inferred`), whose contract Thrust infers per instance,
  and the added functions other than lemmas and law proofs, the functions a rewrite extracted from
  rustc's functions (`functions_extracted`));
  `functions.csv`: the functions they count, one row per function with its stage, status, lines,
  loops and contract, and a `trusted` flag (true for a trusted function and for an added helper marked trusted, whose contract the review checks against rustc's code); a verified-by-attributes function the `upstream/` copy lacks (a law proof, a
  lemma, a function a rewrite adds) has status `added` and is counted as no function; its lines are
  spec lines (decisions.md D74, D83).
  `rows.csv`: the same counts per row of Table `tab:coroutine` (decisions.md D82): each rustc
  function of at least 40 lines of code, and the rest of each source file that holds another
  rustc function (with that file's code and predicates of every stage), the files of `rustc_index`
  forming one row (the `row` rows of `parts.csv`), with the root
  whose run checks the row's functions (`files`), the row's unchecked functions
  (`unchecked`), and its functions kept only so that the code compiles (`compile_only`).
  `../count-annotations/` (`coroutine`, rules in the header of its
  `src/coroutine.rs`) writes the three from the Thrust tree in the tables step of
  `../scripts/run-evaluation.sh`; the committed ones are of fork `main` ab598199, the `thrust/` pin, written by the tables step of the final run (record). `../scripts/tables.py`
  adds to each row the verdict and the time of its root's run from a run's `rq4.csv`. Stage 1
  has `hashes.rs` and `values.rs`, stages 2 to 4 share `index.rs`, and stages 5 to 7 count only
  the code new to them.
- `review_order.py`: a bottom-up review checklist for stages 1 to 4, generated from a checkout of
  the fork at review time (`python3 artifact/rustc-coroutine-layout/review_order.py
  <fork checkout> [--stage N] [--functions-csv FILE] [--list]`, output on stdout, not committed).
  It counts the functions from that checkout with `../count-annotations/` (not from
  `functions.csv`, which is the count of the commit it was last regenerated at), and lists each
  with its location, status, contract as written or `#[thrust::callable]`, panic sites and call
  sites (linked by `Type::name`), in layers from functions that call no other listed function
  outward, and gives each trusted function an item to check its contract against the upstream
  copy. `--list` prints the selection only. The scan is static and approximate; the header of the
  script lists the kinds of panic site it detects and its limits.

## From rustc to the verified files

The patches rebuild the single-file stage files of fork d635f6b9, the tree of the final run of
2026-10-04. They were written and checked by
`artifact/scripts/coroutine_provenance.py` of thrust-research 7767cd1 (`write`, `check`,
`reproduce --fetch`), which a change to those files reruns from that commit. `rewrites.md` gives
the module tree's state in a "Module layout" paragraph of an entry whose main bullets describe
those patches, and in the main bullets themselves where the entry is stated for the module tree
(E2, S1 to S3, S8; E2 and S8 also say what the patches of d635f6b9 show).

The code passes through three steps. Each is recorded as a line selection, the lines of the
source that the result keeps in order (a manifest of line ranges, and the selected lines as a
file for reading), plus a patch from the selection to the result. Dropping lines, which is what
extraction means, stays out of the patch; every changed line is in it. A dropped line inside a
kept item (a statement of a function body, a header line) is therefore in no patch either;
`rewrites.md` lists those separately.

Since the module tree, one more drop is recorded as a patch against the fork tree:
`provenance/module-tree/E7-unneeded-items.patch` (rewrites.md E7) applies with `patch -p1` to
fork c244643a and gives fork e9c1a02c's `tests/rustc_coroutine/compiler/`, that of 457fc1dc.

1. Extraction: `upstream/` to `src/main.rs`, done with `cargo-onefile` and by hand before the
   Thrust work (rewrites.md, E0). `provenance/extraction/main.extract.csv` names, for each
   `// //==` section of `main.rs`, the upstream file and the lines kept from it;
   `main.extract.rs` is that selection and `provenance/extraction.patch` turns it into
   `main.rs`. The upstream files are taken as they are, so the patch also shows where the
   extraction's default `rustfmt` formatting differs from rustc's.
2. Rewrites: `src/main.rs` to `target.rs` of the Thrust tree, ten fork commits, each one patch
   under `provenance/rewrites/` with its commit message (`00-ui-test-header.patch` adds the
   ui_test line the first fork commit prepended). Applied in order with `patch -p1` to a copy
   of `src/main.rs`, they give `target.rs` byte for byte.
3. Stages: `target.rs` to each stage file. `provenance/stages/<stage>.extract.csv` names, per
   section of the stage file, the lines of `target.rs` it keeps (a section whose marker names
   an upstream file is matched against the same section of `target.rs`, any other against the
   whole file); `<stage>.extract.rs` is that selection, `<stage>.patch` turns it into the stage
   file, and `<stage>.fail.patch` turns the pass file into its fail twin.

Each step can be replayed with `patch` alone, for example stage 1 from its selection:

```bash
cd artifact/rustc-coroutine-layout
patch -o /tmp/values.rs provenance/stages/values.extract.rs provenance/stages/values.patch
diff /tmp/values.rs ../../../thrust/tests/ui/pass/rustc_coroutine/values.rs               # empty
```
