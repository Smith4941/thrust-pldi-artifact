# Scalability evaluation

RQ3: how verification time grows with the number k of instantiations of one generic function,
when the function is verified once over its type parameter (setting `generic`, the fork's
default) and when it is verified at every concrete type (setting `per_instance`, the same tree
with `THRUST_PER_INSTANCE_GENERICS=1`, upstream Thrust's treatment), and against verifying a
hand-specialised copy per type. The scalability benchmarks call a generic consumer of std's
iterators at k iterator types ([below](#scalability-benchmarks)); they are the main series the paper
reports, and the `repeat` series of the other variants goes to the supplementary material
(decisions.md D88).

## The generic function

Every variant but `chains` uses one trait and one generic function with a loop:

```rust
trait Step {
    #[requires(Self::inv(x))]
    #[ensures(Self::inv(result) && result > x)]
    fn step(&self, x: i64) -> i64;
    #[predicate]
    fn inv(x: i64) -> bool;
}

#[requires(T::inv(x) && n >= 0)]
#[ensures(T::inv(result) && result >= x + n)]
fn repeat<T: Step>(a: &T, x: i64, n: i64) -> i64 {
    let mut v = x;
    let mut i = 0;
    while i < n {        // invariant: T::inv(v) && v >= x + i
        v = a.step(v);
        i += 1;
    }
    v
}
```

The loop invariant is not the contract: it relates the loop state to the entry value of `x` and to
the counter besides the trait predicate, and without it `result >= x + n` does not follow. Every
call passes `n = 3` and asserts `result >= x + 3`. All calls are in one caller, `main`, so that a
tree that analyzes a callee again per pair of type arguments and caller pays once per instance.

## Variants

| Variant | Program at size k | What it stands for |
| --- | --- | --- |
| inferred (`inferred_k<k>`) | k unrelated types `S_j` (step adds `(j mod 3) + 1`, `inv(x)` is `x >= j`); `main` calls `repeat` once per type; the loop invariant is left to inference | k instances that share nothing but the generic function |
| written (`written_k<k>`) | the same with the invariant written as `invariant!` | the same with the inference taken out, to separate its cost from that of the instances |
| nested (`nested_k<k>`) | a base type (step adds 1, `inv(x)` is `x >= 0`) inside k generic wrappers `W_j<T: Step>`, each around the previous one (step is `T`'s, `inv(x)` is `T::inv(x) && x >= j + 1`); `main` calls `repeat` once on the outermost; the invariant is left to inference | k instances that contain each other: each wrapper's impl is generic too, and its type argument is the stack below it |
| perinstance (`perinstance_t<j>`, j < k) | k programs; program j holds `S_j` and a copy `repeat_s<j>` of `repeat` specialised by hand to `S_j`, with the contract instantiated at `S_j` and the invariant inferred, and the one call | what a verifier that monomorphizes has to check: the body once per type, each in its own run. It is not a measurement of another tool |

Each call passes the least start value its type's predicate allows (`j` for `S_j`, `k` for the
nested stack). The fail twin of every program is the program with the loop of `repeat` (in
perinstance, `repeat_s<j>`) stopping one step early (`while i + 1 < n`), so the body breaks
`result >= x + n` at n = 1 and the query must be `unsat`; the calls are the pass side's, so the
twin is refuted only if the body is verified (with the body `#[thrust::trusted]` it is `sat` in
both settings, from the trusted check at k = 2 and `perinstance_t1`, record). Sizes are k = 1, 2, 4, 8, 16 and 32.

## Scalability benchmarks

Each scalability benchmark has one generic function over std's iterators, `count`, and calls it at
iterators that grow with k:

```rust
#[requires(I::inv(it))]
#[ensures(exists s, last, fin. I::produces(it, s, last) && I::completed(last, fin) && result == s.len())]
fn count<I: IteratorSpec>(it: I) -> usize {
    let mut c = 0;
    for _x in it {       // invariant: c == produced.len()
        c += 1;
    }
    c
}
```

`IteratorSpec` is the trait through which the Thrust fork's `std.rs` specifies std's iterators:
the state invariant `inv`, `produces` (the items between two states) and `completed`, with `next`
specified once through them. `count` is verified once over these predicates, abstract at `I`,
with the trait's laws (`produces_refl`, `produces_trans`) as premises; per instance, at each chain,
they are the chain's definitions in `std.rs`, and Thrust states the laws only for predicates used
through a type parameter, so the solver works from the definitions. The laws of the iterators
below are proved, not trusted, by the fork's ui test
`tests/ui/pass/iterators/std_iterator_spec_laws.rs`, once for `std.rs` and outside these programs'
times. The postcondition says
that the result is the number of items the iterator produced until it was completed, and the
loop invariant, written, that `c` is the number of items produced so far. Every benchmark has an
`_inferred` variant (`tagged_take_inferred_k<k>`, ...): the same programs with the invariant
removed, left to inference, and with `THRUST_FOR_LOOP_HISTORY=1` in their header
(decisions.md D88). Creusot desugars every `for` loop with the ghosts
`iter_old` and `produced`, the items returned so far, and conjoins `produces(iter_old, produced,
iter)` to the loop invariant; the fork does so by default only for a loop whose body starts with a
written `invariant!`, and the option extends it to a loop without one. Without it the inferred
invariant cannot name `produced`, from which the postcondition's sequence `s` follows, and
`count` gets no answer even over one range
(issue,
record).

| Benchmark | Program at size k |
| --- | --- |
| `tagged_take`, `tagged_rev`, `tagged_skip` | `count` at `W((0..10).take(3), Tag<j>)` for j < k (`rev()`, `skip(2)`): one adapter over one range inside a generic wrapper `W<I, M>(I, M)`, whose `Iterator` and `IteratorSpec` impls are `I`'s, written once over `I`, and whose second field is one of k unit types `Tag<j>`. The k types are distinct and of equal cost, and `W`'s impl is generic code verified once in `generic` and at each instance in `per_instance` |
| `take_depth` | `count` once at `(0..100).take(1)` with `take(1)` applied k times: one type whose chain has length k |
| `skip_depth` | as `take_depth` with `skip(1)` in place of `take(1)`, so the chain yields 100 - k items |
| `rev_depth` | as `take_depth` with the innermost `take(1)` replaced by `rev()`: `std.rs`, as Creusot, gives `DoubleEndedIteratorSpec` only to ranges, so `Rev<Rev<_>>` implements no `IteratorSpec` and a chain holds one `Rev` |
| `chains` | `count` at k distinct iterators of different cost, listed below |

The paper calls the four it reports `k-types-take` (`tagged_take`), `k-types-skip` (`tagged_skip`),
`k-chain-take` (`take_depth`) and `k-chain-skip` (`skip_depth`).

In `chains`, function `use_j` calls `count` at the j-th of the iterators `0..10`, `(0..10).take(3)`, `rev()`,
`by_ref()`, `enumerate()`, `skip(2)`, `skip(2).take(3)` and `zip(0..10, 3..5)`, first over `i64`, then
over `i32`, `i16`, `i8`, `isize`, `u64`, ... (`generate_chains.py --list` prints them), so the k
iterators are k distinct types, a range or one or two adapters over ranges, and `main` calls
every `use_j`. `chain` and `filter` are not among them, because `std.rs` trusts their
`produces_trans` law, which the generic setting takes as a premise; nor are `map` and `filter` with
a closure, which get no answer at k = 1 within 60 s in either setting
(record). The fail twin of
every program adds 2 per item instead of 1, so the body of `count` breaks its loop invariant
(inferred: its postcondition); the calls are the pass side's. Sizes are k = 1, 2, 4, 8, 16 and 32.
The depth benchmarks at k = 32 overflowed `thrust-rustc`'s stack in Thrust's analysis before fork PR
#208 (merged as 380e6f53;
record), so `runs.csv` lists all six sizes of
every benchmark for a tree that includes it.

In these programs the callers check nothing about the result of `count`, so no clause uses its
postcondition. Every variant therefore has an `_assert` variant (`take_depth_assert_k<k>`,
`take_depth_inferred_assert_k<k>`, ...), the same program with each call written as
`assert!(count(<iterator>) == <n>)`, where n is the number of items the iterator produces, which
`std.rs`'s `produces` and `completed` of the iterator determine (`generate_chains.py --list` prints
them). Its fail twin keeps the pass side's `count` and has the last caller assert n + 1, so only
that assertion breaks.

## The settings

All programs are solved by the same solver build.

- `generic` verifies `repeat` once over the forall sort of `T`, its loop unknown depending on the
  quantified predicate `T::inv`, and instantiates its contract at each call. A contract written
  over defined predicates does not make a call site analyze the body again, so the query grows
  with k only by the caller's code around each call. In `nested`, each wrapper's impl is verified
  once over its own abstract `T`, so the query has one forall predicate per wrapper and one for
  `repeat`.
- `per_instance` analyzes a generic function with a body at each instance a call site reaches, as
  upstream Thrust does, and everything else as `generic`: the body of `repeat` at each concrete
  type argument, and in `nested` each wrapper's `step` at the concrete stack below it. On these
  programs it gives upstream's clauses and unknowns
  (record).

In `nested`, `repeat` and each wrapper's impl have one instance each, so both settings analyze the
same bodies once; on fork `main` a4b96b20 their queries have the same clauses and unknowns, and
differ only in the forall sorts and predicates of `generic`, so the variant measures the cost of
that encoding at equal work, not a saving from verifying once (sizes in the
record).

## Files

- `programs/{pass,fail}/`: the programs, and `programs/manifest.json`: per program its variant,
  k, member files, annotation items and lines of code without annotations. `generate.py` writes
  them; `generate.py -n` writes nothing and prints each file as `same`, `changed` or `new`
  against the tree. The scalability benchmarks are written by `generate_chains.py`, which takes its
  header, sizes and `-n` from `generate.py` and adds a `--list` of the iterators; they are not in
  the manifest.
- `code-lines.csv`: the lines of code of each pass program, which Table `tab:scalability` prints,
  written by `../count-annotations/` (`scalability`, rules in the header of its
  `src/scalability.rs`) in the tables step of `../scripts/run-evaluation.sh`; for the programs of
  `generate.py` they equal the manifest's.
- `runs.csv`: the inputs of `../scripts/run.py`, every program in both settings: per variant and
  k the pass side and its fail twin, and for perinstance every `perinstance_t<j>` and the fail twin
  of the last program of each size. The rows of `tagged_take`, `tagged_rev`, `tagged_skip`,
  `take_depth`, `skip_depth`, `rev_depth` and `chains`, written and inferred, come last, in their `_assert`
  form (decisions.md D98): `<benchmark>_assert_k<k>` and
  `<benchmark>_inferred_assert_k<k>`, whose callers assert the count, so that a clause uses `count`'s
  postcondition. The forms without the assertion are not
  listed; their programs were measured in the records above, the `_assert` forms first in their
  record.

`../scripts/tables.py` writes, for the paper's body, Figure `fig:scalability-iterators` (one panel
per benchmark of its `SCALABILITY_FIGURE_BENCHMARKS`: `tagged_take`, `tagged_skip`, `take_depth` and `skip_depth`;
the cells of the inferred variant, run with the loop history, against k in each setting, generic
with squares and per instance with squares turned by 45 degrees, n/N beside the marker where some runs verified but
not all, the setting's open marker at the time limit where the runs timed out or ran out of memory, and no marker for a size that was not run), and for the supplementary material Table
`tab:scalability-iterators` (the figure's benchmarks, two side by side as its panels; per
benchmark and k,
the lines of code and the cells of the written and inferred variants in each setting, t/o or OOM where no run answered, `--` for a size that was not run), Table
`tab:scalability` (per k, the lines of code of `inferred_k<k>` and per variant and setting the
median wall time of the Thrust command over the verified runs (D123), the sum over the k programs for perinstance, with n/N
when not every run verified) and Figure `fig:scalability` (its inferred and written cells).
