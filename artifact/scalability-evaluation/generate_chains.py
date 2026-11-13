#!/usr/bin/env python3
"""Write the std iterator programs of the scalability evaluation (RQ3) (see README.md).

usage: generate_chains.py [-n | --list]

Every program has one generic consumer over `I: IteratorSpec`, the trait through which the Thrust
fork's std.rs specifies std's iterators: `count`, a `for` loop that counts the items, whose
postcondition is that the result is the number of items the iterator produced until it was
completed. Each benchmark calls it at iterators that grow with k:

  chains_k<k>          once at each of the first k iterators of CHAINS (k distinct types of
                       different cost)
  tagged_<a>_k<k>      once at each of k types W(<range>.<a>, Tag<j>), j < k, for the adapter <a>
                       of TAGGED (take, rev, skip): the same adapter over the same range in a
                       generic wrapper W whose second field is one of k unit types, so the k
                       types are distinct and of equal cost
  take_depth_k<k>      once at (0..100).take(1) with take(1) applied k times (one type whose
                       chain has length k)
  skip_depth_k<k>      the same with skip(1) in place of take(1)
  rev_depth_k<k>       the same as take_depth_k<k> with the innermost take(1) replaced by rev():
                       std.rs specifies Rev only over ranges, so a chain holds one Rev

In each benchmark the loop invariant of `count` is written; in its `_inferred` variant
(`chains_inferred_k<k>`, ...) the same program has the invariant removed, left to inference, and
its header sets THRUST_FOR_LOOP_HISTORY=1, which gives the `for` loop the ghost history of an
annotated one, as Creusot's desugaring gives every `for` loop.

The fail twin of every program is the program with the loop body of `count` adding 2 instead of 1,
so the body breaks the loop invariant (inferred: the postcondition) and the query must be unsat;
the call sites are those of the pass side.

Each of these variants has an `_assert` variant (`take_depth_assert_k<k>`,
`take_depth_inferred_assert_k<k>`, ...) in which every caller asserts the number of items its
iterator produces, `assert!(count(<iterator>) == <n>)`, so that a clause uses the postcondition of
`count`. std.rs's `produces` and `completed` of the iterators give exactly that number. Its fail
twin keeps `count` of the pass side and has the last caller assert one item more, so only that
assertion breaks.

The programs go to programs/{pass,fail}/ next to this script, with the header, the sizes and the
file handling of generate.py. -n writes nothing and prints each file with `same`, `changed` or `new`
against the files there; --list writes nothing and prints the iterators of every program with the
number of items each produces.
"""

import argparse
import os
import sys

from generate import HERE, SIZES, header, status, write

# The inferred variants' `for` loop gets the ghost history (`iter_old`, `produced`) without a
# written invariant, as Creusot desugars every `for` loop, so that the inferred invariant can name
# the items produced so far.
HISTORY = "THRUST_FOR_LOOP_HISTORY=1"
STEP = "        c += 1;\n"
STEP_FAIL = "        c += 2;\n"  # the fail twin's step
TWIN = " Fail twin: the loop of `count` adds 2 per item, so its body breaks its {}."
TWIN_ASSERT = " Fail twin: `use_{}` asserts one item more than its iterator produces."
INVARIANT = """\
        thrust_macros::invariant!(|iter: I, iter_old: Ghost<I>, produced: Ghost<Seq<<I::Item as Model>::Ty>>, c: usize, it: FnParam<I>|
            iter_old == it.at_entry() && c == produced.len());
"""

# The integer types the chains run over, in the order they are taken.
INTS = ("i64", "i32", "i16", "i8", "isize", "u64", "u32", "u16", "u8", "usize")

# The iterators over ranges of the integer type {t}, in the order they are taken; at size k the
# program takes the first k of `chains()`: every chain at INTS[0], then at INTS[1], ... `Chain` and
# `Filter` are left out because std.rs trusts their `produces_trans` law, which the generic setting
# takes as a premise; `Map` and `Filter` with a closure get no answer at k = 1 in either setting.
# Each comes with the number of items it produces.
CHAINS = (
    ("0..10{t}", 10),
    ("(0..10{t}).take(3)", 3),
    ("(0..10{t}).rev()", 10),
    ("(0..10{t}).by_ref()", 10),
    ("(0..10{t}).enumerate()", 10),
    ("(0..10{t}).skip(2)", 8),
    ("(0..10{t}).skip(2).take(3)", 3),
    ("std::iter::zip(0..10{t}, 3..5{t})", 2),
)


USES = """\
use thrust_models::model::{Mut, Seq};
use thrust_models::{exists, FnParam, Ghost, Model};
"""

COUNT = """\
#[thrust_macros::context]
#[thrust_macros::requires(I::inv(it))]
#[thrust_macros::ensures(exists(|s: Seq<<I::Item as Model>::Ty>, last: I::Ty, fin: I::Ty|
    I::produces(it, s, last) && I::completed(Mut::new(last, fin)) && result == s.len()))]
fn count<I: IteratorSpec>(it: I) -> usize
where
    I::Item: Model,
{
    let mut c = 0;
    for _x in it {
        thrust_macros::invariant!(|iter: I, iter_old: Ghost<I>, produced: Ghost<Seq<<I::Item as Model>::Ty>>, c: usize, it: FnParam<I>|
            iter_old == it.at_entry() && c == produced.len());
        c += 1;
    }
    c
}
"""


# The adapters of the tagged benchmarks, each over the range 0..10 of i64, with the number of items
# it produces.
TAGGED = {"take": ("take(3)", 3), "rev": ("rev()", 10), "skip": ("skip(2)", 8)}

# The generic wrapper of the tagged benchmarks: W's iterator and its specification are I's, and its
# second field, a unit type, only makes W(it, Tag0), W(it, Tag1), ... distinct types.
WRAPPER = """\
#[derive(PartialEq)]
struct W<I, M>(I, M);

impl<I: Model, M: Model> Model for W<I, M> {
    type Ty = W<I::Ty, M::Ty>;
}

impl<I: IteratorSpec, M> Iterator for W<I, M>
where
    I::Item: Model,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.0.next()
    }
}

#[thrust_macros::context]
impl<I: IteratorSpec, M: Model> IteratorSpec for W<I, M>
where
    I::Item: Model,
{
    #[thrust_macros::predicate]
    fn inv(self) -> bool {
        I::inv(self.0)
    }

    #[thrust_macros::predicate]
    fn produces(self, visited: Vec<I::Item>, o: Self) -> bool {
        I::produces(self.0, visited, o.0)
    }

    #[thrust_macros::predicate]
    fn completed(&mut self) -> bool {
        I::completed(Mut::new((*self).0, (!self).0))
    }

    fn produces_refl(a: &Self) {}

    fn produces_trans(
        a: &Self,
        ab: Seq<<Self::Item as Model>::Ty>,
        b: &Self,
        bc: Seq<<Self::Item as Model>::Ty>,
        c: &Self,
    ) {
    }
}
"""


def tag(j):
    return f"""\
#[derive(PartialEq)]
struct Tag{j};

impl Model for Tag{j} {{
    type Ty = Tag{j};
}}
"""


def chains(k):
    """The first k chains, as Rust expressions with their numbers of items: every chain at INTS[0],
    then at INTS[1], ..."""
    every = [(chain.format(t=t), n) for t in INTS for chain, n in CHAINS]
    if k > len(every):
        raise SystemExit(f"only {len(every)} distinct chains")
    return every[:k]


def tagged(adapter):
    """The tagged benchmark of one adapter: the wrapper and k tags, and the k wrapped iterators."""
    call, n = TAGGED[adapter]
    def benchmark(k):
        items = [WRAPPER] + [tag(j) for j in range(k)]
        return items, [(f"W((0..10i64).{call}, Tag{j})", n) for j in range(k)]
    return benchmark


def take_depth(k):
    """One iterator: take(1) applied k times to 0..100."""
    return [], [("(0..100i64)" + ".take(1)" * k, 1)]


def skip_depth(k):
    """One iterator: skip(1) applied k times to 0..100, which leaves 100 - k items."""
    return [], [("(0..100i64)" + ".skip(1)" * k, 100 - k)]


def rev_depth(k):
    """One iterator: rev() over 0..100, then take(1) applied k - 1 times.

    std.rs, as Creusot, implements `DoubleEndedIteratorSpec` only for ranges, so `Rev<Rev<_>>` has
    no `IteratorSpec` and a chain can hold one `Rev`, innermost.
    """
    return [], [("(0..100i64).rev()" + ".take(1)" * (k - 1), 100 if k == 1 else 1)]


# benchmark: k -> (the items besides `count` and the callers, the iterators `count` is called at, each
# with the number of items it produces)
BENCHMARKS = {"chains": lambda k: ([], chains(k))}
BENCHMARKS.update({f"tagged_{a}": tagged(a) for a in TAGGED})
BENCHMARKS["take_depth"] = take_depth
BENCHMARKS["skip_depth"] = skip_depth
BENCHMARKS["rev_depth"] = rev_depth


def consumer(expect_pass, inferred):
    """`count`, its invariant removed when inferred, its step adding 2 on the fail side."""
    assert COUNT.count(STEP) == 1 and COUNT.count(INVARIANT) == 1
    text = COUNT.replace(INVARIANT, "") if inferred else COUNT
    return text if expect_pass else text.replace(STEP, STEP_FAIL)


def caller(j, chain, expected):
    """The j-th caller; with `expected`, it asserts that `count` returns it."""
    call = f"count({chain})" if expected is None else f"assert!(count({chain}) == {expected})"
    return f"""\
fn use_{j}() {{
    {call};
}}
"""


def variant(benchmark, inferred, asserting):
    return benchmark + ("_inferred" if inferred else "") + ("_assert" if asserting else "")


def expected_counts(iterators, asserting, expect_pass):
    """What each caller asserts `count` returns: nothing outside the asserting variants; on their
    fail side, the last caller one item more than its iterator produces."""
    if not asserting:
        return [None] * len(iterators)
    counts = [n for _, n in iterators]
    if not expect_pass:
        counts[-1] += 1
    return counts


def twin_comment(inferred, asserting, callers):
    if asserting:
        return TWIN_ASSERT.format(callers - 1)
    return TWIN.format("postcondition" if inferred else "loop invariant")


def program(benchmark, inferred, asserting, k, expect_pass):
    items, iterators = BENCHMARKS[benchmark](k)
    comment = (f"// Scalability evaluation, variant {variant(benchmark, inferred, asserting)}, k = {k}."
               + ("" if expect_pass else twin_comment(inferred, asserting, len(iterators))))
    body_pass = expect_pass or asserting  # the asserting twin breaks a caller, not `count`
    parts = [header(expect_pass, HISTORY if inferred else ""), "\n", comment, "\n\n", USES, "\n", consumer(body_pass, inferred)]
    for item in items:
        parts += ["\n", item]
    expected = expected_counts(iterators, asserting, expect_pass)
    for j, ((chain, _), n) in enumerate(zip(iterators, expected)):
        parts += ["\n", caller(j, chain, n)]
    parts.append("\nfn main() {\n" + "".join(f"    use_{j}();\n" for j in range(len(iterators))) + "}\n")
    return "".join(parts)


def generate():
    """The files, {relative path: text}."""
    return {os.path.join("programs", "pass" if p else "fail", f"{variant(f, inf, a)}_k{k}.rs"): program(f, inf, a, k, p)
            for f in BENCHMARKS for inf in (False, True) for a in (False, True) for k in SIZES for p in (True, False)}


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    mode = ap.add_mutually_exclusive_group()
    mode.add_argument("-n", action="store_true", help="print each file against the tree, write none; exit 1 if any file is changed or new")
    mode.add_argument("--list", action="store_true", help="print the iterators of every program and the number of items each produces, write none")
    args = ap.parse_args()

    if args.list:
        for benchmark in BENCHMARKS:
            for k in SIZES:
                for j, (chain, n) in enumerate(BENCHMARKS[benchmark](k)[1]):
                    print(f"{benchmark}_k{k} use_{j}: {chain} produces {n}")
        return 0
    files = generate()
    stale = False
    for rel, text in sorted(files.items()):
        path = os.path.join(HERE, rel)
        if args.n:
            st = status(path, text)
            stale = stale or st != "same"
            print(f"{st} {rel}")
        else:
            write(path, text)
    if not args.n:
        print(f"{len(files)} files")
    return 1 if stale else 0


if __name__ == "__main__":
    sys.exit(main())
