# Rewrites of the coroutine layout target

Every change made to the rustc code on its way to the verified files, with the commit that made
it, what it does, why, why it preserves the behaviour the proof is about (panic safety), and the
patch that shows it. How the patches are produced and checked is in [README.md](README.md).

The code passes through three steps. Step 1 was done with `cargo-onefile` and by hand before
the Thrust work started (E0); steps 2 and 3 are fork commits, one rewrite per commit where
possible.

| step | from | to | patches |
| --- | --- | --- | --- |
| 1 extraction | rustc 89a99936, nine files under [upstream/](upstream/) | [src/main.rs](src/main.rs) | [provenance/extraction.patch](provenance/extraction.patch) |
| 2 rewrites | `src/main.rs` | `tests/ui/pass/rustc_coroutine/target.rs` of the fork | [provenance/rewrites/](provenance/rewrites/) |
| 3 stages | `target.rs` | `tests/ui/pass/rustc_coroutine/<stage>.rs` and the fail twins | [provenance/stages/](provenance/stages/) |

Commits are of the Thrust fork unless marked otherwise. Annotations (contracts, predicates,
`Model` declarations, trusted markers, loop invariants) are not rewrites and are not listed;
they are the `+` lines of the step 3 patches that begin with `#[thrust`, a predicate or `Model`
block, or `thrust_macros::invariant!`.

## Index

| id | title | step | commits | patch |
| --- | --- | --- | --- | --- |
| E0 | How the extraction was made | 1 | thrust-research da9b1eb | extraction.patch |
| E1 | One file, one module | 1 | thrust-research da9b1eb | extraction.patch |
| E2 | Feature-gated attributes commented out (none in the module tree) | 1 | thrust-research da9b1eb | extraction.patch |
| E3 | `Debug` removed; `expect` replaced by a local `unwrap_without_debug` | 1 | thrust-research da9b1eb | extraction.patch |
| E4 | Logging macros commented out | 1 | thrust-research da9b1eb | extraction.patch |
| E5 | Assertion messages commented out | 1 | thrust-research da9b1eb | extraction.patch |
| E6 | The `randomize` branch compiled out | 1, 2 | thrust-research da9b1eb, ccd76e4 | 01-standalone-crate.patch |
| E7 | Items nothing needs to compile dropped from the module tree | 1 | b6176800, 457fc1dc (merged as e9c1a02c) | module-tree/E7-unneeded-items.patch |
| R0 | ui_test header, edition and feature gate | 2 | 9208ae6, ccd76e4 | 00-ui-test-header.patch, 01-standalone-crate.patch |
| R1 | `bitflags!` expanded by hand | 2 | ccd76e4 | 01-standalone-crate.patch |
| R2 | `WordIter` for `slice::Iter` in `BitIter` | 2 | d3157ca | 02-word-iter.patch |
| R3 | `IdxRange` for `(a..b).map(I::new)` | 2 | a20970a | 03-idx-range.patch |
| R4 | `IterEnumerated` for `IndexSlice::iter_enumerated`; `SliceIter` undone | 2 | d5e8770; `SliceIter` undone in fork PR #325 (merged as 3639e4f5, record) | 04-slice-iter.patch |
| R5 | `for` loops written as `while let` (none in the module tree) | 2 | c9f31c3, a20970a | 05-for-to-while-let.patch, 03-idx-range.patch |
| R6 | Panic messages dropped | 2, 3 | 63d8ae7, 18bd046 | 06-panic-messages.patch, stages/values.patch |
| R7 | `?` written as `match` | 2 | a57c283 | 07-question-mark.patch |
| S1 | Derives changed in the module tree | 3 | 18bd046, abdb69b, 9b618cb, 85d0dac, b2d3d28 | stages/*.patch |
| S2 | A `PartialEq` for `TargetDataLayout` with an unanalysed body (none since fork main 7f5be643) | 3 | 6852457 | stages/values.patch |
| S3 | `AbiAlign::min` and `max` (none in the module tree) | 3 | 18bd046 | stages/values.patch |
| S4 | Items copied between stage files | 3 | 9b618cb, 85d0dac, b2d3d28 | stages/*.patch |
| S5 | Test drivers and stubs | 3 | 18bd046, abdb69b, 9b618cb, 85d0dac | stages/*.patch |
| S6 | Draft-only changes (stages 5 to 7) | 3 | b2d3d28, 932562c, db249cf, 09b4c232, 6c135a0f | stages/{eligibility,univariant,layout}.patch |
| S7 | Compile settings of the stage files | 3 | 18bd046, abdb69b, 9b618cb, 85d0dac, b2d3d28 | stages/*.patch |
| S8 | `IndexSlice` methods restated on `IndexVec` (only `at`, in two roots, in the module tree) | 3 | 9b618cb | stages/indexvec.patch |
| S9 | `impl TryInto<u64>` parameters of `Size::from_bits` and `from_bytes` named | 3 | eba79e9 | stages/values.patch |
| S10 | `F: Deref<Target = &LayoutData>` bounded by `LayoutRef` | 3 | 37a4e22d | stages/{univariant,layout}.patch |
| S11 | Ghost fields in `BitMatrix` and `BitIter` | 3 | b6e01a5e, cf495c7d, bd4fd8fe, b5309e73 | stages/{bitset,eligibility,layout}.patch, stages/bitset.fail.patch |
| R8 | Local `Map`, `Filter`, `all`, `extend` and `collect` for std's in `layout()` | 3 | 7e173880, a68a1e04 | stages/layout.patch |
| R9 | Closure contracts at std's `find` calls (no local `find`) | 3 | ba988afa | none: the step 3 patches are of d635f6b9 and still show the local `find` |
| R10 | Writes to `assignments` in two loops of `coroutine_saved_local_eligibility` as helper functions | 3 | fork PR #162 (unmerged) | none: the step 3 patches are of d635f6b9 |
| R11 | The split of the prefix's memory order in `layout()` as `split_in_memory_order` | 3 | fork 5aa7ffd7 (branch `rq4-stage7-verify2`, unmerged) | none: the step 3 patches are of d635f6b9 |
| R12 | The destructuring and combination of a variant's offsets in `layout()` as `combine_offsets` | 3 | fork 5aa7ffd7, 25a238f5, 413dd75a, ed1bfcf7 (unmerged) | none |
| R13 | Arguments in `layout()` bound to names | 3 | fork 5aa7ffd7, 1d98c59f, 413dd75a (unmerged) | none |
| R14 | The collection of a variant's field layouts in `layout()` as `variant_field_layouts` | 3 | fork f03080bb, 413dd75a, ed1bfcf7 (unmerged) | none |
| R15 | The extension of the prefix's fields in `layout()` as `promote_locals` | 3 | fork 8cd3285f, 413dd75a (unmerged) | none |
| R16 | The split of the prefix's fields in `layout()` as `split_prefix_fields` | 3 | fork 8cd3285f (unmerged) | none |
| R17 | The body of `layout()`'s per-variant closure as `variant_layout` | 3 | fork 23932ef2, 2c3aaba1, 2aeffe7d (unmerged) | none |
| R18 | The body of `univariant_biased`'s field loop as `place_field` | 3 | fork 1fd3e0c8, fabec746 (unmerged) | none |
| R19 | `univariant_biased`'s field loop as `place_fields` | 3 | fork 6092cb27, bd8029fa (unmerged) | none |
| R20 | The data layout as a parameter of `alignment_group_key` | 3 | fork 6a596429 (unmerged) | none |
| R21 | `IndexVec::from_raw` in `scalar_pair` | 3 | fork PR #221 (merged as 4c323d65), #226 (merged as a40add81), #263 (merged as c8544c5a; verified on fork `main` c8544c5a, record) | none |
| F1 | Fail twins | 3 | 18bd046, abdb69b, 9b618cb, 85d0dac, 7216419 | stages/*.fail.patch |

Not rewrites, kept in step 2 for completeness: 08-trusted-markers.patch (3aa2440,
`#[thrust::trusted]` with `#[thrust::callable]` on bit-level and std-backed helpers; its commit
message speaks of `requires(true)` / `ensures(true)`, which the patch does not add),
09-model-declarations.patch (f153a8d, one `impl Model` per type), 10-header-comment.patch
(b05b0e4, the header comment of `target.rs`).

A line selection records dropped lines, not where they were: a statement dropped from inside a
kept function, or a header line of `target.rs` left out of a stage file, does not appear in any
patch. S6 and S7 list the changes of that kind, found by comparing each function body and file
header of the stage files with `target.rs`.

## Step 1: extraction

The nine upstream files hold 7049 lines; `src/main.rs` keeps 1694 of them as they are and
changes 102, as [provenance/summary.csv](provenance/summary.csv) counts; part of the changed
and added lines is the formatting difference of E1. The extraction was made
in June 2026 with `cargo-onefile` and by hand (E0); the patch is generated afterwards and is the
record of what it changed beyond dropping items. Compared token by token, with comments and
whitespace ignored, each `// //==` section of `main.rs` differs from its upstream file only in
dropped items, `use`-list entries, visibility and paths, and the changes E1 to E5 name; no
statement of a kept function is dropped.

### E0 How the extraction was made

- What: `cargo-onefile` (`cargo onefile -d`, run in `compiler/rustc_abi`) merged the crate and
  its path dependencies (`rustc_index`, `rustc_hashes`), without crates.io dependencies, into
  one file: the source files concatenated as they are, each behind a separator line naming its
  path relative to the crate (`// //== ./../rustc_hashes/src/lib.rs`, the `// //==` markers of
  `main.rs`), in the order `rustc_hashes`, then the files of `rustc_index` and of `rustc_abi`
  alphabetically by path. Rerun today (version 0.2.1) the output has 9923 lines in 21 files;
  `main.rs` keeps 9 of the files in that order. The tool rejects the crates' `edition = "2024"`
  manifests, so the rerun needed the edition set to 2021 for the run only, which does not
  change the concatenation. From the merged file the items needed for `coroutine::layout()` to
  build were kept and the rest dropped, by repeating: remove what looks unneeded, compile, put
  back what the errors name; `cargo fmt` with the default configuration formatted the result.
  No tool gave the dependencies of one function: `cargo-modules` was tried and counted every
  method of a type as a dependency while missing `coroutine_saved_local_eligibility`.
- Record: the tool changes no line of the sources, so a reader needs neither it nor its output:
  every difference between `upstream/` and `main.rs` is either a dropped line (absent from
  `main.extract.csv`) or a line of `extraction.patch`. Since the tool's output is the upstream
  files concatenated, the step is recorded
  against the upstream files themselves, as one line selection (`main.extract.csv`) and one
  patch (`extraction.patch`) covering the manual work and the formatting together, rather than
  as a tool run plus a patch.
- Why: Thrust verifies one file, and the external crates (`bitflags`, `tracing`, and `rand` /
  `rand_xoshiro` behind the `randomize` default feature) were left out, which E3 to E6 and R1
  then deal with.
- Check: the rustc commit, 89a99936, was recovered after the checkout used had been deleted,
  by confirming that the `cargo-onefile` output at that commit is identical to the file the
  extraction started from. The tracked `upstream/` copies are of that commit, and
  `extraction.patch` shows everything that differs between them and `main.rs`.

### E1 One file, one module

- What: the items `layout()` needs are copied from nine files into one, in the order of the
  `// //== <file>` markers; `pub(super)` becomes `pub`, `super::` and crate paths are dropped,
  `use` lists are trimmed to the kept items, doc comments and most comments are dropped, and
  the file is formatted with default `rustfmt`, where rustc formats with its own
  `rustfmt.toml` (wider one-line heuristics), so a struct literal or a signature rustc writes
  on one line appears in the patch as several. Two match arms in `univariant_biased` lose their
  block braces: the `(Some((i, a)), Some((j, b)), None)` arm around its single `match`
  expression, and `None => { align }` in the `repr(transparent)` part, whose other content was a
  comment. A `fn main() {}` makes the file a binary crate.
- Why: Thrust verifies one file (single-crate, single-module analysis).
- Equivalence: modules, visibility, comments and formatting do not change what the code
  computes; a block whose only statement is an expression is that expression.

### E2 Feature-gated attributes commented out

- No change remains under this entry in the module tree. Since fork b35a1226 (fork PR
  #243), `rustc_abi/src/lib.rs` and
  `rustc_index/src/bit_set.rs` carry every `#[cfg_attr(feature = "nightly", ...)]` their
  upstream copies have on the kept items, and `rustc_abi`'s three crate attributes
  `#![cfg_attr(feature = "nightly", ...)]`. The roots are compiled without cargo features, so
  the attributes expand to nothing and the derive macros of `rustc_macros` are not needed.
  `extraction.patch` and the step 3 patches, which are of d635f6b9, show them commented out. The
  number is kept so that the later entries keep theirs.

### E3 `Debug` removed; `expect` replaced by a local `unwrap_without_debug`

- What: `Debug` is removed from most derives (`/*Debug,*/`), from the bound of `F` in
  `univariant` and `univariant_biased` (`/*fmt::Debug +*/`), and `impl Debug for ReprFlags` is
  dropped. The two `.expect("...")` calls in `univariant` become `.unwrap_without_debug()`, a
  method of a local trait `Unwrap` defined at the end of `main.rs`:

  ```rust
  x.expect("alt layout should always work")   ->   x.unwrap_without_debug()
  // where: fn unwrap_without_debug(self) -> T { let Ok(item) = self else { panic!() }; item }
  ```
- Why: `Result::expect` requires `E: Debug`, and `Debug` is used only by the logging macros and
  the panic messages, which go with it (E4, E5, R6). The second call is `Option::expect` on
  `alt_layout.largest_niche`, which needs no `Debug`; replacing it drops its panic message, as
  R6 does.
- Equivalence: `expect` and `unwrap_without_debug` both return the `Ok` / `Some` payload and
  panic otherwise; only the panic payload and the reported location (`expect` is
  `#[track_caller]`) differ. A removed `Debug` impl is unused by the remaining code (it compiles).
- Module layout (fork main 36f400c0): `Debug` is back: the derives and the `F: fmt::Debug`
  bounds are rustc's (Thrust does not analyse a derived `Debug::fmt`), `ReprFlags` derives it in
  place of what `bitflags!` generates (R1), and the hand-written impls of `Endian`, `Size`,
  `Align`, `WrappingRange`, `Hash64` and `IndexVec` stay with `fmt` marked
  `#[thrust::ignored]` (`IndexSlice`'s, which nothing needs, E7 drops), since Thrust cannot type a `Formatter` write. The two calls are rustc's `expect` and there is no
  `Unwrap` trait (5374a386): `Option::expect` and `Result::expect` have `unwrap`'s contracts, and
  the payload is typed (case-study-gaps.md, gap 8).

### E4 Logging macros commented out

- What: every `trace!` and `debug!` call (the `tracing` crate) is commented out.
- Why: an external crate; its macros expand to code Thrust cannot type.
- Equivalence: the macros do not contribute to the returned layout; the panic-safety claim is
  about the remaining code.

### E5 Assertion messages commented out

- What: the message arguments of the two `debug_assert_eq!` in
  `IndexSlice::invert_bijective_mapping` are commented out; the `/* signed = */` comment on an
  argument is dropped. The stage files compile without debug assertions (S7), so these two
  assertions are not in the verified code at all.
- Why: the messages format `self` with `Debug` (E3).
- Equivalence: the asserted condition is unchanged; the message is the panic payload.
- Module layout (fork main 36f400c0): `invert_bijective_mapping` is verified against a contract
  and its two `debug_assert_eq!` calls are dropped with their messages, as in the stage files
  (S6).

### E6 The `randomize` branch compiled out

- What: `univariant_biased` keeps its `#[cfg(feature = "randomize")]` block, but the Thrust test
  is compiled without cargo features, so the branch is compiled out. rustc itself builds
  `rustc_abi` with its default features `nightly` and `randomize` (`rustc_middle` depends on it
  with default features).
- Why: the branch uses the `rand` crates.
- Equivalence: it is the same source under a different, valid `cfg`. For `layout()` the branch
  is not taken under either `cfg`: both calls pass `ReprOptions::default()`, whose flags are
  empty, so `can_randomize_type_layout()` is false. `univariant_biased` is trusted in the stage
  plan anyway (the evaluation README, stage 6).

### E7 Items nothing needs to compile dropped from the module tree

- What: the extraction is the smallest part of rustc's code that compiles with `layout()`
  (decisions.md D74, D86). Fork b6176800 (fork PR #175,
  merged as 9b5c0741 with the same module tree) drops from the module tree four rustc functions that no root's run checks and that
  the tree compiles without: `Hash64::as_u64`, `VariantLayout::has_fields`, and `IndexVec`'s
  `Borrow` and `BorrowMut` impls, with their import. Fork 457fc1dc (fork PR
  #181, merged into fork `main` as e9c1a02c) drops the three `#[thrust::ignored]`
  functions the tree compiles without: `IndexVec`'s `Extend` impl (`layout()` calls
  `extend_from`, R8) and `FromIterator` impl (the code collects through `collect_index_vec`), and
  `IndexSlice`'s `Debug` impl (`IndexVec`'s `Debug` formats the raw `Vec`), with its import.
- How: as E0. Each function without a contract that no run checked, and then the ignored
  functions, was deleted, the tree was compiled, and each item the errors named was put back.
- What stays: nine of them, which the tree needs in order to compile:
  - `Endian::as_str`, for `Endian`'s `Debug` impl, which `TargetDataLayout`'s derived `Debug`
    needs;
  - `AbiAlign::max` and `VariantLayout::is_uninhabited`, for `layout()`;
  - `AbiAlign::min` and `deref`, `BackendRepr::is_unsized`, and `LayoutData::is_uninhabited`,
    `is_unsized` and `is_zst`, for `univariant_biased`.

  Their lines count as code, but they are neither verified nor trusted, since nothing assumes a
  contract of them: `compile-only` in `functions.csv`, `compile_only_functions` and
  `compile_only_loc` in `stages.csv` and `rows.csv`; Table 5 carries no notes (D83), and the
  paper's notes (`paper/sections/06-evaluation.notes.md`) ask the text to name them. A
  `#[thrust::ignored]` function stays in the trusted count: each of the six, the `Debug` impls of
  `Hash64`, `Endian`, `Size`, `Align`, `WrappingRange` and `IndexVec`, is a trait impl method,
  which decisions.md D56 makes trusted to meet its trait's contract, and the tree does not compile
  without it.

  Two more stayed on e9c1a02c for `LayoutData::scalar_pair` of layout/simple.rs. Since R21,
  `Hash64::new` is checked where `scalar_pair` calls it, in the `scalar_pair.rs` root, and
  `IndexVec`'s `From<[T; N]>` has no caller and is gone from fork `main` since a40add81 (fork PR
  #226).
- Counted with them: layout/simple.rs, which the tree needs for `LayoutData::scalar_pair`
  (`univariant_biased` calls it), counts in stage 6, in `univariant_biased`'s row (its `row` row
  in `parts.csv`, D86), so its lines are in LoC; `scalar_pair` is checked there by the
  `scalar_pair.rs` root (R21).
- Why: dropping what the code does not need is what extraction means (E0); these four stayed
  through the first extraction, and the module tree is checked against compilation once more
  here.
- Equivalence: no remaining code calls a dropped function. `as_u64`'s one call stays in the
  `randomize` block, which is compiled out (E6). Dropping the three ignored impls leaves each
  query of every root with the same lines when the numbered and hashed names are masked, and for
  the roots of stages 1 to 4 under a consistent renaming of those names.
- Scope: the rule is applied to functions. The derives and bounds stay as rustc writes them (E3,
  module layout), though seven derives compile without `Debug`; without `TargetDataLayout`'s,
  `Endian`'s `Debug` impl and `Endian::as_str` could go.
- Patch: `provenance/module-tree/E7-unneeded-items.patch` against fork c244643a, giving fork
  e9c1a02c's `tests/rustc_coroutine/compiler/`, that of 457fc1dc
  (record of b6176800,
  record of 457fc1dc).

## Step 2: target.rs

`src/main.rs` plus the ui_test header line is the fork's 9208ae6; the ten commits below give
`target.rs` (b05b0e4; 901ae44 only moved the file). Each patch carries its commit message.

### R0 ui_test header, edition and feature gate

- What: `//@ignore-on-host` (9208ae6, the file is not run as a test), `//@edition: 2024`,
  `#![feature(new_range_api)]` (ccd76e4).
- Why: `rustc_abi` is an edition-2024 crate; `std::range::RangeInclusive` is unstable on
  Thrust's toolchain (nightly-2025-09-08) and needs the gate there, while rustc at 89a99936
  uses it without one.
- Equivalence: compilation settings, not code. The stage files use other settings (S7).

### R1 `bitflags!` expanded by hand

- What: the `bitflags! { impl ReprFlags: u8 { ... } }` invocation becomes an inherent `impl`
  with the same constants and the three methods the target uses:

  ```rust
  bitflags! { impl ReprFlags: u8 { const IS_C = 1 << 0; ... } }
  ->
  impl ReprFlags {
      pub const IS_C: ReprFlags = ReprFlags(1 << 0); ...
      pub const fn bits(&self) -> u8 { self.0 }
      pub const fn contains(&self, other: Self) -> bool { self.0 & other.0 == other.0 }
      pub const fn intersects(&self, other: Self) -> bool { self.0 & other.0 != 0 }
  }
  ```
- Why: `bitflags` is an external crate; the rule allows inlining external-crate definitions.
- Equivalence: for the `impl` form over a newtype, `bitflags` 2.13.0 (the version in
  `Cargo.lock`) generates each constant as `from_bits_retain` of the same expression, which is
  `Self(bits)`, and `bits`, `contains` and `intersects` as `self.0`, `self.0 & other.0 ==
  other.0` and `self.0 & other.0 != 0` (its `src/public.rs`). The methods are trusted in
  `target.rs` (patch 08); bit operations are not analysed.

### R2 `WordIter` for `slice::Iter` in `BitIter`

- What: `BitIter` holds a `WordIter<'a>` (a slice and a position) instead of
  `slice::Iter<'a, Word>`; `next` yields `&words[pos]` and advances. The body of
  `BitIter::next`, with its `*self.iter.next()?`, is unchanged.
- Why: `slice::Iter` stores raw pointers, which Thrust has no model for.
- Equivalence: by inspection of the two `next` bodies, the same references in the same order,
  and `None` on every call after the end, as for `slice::Iter`; `pos` never exceeds the slice
  length, so `pos += 1` cannot overflow. `BitIter` uses no method of its iterator but `next`.
  `WordIter::next` is itself verified against its contract in stage 4.

### R3 `IdxRange` for `(a..b).map(I::new)`

- What: `BitMatrix::rows` and `IndexSlice::indices` return `IdxRange<I>` (`start`, `end`), whose
  `next` increments `start` and yields `I::new` of its old value, instead of
  `(0..self.num_rows).map(R::new)` and `(0..self.len()).map(|n| I::new(n))`.
- Why: a `Map` over a function item or closure is an adapter chain Thrust cannot type.
- Equivalence: by inspection, the same items in the same order. Both advance the counter before
  calling `I::new`, so an `I::new` that panics (the `u32` impl asserts `idx <= u32::MAX`) does so
  on the same call and leaves the same state. `IdxRange` is `Clone` but implements no iterator
  method besides `next`, while the type `indices` returned was also a `DoubleEndedIterator` and an
  `ExactSizeIterator`; no caller uses those (the file compiles), and the one consumer, `fields.indices().collect()` in
  `univariant_biased` (`collect_index_vec(fields.indices())`, R8), collects the same elements
  (only the `Vec`'s initial capacity differs).
  `IdxRange::next` is verified against its contract in stage 4.

### R4 `IterEnumerated` for `IndexSlice` iteration

- What: `iter_enumerated` returns `IterEnumerated<'a, I, T>`, which yields `(I::new(n), &raw[n])`
  for `n` in `0..len`, in place of `iter().enumerate().map(...)`. `IndexSlice::iter` returned an
  own `SliceIter<'a, T>` (a slice and a position) in place of `slice::Iter`, as did `impl
  IntoIterator for &IndexVec`; since fork 3639e4f5 (fork PR
  #325) both return std's `slice::Iter`, as
  rustc's do, and `SliceIter` is gone. The by-value `into_iter` and `iter_mut` (`vec::IntoIter`,
  `slice::IterMut`) are left as they are.
- Why: as R2 and R3.
- Equivalence: by inspection, the same items in the same order, `None` on every call after the
  end, and `I::new(n)` called on the same calls as the original `map` closure (`iter_enumerated`
  keeps its leading `let _ = I::new(self.len())`). `iter_enumerated` no longer returns an
  `impl DoubleEndedIterator + ExactSizeIterator`; no caller uses either trait. The adapters applied to them in the target
  (`map`, `filter`, `all`, `any`, `sum`, `collect`) only call `next`; where `slice::Iter`
  overrides one of them (`all`, `any`, `fold`), the override calls the closure on the same
  elements in the same order and stops at the same element. `IterEnumerated::next` is verified
  against std.rs's `IteratorSpec` contract: the `index` root selects it.

### R5 `for` loops written as `while let`

- What: in step 2, every `for` in the analysed code (nine loops; the loops of the trusted
  `univariant_biased` are left alone) becomes

  ```rust
  for x in e { body }   ->   let mut it = e.into_iter(); while let Some(x) = it.next() { body }
  ```

  where `e` is already an iterator the `into_iter()` call is omitted.
- Why: `for` expands to std's `IntoIterator::into_iter` and `Iterator::next`, whose bodies Thrust
  does not analyse; writing the calls out puts the iterator's `next`, with its contract, in the
  verified code.
- Equivalence: the Rust Reference defines `for PAT in e { body }` as
  `match IntoIterator::into_iter(e) { mut iter => loop { match Iterator::next(&mut iter) { Some(val) => ..., None => break } ... } }`
  (Expressions, Loop expressions, `expr.loop.for.desugar`) and `while let` as a `loop` around a
  `match` with `_ => break` (`expr.loop.while.let.desugar`). The rewrite differs in method-call
  syntax, which resolves to the same trait methods here (none of the iterated types has an
  inherent `into_iter` or `next`), in the lifetime of temporaries of `e`, which end at the
  `let` rather than after the loop (none of the nine `e` creates one), and in when the iterator
  is dropped, at the end of the enclosing block rather than of the loop; the only iterator with a
  `Drop` impl is the `vec::IntoIter` over `in_memory_order`, whose drop frees the buffer and has
  no other effect. The introduced names (`rows`, `entries`, `variants`, `locals`, `conflicts`,
  `assignments_iter`, `assignments_iter_mut`, `ineligible`, `in_memory_order_iter`) are not used
  in the loop bodies. `continue` in a body re-evaluates the `while let` condition, which is the
  next call of `next`, as in the `for` desugaring. For an `e` that is already an iterator, the
  blanket `impl<I: Iterator> IntoIterator for I` is the identity.
- Module layout (fork PRs #207, merged into fork
  `main` as b86b3c2c, and #218, merged as 9f4f7de9):
  the loops are rustc's `for`, which Thrust desugars itself. Every annotated `for` loop's invariant
  names its iterator `iter`, so an inner loop's `iter` is its own; an inner invariant names an outer
  loop's iterator through the outer loop's label (`iter!('l)`, fork PR
  #202, merged into fork `main` as bc82836d), also
  of an outer loop with no written invariant, whose invariant Thrust infers (fork PR
  #217, merged as 063e2923). The two outer loops of
  `coroutine_saved_local_eligibility` (layout/coroutine.rs) carry the labels `'variants` and
  `'rows`, which rustc's code does not have and which have no effect on what the loops do; the loop
  over the variants has no written invariant. None of the names step 2 introduces is left in the
  module tree.

### R6 Panic messages dropped

- What: in step 2, `unreachable!("assignment does not match variant")` and `panic!("Layout of
  fields should be Arbitrary for variants")` become `unreachable!()` and `panic!()`. In step 3,
  the two `panic!("Use of unknown address space {c:?}")` of `pointer_size_in` and
  `pointer_align_in` become `panic!("Use of unknown address space")` in univariant.rs and
  layout.rs (18bd046, abdb69b, 9b618cb, 85d0dac), and `panic!()` in values.rs (fork main
  38314f14), where a comment at each site names this entry. In values.rs,
  `panic!("obj_size_bound: unknown pointer bit size {bits}")` becomes `_ => panic!()` (fork main
  dfa0a12a), and `Size::add`'s `panic!("Size::add: {} + {} doesn't fit in u64", ..)` becomes
  `panic!()` (fork main b95a8594).
- Why: in step 2, a message builds `fmt::Arguments`, which Thrust cannot type in analysed code.
  In step 3, `{c:?}` needs `Debug` on `AddressSpace`, whose derive the stage files of d635f6b9
  comment out (`/*Debug,*/`). In
  univariant.rs and layout.rs the two functions are trusted, so their bodies, message included,
  are not analysed. In values.rs they are analysed (`find`, std's, has a contract), so
  the remaining message is dropped too. `obj_size_bound` is analysed since fork main dfa0a12a,
  where its shifts are translated, and `Size::add` since b95a8594, where it has a contract.
- Equivalence: `panic!()` and `panic!("...")` unwind the same way; the message is the payload.
- Module layout (fork main 36f400c0, 5374a386): every panic message is rustc's, step 2's
  included. `Size::bits`'s `overflow` is analysed under `#[thrust_macros::requires(false)]`: a
  panic's payload is typed and the call that panics must be unreachable
  (case-study-gaps.md, gap 8).

### R7 `?` written as `match`

- What: two of the three `?` in `layout()`, on the two `calc.univariant(...)` calls; the second
  is inside the closure passed to `variant_fields.iter_enumerated().map(...)`:

  ```rust
  let prefix = calc.univariant(...)?;
  ->
  let prefix = match calc.univariant(...) { Ok(prefix) => prefix, Err(err) => return Err(err) };
  ```

  The third, `.collect::<Result<IndexVec<VariantIdx, _>, _>>()?`, is left as it is.
- Why: `?` expands to `Try::branch` and `FromResidual::from_residual`, std bodies without
  specifications.
- Equivalence: the Rust Reference (`expr.try`) gives `?` on a `Result` as returning
  `Err(From::from(e))` on `Err(e)` from the innermost function or closure, which is where
  `return` returns too. The first site converts `LayoutCalculatorError<F>` into the same type
  (`layout()`'s return type). At the second, the closure's error type is inferred; the only
  impl that satisfies both this `?` and the outer `?` after `collect` is
  `impl<T> From<T> for T`, so it is `LayoutCalculatorError<F>` in the original too, and
  `From::from` is the identity.

## Step 3: stage files

A stage file keeps the items of `target.rs` its stage needs (dropping items is extraction and
does not appear in the patch) and adds annotations. The changes below are what its patch shows
as `-` / `+` pairs or as new code, and the changes a line selection hides (S6, S7).

### S1 Derives changed in the module tree

- What:
  - bit_set.rs: `Clone, Eq, PartialEq` on `BitMatrix` commented out, `Hash` kept.
  - lib.rs: `PartialEq` added on `StructKind`, whose rustc derives are `Copy, Clone, Debug`.

  The other derives are rustc's, but for `ReprFlags`'s `Debug` (E3, R1). Both sites carry a
  comment naming this entry or its reason.
- Why: `BitMatrix`'s ghost field `col_bound` (S11) has no executable `clone` or `eq`.
  `univariant`'s contract compares the kind with `==`.
- Equivalence: an impl that is removed is used by nothing in the tree, since Rust rejects a use
  of a missing impl at compile time and the roots compile. The added `PartialEq` is used by
  contracts only, which are annotations.

### S2 A `PartialEq` for `TargetDataLayout` with an unanalysed body

- No change remains under this entry. Since fork main 7f5be643, values.rs derives
  `PartialEq, Eq` on `TargetDataLayout` as rustc does; the solver handles the derived `&&` chain
  on fptprove develop 9b41f7c56. The number is kept so that the later entries keep theirs.

### S3 `AbiAlign::min` and `max`

- No change remains under this entry. lib.rs keeps both methods as rustc has them, with the
  `PartialOrd, Ord` derives of `Align`. The number is kept so that the later entries keep theirs.

### S4 Items copied between stage files

- What: indexvec.rs and bitset.rs carry a copy of `Idx` and its `usize` impl (bitset.rs also
  `IdxRange`), and idx.rs carries `type Word` and `WordIter` from the `bit_set` section of
  `target.rs`, so that each file is a test of its own; the drafts copy whole stages the same way
  and say so in their `// //==` markers. indexvec.rs keeps `new` and `index` of `Idx`; bitset.rs
  keeps the whole trait with other annotations: its `index_is` takes a `usize`, it has no
  `can_new`, and `new` and `plus` carry no contract.
- Why: a ui test is one file.
- Equivalence: the bodies are those of `target.rs`. The stage counts (`stages.csv`) attribute
  the copies of `Idx` and `IdxRange` in indexvec.rs and bitset.rs to their own stage only;
  `WordIter` and `IdxRange` are std stand-ins and count nowhere (decisions.md D83).

### S5 Test drivers and stubs

- What: each stage file ends with a `fn main()` that exercises the contracts (a `TargetDataLayout`
  literal, an `IdxRange` and a `WordIter` run, `DenseBitSet` inserts) and, where an input has no
  constructor in the file, a stub such as `fn words() -> &'static [Word] { unimplemented!() }`
  with an `ensures` describing it. indexvec.rs adds `filled`, a client of `IndexVec` with one loop
  invariant, as the caller its trusted contracts are checked against.
- Why: ui_test needs a `main`, and the trusted contracts need a verified caller.
- Module layout (fork 8d828010, fork PR #172, merged as c244643a): each root selects one row
  of Table 5 (decisions.md D82). `index.rs`, which selects all of `rustc_index`, calls the drivers
  of idx.rs, indexvec.rs and bitset.rs from its `main` (`drive_iterators`, `drive_index_vec`,
  `drive_bit_sets`) and selects its clients with contracts (`filled`, `two_members`,
  `column_bound`) as `crate`; `hashes.rs` selects `rustc_hashes` and has no driver. bitset.rs's
  specification of `vec![elem; n]` at the word type is gone: with `vec.rs` selected too, Thrust
  applied it to `IndexVec::from_elem` at `T` and stopped. `new_empty` takes std.rs's
  specification, which bounds the index from below, and the premise of the lemma `zero_bits_ones`
  states `0 <= k` too (issue).
- Equivalence: test code, not rustc code. The stage counts leave out `main` and the stubs
  after it, the driver, and `filled` with its loop and invariant.

### S6 Draft-only changes (stages 5 to 7)

- What: in eligibility.rs, univariant.rs and layout.rs, the drafts, which are not measured:
  - `Index` and `IndexMut` written as `std::ops::Index` / `std::ops::IndexMut`,
    `use std::ops::{Deref as _, DerefMut}`, derive lists reordered, and the doc comment of
    `IdxRange` shortened;
  - the `Model` of `IndexSlice`, which `target.rs` does not declare, given as
    `<[T] as Model>::Ty` in all three, and that of `IndexVec` as well in eligibility.rs and
    layout.rs (univariant.rs keeps `type Ty = Self`);
  - in layout.rs (b2d3d28) and eligibility.rs (6c135a0f), the two `debug_assert_eq!` of
    `invert_bijective_mapping` dropped (in eligibility.rs they call std's `map` and `sum` on the
    local `SliceIter` of R9); in layout.rs also the bodies of `coroutine_saved_local_eligibility` and `LayoutCalculator::univariant`
    replaced by `unimplemented!()` under `#[thrust::trusted]`, since the file only calls them
    (their stages are eligibility.rs and univariant.rs); the stubs carry the contracts of those
    files (09b4c232);
  - the derives of S1 and the panic messages of R6.
- Why: to reach the next frontend wall (the evaluation README, "Where stages 5–7 stop").
- Equivalence: path spelling and comments; the `Model` declarations are annotations. The dropped
  `debug_assert_eq!` compile to nothing without debug assertions (S7). The two stubbed functions
  are not code of layout.rs's stage: their bodies are kept in eligibility.rs, which checks its
  body against the contract the stub assumes, and univariant.rs, which trusts it under the same
  contract (stage 6).

### S7 Compile settings of the stage files

- What: every stage file and fail twin is compiled with `//@compile-flags: -Adead_code -C
  debug-assertions=off`, which `target.rs` does not set. values.rs, idx.rs, indexvec.rs,
  bitset.rs and eligibility.rs leave out the `//@edition: 2024` line of `target.rs` (R0), so
  ui_test compiles them with its default edition, 2021; univariant.rs and layout.rs keep it,
  with `#![feature(new_range_api)]`. The lines are left out by the line selection, so no patch
  shows them.
- Why: Thrust is run with debug assertions off (Thrust's `README.md`). The edition line was not
  carried over when the stage files were written.
- Equivalence: not a rewrite of code, but it narrows the property. rustc's release build also
  compiles `rustc_abi` without debug assertions. `-C debug-assertions=off` removes:
  - the `debug_assert!` family: in the module tree, `univariant`'s `debug_assert_eq!` that the two
    field orders give layouts of equal size (layout.rs:306 at fork main a4b96b20); the two of
    `invert_bijective_mapping` are dropped from the source (S6);
  - since `-C overflow-checks` defaults to the value of debug assertions, the overflow checks of
    `+`, `-` and `*`.

  What stays checked: the panics of `unwrap` and `expect`, indexing, `assert!`, `panic!` and
  `unreachable!`, and Thrust's own obligation that an unsigned subtraction stays non-negative,
  which it asserts in every mode although the release build wraps there
  (decisions.md D33). Thrust reads `+` and `*` as operations on unbounded
  integers, so the proofs hold for runs in which they do not overflow, and a narrowing `as` cast
  wraps into its target's range as in Rust. For the edition: compiled with
  `thrust-rustc -C debug-assertions=off -Zdump-mir=built` as edition 2021 and as 2024, each of
  the five files gives built MIR that differs only in where `StorageDead` statements and, in
  eligibility.rs, reborrows into the return place are placed (the 2024 temporary scopes), which
  does not change what the code computes.

### S8 `IndexSlice` methods restated on `IndexVec`

- What: the roots `index.rs` and `indexvec_int_range.rs` (`tests/ui/pass/rustc_coroutine/`),
  and their fail twins, define on `IndexVec` `at(&self, index: I) -> &T`,
  `&self.raw[index.index()]`, in place of the `Index` impl of `IndexSlice`, with a contract over
  the vector's sequence. `index.rs` checks `at` against it; `indexvec_int_range.rs`, whose
  `THRUST_VERIFY_ONLY` does not select `at`, assumes it (fork PR
  #254, merged as ed09475f). `len` and
  `next_index` are not restated: `IndexVec` reaches them through `Deref` to `IndexSlice`, as
  rustc does, and `push` calls `IndexSlice::next_index`; `is_empty`, which nothing calls, is in
  neither, as the extraction dropped it (E0). The extern-spec contracts of
  `IndexVec`'s `Deref` and `DerefMut` in vec.rs (`*result == *slf`) carry the vector's sequence
  (fork PR #241, merged as 22ee73a2).
- Why: as the comment at `at` in both roots states, rustc reads an element through
  `Index<R: IntoSliceIdx<I, [T]>>` on `IndexSlice`, the part of stage 2 that waits on generic
  slices. `IndexSlice`'s `Index` has an extern-spec contract in slice.rs; whether the drivers
  verify through it without `at` has not been run.
- Equivalence: by inspection. `at` reads the element and panics exactly when `Index<I>` does,
  `into_slice_idx` of an `I` being `index()`. `at` is not rustc code and only the test code calls
  it (`drive_index_vec` in `index.rs`, `main` in `indexvec_int_range.rs`), so the stage counts
  leave it out (S5).
- Counts: vec.rs restates none of `len`, `is_empty` and `next_index`, so the committed
  `functions.csv` (fork ab598199) lists none of them.
- Patch: `stages/indexvec.patch`, of d635f6b9, shows the stage file `indexvec.rs`, which restated
  `len`, `is_empty` and `next_index` as well and trusted `at`.

### S9 `impl TryInto<u64>` parameters of `Size::from_bits` and `from_bytes` named

- What: in values.rs (eba79e9), the argument-position `impl Trait` becomes a type parameter
  with a specification trait as its bound:

  ```rust
  pub fn from_bits(bits: impl TryInto<u64>) -> Size
  ->
  pub fn from_bits<T: TryIntoSpec<u64>>(bits: T) -> Size      // likewise from_bytes
  ```

  The two functions also gain contracts (`requires(T::fits(bits))`, an `ensures` through
  `T::converts_to`), which are annotations. The site carries a comment naming the reason.
- Why: `TryInto` says nothing about the converted value, and a formula cannot name the type of
  an `impl Trait` parameter to call the predicates of `TryIntoSpec` on.
- Equivalence: the D43 rewrite (decisions.md): rustc desugars
  `impl Trait` in argument position to an anonymous type parameter with that bound (Rust
  Reference, `type.impl-trait.param`). The body is unchanged. `TryIntoSpec<U>` has `TryInto<U>`
  as a supertrait (`std.rs` of the fork), so the bound implies the original one, by
  inspection; a call site is accepted only if its argument type implements `TryIntoSpec<u64>`,
  and the stage file compiles.

### S10 `F: Deref<Target = &LayoutData>` bounded by `LayoutRef`

- What: in univariant.rs and layout.rs (37a4e22d, merged into fork `main` at 31338888), the
  field-layout parameter of `univariant`, `univariant_biased` and `layout()` takes a
  specification trait as its bound:

  ```rust
  F: Deref<Target = &'a LayoutData<FieldIdx, VariantIdx>> + Copy
  ->
  F: LayoutRef<'a, FieldIdx, VariantIdx>
  ```

  `LayoutRef` has the original bound as a supertrait and one predicate, `layout_is(f, l)`, which
  says that `**f` is `l`. The `requires` that use it (`niche_wf` of every field layout) are
  annotations. The site carries a comment naming the reason.
- Why: `Deref` says nothing about the value it yields, so a contract cannot relate `F`'s model to
  the `LayoutData` it dereferences to. This is the form of S9's `TryIntoSpec`, and of the logic
  functions Creusot and Verus attach to a trait bound.
- Equivalence: the bodies are unchanged, and `LayoutRef` has the original bound as a supertrait,
  so the new bound implies the original one, by inspection. A call site is accepted only if its
  argument type implements `LayoutRef`, and the stage files compile.
- Module layout (fork `trusted-reduction-rq4`, from `eval3-trusted` cca21952 and 4fdb919e): undone.
  `F`'s bound is rustc's, each `F::layout_is(x, l)` is written `post!(<F as Deref>::deref(&x), d)
  && **d == l`, and `univariant`, `univariant_biased` and `layout()` require that `F`'s `deref` is
  a function (`deref_functional`), which rustc's `Layout` satisfies
  (2026-10-18 record, item 4).

### S11 Ghost fields in `BitMatrix` and `BitIter`

- What: `BitMatrix` gains `col_bound: Ghost<USize>`, a bound on the columns of its set bits
  (cf495c7d). `wf` bounds it by `num_columns`, and `BitMatrix::iter`'s `ensures` bounds the
  iterated columns by it; the tree has no `BitMatrix` constructor, so the field is read in
  contracts only. `BitIter` gains four ghost fields (`bound`, `count`, `left`, `rest`: a bound on
  the elements, the number yielded, the number still to come, and the members still to come).
  The `iter` of `DenseBitSet` and of `BitMatrix` set them in their bodies (bd4fd8fe), through
  bindings for the `ghost!` captures; `BitIter::new` (`#[thrust::callable]`, checked) leaves them
  empty. `BitIter::next` (b5309e73) names `self` `this`, which its loop invariant reads, and sets
  `count`, `left` and `rest` before it returns an element:

  ```rust
  let domain_size = self.domain_size;     // DenseBitSet::iter
  let words = &self.words;                // DenseBitSet::iter
  let col_bound = &self.col_bound;        // BitMatrix::iter
  let all_words = &self.words;            // BitMatrix::iter
  let this = self;                        // BitIter::next, whose body then reads `this.`
  ```

  The arguments of the `proof!` calls in these bodies are evaluated and unused:
  `WORD_BITS * words.len()`, `end - start`, `row.index()`, `this.iter.pos`. `DenseBitSet` has no
  ghost field: its member count `card` is a logic function over `ones`, the recursive count of
  set bits below `domain_size`, and its derived `PartialEq` and `Eq` are rustc's.
- Why: eligibility's property 3 bounds the promoted field index by the number of ineligible
  locals, and `storage_conflicts` bounds every set bit's column. The count is a function of the
  words; the column bound and the iterator's progress are carried as proof-only state. A
  `ghost!` captures only local variables, and a loop invariant names the variables live at the
  loop head, which `self` cannot be.
- Equivalence: `Ghost` has no runtime data and its `Hash` is empty (`std.rs` of the fork), so the
  structs' runtime contents, their derived `Hash` and every computation on them are unchanged,
  by inspection. The bindings copy a `usize` or borrow a field, and `this` is `self` moved to a
  new name. The `proof!` arguments have no side effect: `end - start` cannot wrap (`range`
  ensures `start <= end`), `row.index()` is `Idx::index`, and `WORD_BITS * words.len()` wraps
  silently at most, as the stage files build with `-C debug-assertions=off`. The derives that
  `BitMatrix` loses are S1.

### R8 Local `Map`, `Filter`, `all`, `extend` and `collect` in `layout()` and `univariant_biased`

- What: in `layout()` (layout/coroutine.rs) and `univariant_biased` (layout.rs), std's iterator
  adapters and consumers become local definitions with contracts and verified bodies
  (tests/rustc_coroutine/thrust/iter.rs). In `layout()`:
  - `it.map(f)` becomes `Map::new(it, f)`, and `it.filter(p)` becomes `Filter::new(it, p)`;
  - `prefix_layouts.extend(it)` becomes `prefix_layouts.extend_from(it)`;
  - `.collect()` into an `IndexVec` becomes `collect_index_vec(..)`, and
    `.collect::<Result<IndexVec<VariantIdx, _>, _>>()` becomes
    `collect_index_vec_result::<VariantIdx, _, _, _>(..)`, the `?` after it kept;
  - `variants.iter().all(|v| v.is_uninhabited())` becomes `iter_all(variants.iter(), ..)`.

  In `univariant_biased`, at the sites below (upstream layout.rs line numbers):
  - `fields.indices().collect()` (1110) becomes `collect_index_vec(fields.indices())`;
  - `fields_excluding_tail.iter().map(|f| f.align.bytes()).max()` (1143) becomes
    `Map::new(fields_excluding_tail.iter(), |f: &F| f.align.bytes()).max()`;
  - `fields_excluding_tail.iter().filter_map(|f| f.largest_niche).map(|n| n.available(dl))
    .max().unwrap_or(0)` (1144) becomes `Map::new(fields_excluding_tail.iter(), |f: &F|
    f.largest_niche.map_or(0, |n| n.available(dl))).max().unwrap_or(0)`;
  - `fields.iter_enumerated().filter(|&(_, f)| !f.is_zst())` (1335) becomes
    `Filter::new(fields.iter_enumerated(), |&(_, f)| !f.is_zst())`;
  - `fields.iter().any(|f| f.is_uninhabited())` (1405) becomes
    `!iter_all(fields.iter(), |f| !f.is_uninhabited())`.

  `Map` is evaluation 1's adapter of `tests/ui/pass/creusot/map.rs`, implementing std's
  `Iterator` and std.rs's `IteratorSpec`.
  `Filter` yields the first item its closure accepts, with `P: Fn` in place of `FnMut`. The
  collections push each item `next` yields. `Filter::next` and `iter_all` keep a `Vec` of the
  items they passed on (`skipped`, `seen`), for the loop invariants; std's versions make no such
  allocation. The `impl Extend for IndexVec`, which nothing calls, is `#[thrust::ignored]`. `Size` and `Align` get local `PartialOrdSpec` impls stating the derived
  order of their single field, so `Ord::max` / `min` keep their calls.

  Bounds added for annotations only: `thrust_models::Model` on `F` and on the index types of
  `layout()`, with `Model<Ty: PartialEq>` on its `FieldIdx`; `Model<Ty: PartialEq>` on
  `FieldIdx` and `F` of `univariant` and `univariant_biased`; `T: Idx + Model` on the `Iterator` impl
  of `BitIter` (bit_set.rs), where rustc has `T: Idx`, and the same kind of bound on the impls of
  `IdxRange`, `SliceIter` and `IterEnumerated`.
- Why: std's `Map`, `Filter`, `Iterator::all`, `Extend::extend` and `FromIterator::from_iter`
  have no body Thrust analyses, and std.rs specified none of what they yield when trusted extern
  specs on them were rejected (user, 2026-10-01). std.rs has `IteratorSpec` for `Map` and
  `Filter`, and contracts of `Vec::extend`, `Vec::from_iter` and `collect` that state what the
  iterator produced, since 2026-10-17 (fork 98a92407, 17fb88da, 771984e8); `all` has none. The
  undo is designed in `docs/design/std-iterator-undo.md`. As for R2 to R4, a local definition puts `next`, with
  its contract, in the verified code.
- Equivalence, by inspection:
  - std's `Map::next` is `self.iter.next().map(&mut self.f)`; the local one calls `f` once per
    `Some` item, in order, and passes `None` through.
  - std's `Filter::next` is `self.iter.find(&mut self.predicate)`; the local one calls the closure
    on each item in order, stops at the first accepted one, and leaves the iterator after it. The
    site's closure captures by shared reference, so it implements `Fn`.
  - `collect` into `IndexVec` is `IndexVec::from_raw(Vec::from_iter(iter))`, and the local
    function pushes the same items in the same order. `collect` into `Result` stops at the first
    `Err` and returns it, as `iter::try_process` does.
  - `Extend::extend` appends every item in order, as does `extend_from`, which needs
    `T: Copy`; `F` is `Copy`. `Iterator::all` returns `false` at the first rejection and `true`
    at the end, as `iter_all` does.
  - The derived `Ord` of a single-field struct is the field's order.
  - The callers use no adapter method std overrides (`size_hint`, `fold`, `nth`).
  - `max` and `unwrap_or` are std's, applied to the local `Map` as to std's. In
    `univariant_biased` the `largest_niche_size` maxima agree: the original takes the maximum of
    `available` over the fields that have a niche and `0` when there is none; the rewrite takes
    the maximum over all fields of `available` or `0`. `Niche::available` returns `u128`, so it
    is never below `0`, and the two maxima are equal. Both call `available` only on the fields
    that have a niche, in field order, so the assertion inside it fails on the same field.
  - `!iter_all(it, |f| !p(f))` is `any(p)`: both stop at the first item `p` accepts and are
    `false` at the end of the iterator.
  - `Filter::next`'s `skipped` and `iter_all`'s `seen` are only pushed to. `Vec::push` panics only
    on capacity overflow, which std.rs's specification of `push` leaves outside the model, so
    the allocation adds no panic to the property the proof is about.

  Checked by the probes `iter_item_count`, `map_closure_pre`, `slice_iter_count`,
  `iter_filter_count`, `iter_all`, `collect_index_vec_result` and `derived_ord_struct` under
  `tests/ui/{pass,fail}/rustc_coroutine/probes/`, which keep their own copy of Creusot's iterator
  trait.

### R9 Closure contracts at std's `find` calls

- What: `pointer_size_in` and `pointer_align_in` in lib.rs call std's
  `self.address_space_info.iter().find(..)`; the call is rustc's. The closure carries a written
  contract (`thrust_macros::closure!` with `captures(c: &mut &AddressSpace)`, `requires(true)` and
  `ensures(result == ((**p).0 == **c))`), an annotation, and the capture is named as the `FnMut`
  call reads it. The case study has no local `find`: std.rs's contract of `Iterator::find` states
  the predicate's states along the items it rejected, and the case study's iterators implement
  std's `Iterator` and std.rs's `IteratorSpec` (tests/rustc_coroutine/thrust/iter.rs,
  `rustc_index`).
- Why: the `panic!` branch needs that the closure rejected the items before the one found, which
  std.rs's `find` states along its state chain.
- Equivalence: no rewrite of the call. The closure is rewritten: rustc's `|(a, _)| a == &c`
  (upstream lib.rs:687 in `pointer_size_in`, :725 in `pointer_align_in`) becomes
  `|p: &&(AddressSpace, PointerSpec)| -> bool { p.0 == c }`, as `closure!` needs a typed
  parameter and, with `ensures`, a return type (fork 0602f721 `thrust-macros/src/closure.rs:172-186`).
  Both compare the item's `AddressSpace` with `c` through `AddressSpace`'s derived `PartialEq`
  (`a == &c` compares two references, which compares the values). A typed tuple pattern,
  `|(a, _): &&(AddressSpace, PointerSpec)| -> bool { a == &c }`, may be accepted by `closure!` at
  0602f721, whose doc comment allows destructuring tuples and shared references (closure.rs:20-23);
  it was not tried.

  Checked by values.rs and its fail twin. std.rs's `find` is a trusted specification (it follows `fold`'s state chain and Creusot's `Map`; the closure's `pre!` is required at the chain's states only, fork ba988afa); values.rs has no run at fork 61faaf05.

### R10 Writes to `assignments` as helper functions in `coroutine_saved_local_eligibility`

- What: the writes of two loops of layout/coroutine.rs move into two private functions of the
  same module.
  - In the storage-conflict loop, `ineligible_locals.insert(remove); assignments[remove] =
    Ineligible(None);` becomes `make_ineligible(&mut ineligible_locals, &mut assignments, remove)`.
  - In the last loop, `assignments[local] = Ineligible(Some(FieldIdx::new(idx)))` becomes
    `set_assignment(&mut assignments, local, Ineligible(Some(FieldIdx::new(idx))))`.

  Each helper's body is the code it replaces. Its contract states what that code does: the
  lengths, the set's new member, the new value at the written position, and the old value at
  every other position. `make_ineligible`'s set facts come from `DenseBitSet::insert`'s contract,
  which is trusted. The stage 5 root selects both helpers, so their bodies are checked, and
  their `requires` are checked at the calls.
- Why: no annotation can do it. Without the helpers the solver gives no answer on these two
  loops' back edges, at 6 GB and at the protocol's 8 GB. Every call's result gets a template, an
  unknown over every live variable, also when the callee's contract is written in full (fork
  issue #163). The solver does not finish
  eliminating the chain of those templates:
  - in the storage-conflict loop, the chain of `insert`, `deref_mut` and `index_mut` after the
    join of the `if` that picks `remove`;
  - in the last loop, the recursive element templates of `index_mut`'s `&mut`.

  Of Thrust's annotations:
  - an invariant replaces only a loop head's template;
  - a lemma, `proof_assert!` and `assume!` each add facts and keep every template;
  - with the predicates opaque, these back edges still give no answer (fragment map, goals 36
    and 16).

  A call to a function with a written contract is read through that contract, so the back edge
  holds one template where it held the chain. Fork #164's opt-in `THRUST_CALL_RESULT_DIRECT`
  (#163; fork main d3516006) makes `make_ineligible` unnecessary, but with it, and `make_ineligible`
  inlined, the last loop's back edge gets no answer in 120 s whether `set_assignment` is kept or
  inlined, so both helpers stay
  (2026-11-03-rq4-stage5-call-result-direct.md).
  Analysis and runs:
  2026-11-02-rq4-stage5-annotated-root.md.

  The variants loop's `Assigned(idx)` arm makes the same two writes and keeps them: its back edge
  answers at 8 GB.
- Equivalence: each helper's body holds the statements of its call site, in the same order. Its
  parameters are borrowed or copied as the statements used them (`remove` and `local` are
  `Copy` indices). `FieldIdx::new(idx)`, and with it its panic, still comes before `index_mut`:
  an assignment evaluates the assigned value before the assignee (Rust Reference, Expressions,
  Assignment expressions), and a call evaluates its arguments before the body. `insert`'s and
  `index_mut`'s bound checks run inside the helpers on the same values.

### R11 to R16 Helpers and bindings in `layout()`

The stage 7 root (fork branch `rq4-stage7-verify2`, unmerged) moves five pieces of `layout()` into
private functions of layout/coroutine.rs and binds some arguments to names. Each helper's body is
the code it replaces, in the same order; its contract states what that code reads and what
`layout()` reads of its result. The root selects every helper, so their bodies are checked, and
their `requires` are checked at the calls.

- What:
  - R11: the loop that splits `in_memory_order` at `b_start` becomes `split_in_memory_order`
    (its `ensures`: the second part has n - b entries, each below n - b).
  - R12: in the per-variant closure, `let FieldsShape::Arbitrary { offsets, in_memory_order } =
    variant.fields else { unreachable!() }` and the combination of the variant's offsets and
    memory order with the promoted ones become `combine_offsets(.., index, .., variant.fields,
    fields.len())`; the added arguments, the variant's index and the number of its fields, are
    read only by the contract (univariant's `arbitrary_of`, and the number is the count of the
    variant's entries Assigned to it).
  - R13: the tag's layout (`let tag_layout = tag_to_layout(tag)`), `repr`, the per-variant
    closure's `calc`, the `univariant` arguments and R14's collected fields are bound to names,
    for `assume!`, `proof_assert!` and the helpers' arguments, which name local variables only.
  - R14: the variant's field layouts (`variant_fields.iter().filter(..).map(..).collect()`,
    with R8's `Filter`, `Map` and `collect_index_vec`) become `variant_field_layouts`, whose
    `ensures` gives their count (at most the locals Assigned to the variant, and, by
    `map_count_exact`, the number of its entries Assigned to it) and that each is an entry of
    `local_layouts`.
  - R15: `prefix_layouts.push(tag_layout)` and `prefix_layouts.extend(promoted_layouts)` with
    the map over the ineligible set become `promote_locals`, whose `ensures` gives the length
    P + 1 + c with 0 <= c, the kept entry fields, the tag's layout at P, that the rest are entries of
    `local_layouts`, `FieldIdx`'s bound and `layouts_wf` of the result.
  - R16: the `match prefix.fields` that splits the prefix's offsets and memory order becomes
    `split_prefix_fields`, whose `ensures` gives the promoted parts' lengths and the memory
    index's entries below its length.
- Why: no annotation does it. In `layout()`'s body every call's result gets a template, and the
  parts after `extend_from` gave no answer at 60 s and 120 s, PCSat still eliminating that chain
  of templates at 60 s; a `proof_assert!` adds facts and keeps every template (as R10's reason).
  A call to a function with a written contract is read through that contract, so the chain is
  one template per helper. With the helpers, the parts of `layout()`'s body before the variants
  are `sat` at 60 s (record). R12's
  destructuring moved into the helper so that `layout()` reveals no predicate: `arbitrary_of`
  passes as an atom from `univariant`'s `ensures` to the two helpers, which reveal it.
  With `THRUST_CALL_RESULT_DIRECT` (a call's result typed by the callee's contract) the helpers
  stay: it answered two of f3165768's open parts and one of 413dd75a's, which annotations closed
  at later heads, none of ed1bfcf7's, and at the head two that are `sat` without it as well; each helper also states a
  fact through a reveal `layout()` avoids, holds a loop, or holds closures that would otherwise
  nest in the per-variant closure (the record's section on the option).
- Equivalence: each helper's body holds the statements of its call site, in the same order, on
  the same values; its parameters are moved, borrowed or copied as the statements used them.
  The `unreachable!()` of R12 and R16 is the one of the code they replace, on the same
  condition. R12's added `fields.len()` has no effect (`IndexVec::len` cannot panic). R13's
  bindings evaluate each argument once, where the call evaluated it.

### R17 The per-variant closure's body in `layout()` as `variant_layout`

- What: the closure's body from the collection of the variant's fields (R14) to the combination of
  its offsets (R12) becomes `variant_layout(calc, local_layouts, &assignments, &promoted_offsets,
  &promoted_memory_index, prefix_size, prefix_align, index, variant_fields)`, which returns the
  variant's `LayoutData`; the closure keeps `?` on it, the updates of `size` and `align` and
  `VariantLayout::from_layout`. The helper binds `m = fields.len()` and `layout()` binds
  `variant_fields.iter_enumerated()` to `variant_iter` (R13).
- Why: with the facts written out in the closure's `requires`, `Map::new` and `collect` over the
  variants and the closure's `combine_offsets` call gave no answer at 300 s
  (record). The helper's `requires`
  is opaque predicates, which the closure's `requires` and `Map::new`'s precondition read as
  atoms: three at 59eca03d, which `layout()` establishes by lemmas, and from 2aeffe7d a fourth,
  `variant_fits_wf`, which `layout()` assumes (below); a `proof_assert!` can name the helper's parameters,
  which it cannot do for a closure's captures (a capture is a projection, not a local), so the
  facts `combine_offsets` requires are stated as cuts before the call.
- Equivalence: the helper's body is the closure's statements in the same order on the same
  values; the captures are passed as the references the body used. The helper's `?` on
  `univariant` returns the error the closure's `?` returned, and the closure's `?` on the helper's
  result converts it with the identity `From`. The bindings evaluate `fields.len()` and
  `iter_enumerated()` once, where the code evaluated them.
- R12's destructuring and `combine_offsets` call, and R13's bindings of `calc`, the `univariant`
  arguments and R14's fields, now sit in `variant_layout` instead of the closure.
- The variant's `fits_in_any_order` (A2) is a fourth atom of the helper's and the closure's
  `requires`, `variant_fits_wf` (fork 2aeffe7d, unmerged): every field list that holds, in order,
  the layouts of a selection of the variant's Assigned entries fits in any order after the prefix.
  `layout()` assumes it for every variant right after the prefix's `univariant` call, where its
  size bound and the prefix's size and alignment make it true; `variant_layout` proves
  `fits_in_any_order` at its fields from it, with `variant_field_layouts`' `ensures` that the
  fields are such a selection. The `assume!` sits where the facts it rests on hold, and its
  reachability twin is in `layout()`.

### R18 The body of `univariant_biased`'s field loop as `place_field`

- What: in `univariant_biased`'s field loop, the statements after the two unsized checks (the
  `pack` clamp of the field's alignment, `align_to` and the two maxima, the write
  `offsets[i] = offset`, the niche choice, and `checked_add(..).ok_or(SizeOverflow)?`) become
  `place_field(offset, align, max_repr_align, offsets, i, largest_niche,
  largest_niche_available, field, pack, niche_bias, dl)?`. Within it, `align_to` with the
  alignment's maximum is `align_step`, the write is `set_offset`, and the niche choice is
  `offer_niche`.
- Why: every call result gets an unknown over the variables in scope, so a fact the size
  arithmetic needs passes through the unknowns of every call before it
  (the fork's `tests/ui/pass/rustc_coroutine/probes/call_result_chain.rs`, which verifies with the
  loop body as a function with a checked contract and the loop's invariant written, and not with
  either alone, record). A function with a written contract cuts the body there; the `place_field` root checks
  the body against it: at fork 9d8c45ce all its goals are `sat` and its twin is refuted
  (record).
- Equivalence: the helpers' bodies are the loop body's statements in the same order on the same
  values, the loop's locals passed as `&mut`; the `?` returns the same error, which the loop's `?`
  passes on with the identity `From`.

### R19 `univariant_biased`'s field loop as `place_fields`

- What: `let mut unsized_field = None::<&F>;` and the loop `for &i in &in_memory_order { .. }`
  become `let unsized_field = place_fields(fields, &in_memory_order, kind, pack, &niche_bias, dl,
  &mut offsets, &mut offset, &mut align, &mut max_repr_align, &mut largest_niche,
  &mut largest_niche_available)?;`. The helper holds the declaration and the loop over
  `in_memory_order` and returns `Ok(unsized_field)`; it binds its parameters `offset` and `align`
  to `next_offset` and `struct_align`, which the loop uses, so that its invariant names their
  entry values by the parameters. The loop body ends with the call to `place_field` (R18).
- Why: with the loop in `univariant_biased`, the loop's inferred invariant joined every goal after
  it to the whole body (about 390 clauses a part,
  record). The helper's contract
  cuts the body at the loop, and `univariant_biased` reads the helper by it. The `place_fields`
  root, a draft, checks the loop against a written invariant over the loop's history (`produced`);
  the goals it leaves open are with fptprove (branch `thrust-benchmarks` 73fac2d8d2 and
  6bf31e3c90).
- Equivalence: the helper's body is the declaration and the loop in the same order on the same
  values; a `return Err(..)` of the loop returns the same error from the helper, which the call's
  `?` passes on with the identity `From`, and the helper's `Ok` carries `unsized_field`, which
  `univariant_biased` reads after the loop.

### R20 The data layout as a parameter of `alignment_group_key`

- What: in `univariant_biased`, the closure `alignment_group_key`, which captured the data layout
  `dl`, takes it as a second parameter: `|layout: &F, dl: &TargetDataLayout|`, and the two sort
  keys call it as `alignment_group_key(f, dl)`.
- Why: its precondition (the field's niche is well formed under the data layout, which
  `Niche::available` needs) is stated of the data layout, and a sort key that calls it has to
  establish it from its own precondition. With the layout captured, the precondition names the
  captured layout, which a sort key's written contract (`closure!`) cannot relate to its own:
  Creusot (`f.precondition(args)`) and Verus (`call_requires(f, args)`) name a captured closure's
  precondition in the caller's specification, but `closure!` restates each capture it names with
  its type, and a closure's type cannot be written. As a parameter, the precondition is stated
  of the argument, which the keys pass from their own capture.
- Equivalence: both keys pass the `dl` the closure captured, the same reference, so each call
  computes the same value.

### R21 `IndexVec::from_raw` in `scalar_pair`

- What: in `LayoutData::scalar_pair` (layout/simple.rs), the two `[..].into()` that build the
  pair's `offsets` and `in_memory_order` become `IndexVec::from_raw([..].into())`. The niche
  chain `Niche::from_scalar(dl, b_offset, b).into_iter().chain(Niche::from_scalar(dl, Size::ZERO, a))
  .max_by_key(..)` calls std's `Iterator::max_by_key`, as rustc does; its key closure is written
  with `closure!`, which restates the captured `dl` as `&mut &TargetDataLayout` (the `FnMut`
  bound of `max_by_key` makes the closure hold it behind a `&mut`).
- Why: `Into::into` of an array into an `IndexVec` goes through the blanket `impl Into for T:
  From`, which std.rs cannot specify per impl; `<[T; N] as Into<Vec<T>>>::into` has a
  specification. std.rs's `max_by_key` requires the key's precondition only in the states the
  closure reaches (`hist_inv!`), as `sort_by_key` does; `Niche::available` requires the niche to
  be well formed, which holds of the niches `from_scalar` returns but not of every niche.
  Creusot's `Map` specification preserves the closure's invariants the same way
  (`creusot-std/src/std/iter/map.rs`); Verus (d804d56) specifies only `cmp::max_by_key` of two
  values (`source/vstd/std_specs/cmp.rs:313`), not the iterator's.
- Equivalence: `IndexVec`'s `From<[T; N]>` is `IndexVec::from_raw(array.into())`, which the
  rewrite writes out. After the rewrite that impl (the fork's `rustc_index/src/vec.rs`) has no
  caller, so under D74 and D86 it leaves the extraction: fork `main` a40add81 (fork PR
  #226). The key closure written with `closure!`
  has rustc's body, `niche.available(dl)`; `closure!` adds its contract (`requires` that the niche
  is well formed in `dl`) and restates the capture of `dl` behind the `&mut` the `FnMut` bound
  gives it.
- Contract: the verification of `scalar_pair` rests on std.rs's `Chain` specification, whose
  `produces_trans` is `#[thrust::trusted]` (case-study-gaps.md gap 18), and on std.rs's
  `max_by_key` contract.
- Fail twin: the `scalar_pair` root's twin breaks two goals, and the test passes only if both are
  refuted (`//~ ERROR` on the driver, `//@error-in-other-file` for layout/simple.rs). It asserts
  `b_offset.raw == 0` before the niches are computed: placed after the `max_by_key` call, that
  assertion was refuted in only some runs, so no goal of `scalar_pair` after the call is broken by
  the twin. It also asserts false after a call of `max_by_key` on a chain of two concrete `u32`
  options with a closure that requires nothing (`drive_max_by_key`), which shows the code after
  such a call reachable under the contract. That this carries over to the site's `Niche` items is
  inferred, not run: the
  `produces` and `completed` of `Chain` and of `Option`'s iterator are generic in the item type,
  and `max_by_key`'s `ensures` does not mention the closure, whose precondition the site
  establishes as a separate goal. Verdicts on fork `main` c8544c5a: record; with the local
  `max_by_key` before it, record.

### F1 Fail twins

Each fail twin of tests/ui/fail/rustc_coroutine/ is generated from its pass root by the patch of
the same name in tests/rustc_coroutine/fail/ (`check.sh` regenerates and compares them). The patch
drops the header line `//@check-pass`, adds `//@no-rustfix`, and changes the code where its
contracts must refute it. Where the refuted part is in the root itself, the patch marks that
function with `//~ ERROR: Unsat`; otherwise it adds `//@error-in-other-file: Unsat`.
- values.rs: `AddressSpace(1)` where the `requires` of `pointer_size_in` demands the default
  one; refuted in `main`'s part.
- index.rs: three changes, each its own part: `assert!(a.unwrap() == 1)` against
  `IdxRange::next`'s `Some(0)` in `drive_iterators`, which has no contract and is in `main`'s
  part; `v.push(elem + 1)` against `filled`'s postcondition; and `assert!(e < 4)` against the
  column bound 5 of `column_bound`.
- indexvec_int_range.rs, a check of `THRUST_INT_RANGE`: `v.push(elem + 1)` against `filled`,
  with `error-in-other-file` and the solver timeout 120 s where the pass root has 300 s.
- univariant.rs: `drive`'s `requires` without `deref_functional`, which `univariant` requires.
- hashes.rs has no driver and no fail twin.

The stage 5 twin, eligibility.rs (fork PR #162, unmerged), departs from this pattern.
- Its root has no driver to break, and its function is private to its module.
- Its patch changes only the header: it selects `set_assignment` (R10) and adds
  `--cfg thrust_eligibility_fail`.
- The code it breaks is a `cfg_attr` in layout/coroutine.rs. Under that cfg, the `cfg_attr` adds
  an `ensures` that the write gives every position the new value, which the helper's body
  refutes.
- In every other root the cfg is unset and the attribute is inert. The module allows
  `unexpected_cfgs`, so no root warns.
- The twin does not check `coroutine_saved_local_eligibility` itself. A stage-level twin was
  tried by removing `make_ineligible`'s frame clause, which the root needs. The storage-conflict
  back edge then gives no answer instead of `sat`, not `Unsat`, so it would not fail reliably.
