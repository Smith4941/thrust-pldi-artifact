// WHY3PROVE
#![feature(unboxed_closures)]
extern crate creusot_std;
use creusot_std::{invariant::Invariant, prelude::*};

mod common;
use common::Iterator;

pub struct Map<I: Iterator, F> {
    pub iter: I,
    pub func: F,
    pub produced: Snapshot<Seq<I::Item>>,
}

impl<I: Iterator, B, F: FnMut(I::Item, Snapshot<Seq<I::Item>>) -> B> Iterator for Map<I, F> {
    type Item = B;

    #[logic(open, prophetic)]
    fn completed(&mut self) -> bool {
        pearlite! {
            *(^self).produced == Seq::empty() &&
            self.iter.completed() && self.func == (^self).func
        }
    }

    #[logic(open, law)]
    #[ensures(self.produces(Seq::empty(), self))]
    fn produces_refl(self) {
        proof_assert!(forall<x: Seq<I::Item>> x.concat(Seq::empty()) == x);
    }

    #[logic(open, law)]
    #[requires(a.produces(ab, b))]
    #[requires(b.produces(bc, c))]
    #[ensures(a.produces(ab.concat(bc), c))]
    fn produces_trans(a: Self, ab: Seq<Self::Item>, b: Self, bc: Seq<Self::Item>, c: Self) {
        proof_assert!(forall<fs1: Seq<&mut F>, s1: Seq<I::Item>, fs2: Seq<&mut F>, s2: Seq<I::Item>>
            a.func.hist_inv(c.func) && a.steps(ab, b, fs1, s1) && b.steps(bc, c, fs2, s2) ==>
            { Self::steps_trans(a, ab, b, bc, c, fs1, s1, fs2, s2); a.produces(ab.concat(bc), c) });
    }

    #[logic(open, prophetic, inline)]
    fn produces(self, visited: Seq<Self::Item>, succ: Self) -> bool {
        pearlite! {
            self.func.hist_inv(succ.func)
            && exists<fs: Seq<&mut F>> fs.len() == visited.len()
            && exists<s: Seq<I::Item>>
                #[trigger(self.iter.produces(s, succ.iter))]
                s.len() == visited.len() && self.iter.produces(s, succ.iter)
            && succ.produced.inner() == self.produced.concat(s)
            && (forall<i> 1 <= i && i < fs.len() ==>  ^fs[i - 1] == * fs[i])
            && if visited.len() == 0 { self.func == succ.func }
               else { *fs[0] == self.func &&  ^fs[visited.len() - 1] == succ.func }
            && forall<i> 0 <= i && i < visited.len() ==>
                 self.func.hist_inv(*fs[i])
                 && (*fs[i]).postcondition_mut((s[i], Snapshot::new(self.produced.concat(s.subsequence(0, i)))), ^fs[i], visited[i])
        }
    }

    #[ensures(match result {
      None => self.completed(),
      Some(v) => (*self).produces_one(v, ^self)
    })]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => {
                proof_assert! { self.func.precondition((v, self.produced)) };
                let produced = snapshot! { self.produced.push_back(v) };
                let r = (self.func)(v, self.produced);
                self.produced = produced;
                snapshot! { Self::produces_one_invariant };
                Some(r)
            }
            None => {
                self.produced = snapshot! { Seq::empty() };
                None
            }
        }
    }
}

impl<I: Iterator, B, F: FnMut(I::Item, Snapshot<Seq<I::Item>>) -> B> Map<I, F> {
    #[logic(open)]
    #[requires(a.func.hist_inv(c.func) && a.steps(ab, b, fs1, s1) && b.steps(bc, c, fs2, s2))]
    #[ensures(a.produces(ab.concat(bc), c))]
    pub fn steps_trans(a: Self, ab: Seq<B>, b: Self, bc: Seq<B>, c: Self,
        fs1: Seq<&mut F>, s1: Seq<I::Item>, fs2: Seq<&mut F>, s2: Seq<I::Item>) {
        proof_assert!(forall<x: Seq<I::Item>, y: Seq<I::Item>, z: Seq<I::Item>> x.concat(y.concat(z)) == x.concat(y).concat(z));
        proof_assert!(forall<x: Seq<I::Item>, y: Seq<I::Item>, i: Int> 0 <= i && i <= x.len() ==> x.concat(y).subsequence(0, i) == x.subsequence(0, i));
        proof_assert!(forall<x: Seq<I::Item>, y: Seq<I::Item>, i: Int> x.len() <= i && i <= x.len() + y.len() ==>
            x.concat(y).subsequence(0, i) == x.concat(y.subsequence(0, i - x.len())));
        let (fs, s, v) = (fs1.concat(fs2), s1.concat(s2), ab.concat(bc));
        proof_assert!(a.func.hist_inv(b.func));
        proof_assert!(forall<i> 0 <= i && i < ab.len() ==>
            fs[i] == fs1[i] && s[i] == s1[i] && v[i] == ab[i] && s.subsequence(0, i) == s1.subsequence(0, i));
        proof_assert!(forall<i> ab.len() <= i && i < v.len() ==>
            fs[i] == fs2[i - ab.len()] && s[i] == s2[i - ab.len()] && v[i] == bc[i - ab.len()]
            && a.produced.concat(s.subsequence(0, i)) == b.produced.concat(s2.subsequence(0, i - ab.len())));
        proof_assert!(fs.len() == v.len() && s.len() == v.len() && a.iter.produces(s, c.iter)
            && c.produced.inner() == a.produced.concat(s)
            && (forall<i> 1 <= i && i < fs.len() ==>  ^fs[i - 1] == * fs[i])
            && if v.len() == 0 { a.func == c.func }
               else { *fs[0] == a.func &&  ^fs[v.len() - 1] == c.func }
            && forall<i> 0 <= i && i < v.len() ==>
                 a.func.hist_inv(*fs[i])
                 && (*fs[i]).postcondition_mut((s[i], Snapshot::new(a.produced.concat(s.subsequence(0, i)))), ^fs[i], v[i]));
        proof_assert!(a.steps(v, c, fs, s));
    }

    #[logic(open, prophetic)]
    pub fn steps(self, visited: Seq<B>, succ: Self, fs: Seq<&mut F>, s: Seq<I::Item>) -> bool {
        pearlite! {
            fs.len() == visited.len() && s.len() == visited.len() && self.iter.produces(s, succ.iter)
            && succ.produced.inner() == self.produced.concat(s)
            && (forall<i> 1 <= i && i < fs.len() ==>  ^fs[i - 1] == * fs[i])
            && if visited.len() == 0 { self.func == succ.func }
               else { *fs[0] == self.func &&  ^fs[visited.len() - 1] == succ.func }
            && forall<i> 0 <= i && i < visited.len() ==>
                 self.func.hist_inv(*fs[i])
                 && (*fs[i]).postcondition_mut((s[i], Snapshot::new(self.produced.concat(s.subsequence(0, i)))), ^fs[i], visited[i])
        }
    }

    #[logic(open, prophetic)]
    pub fn next_precondition(iter: I, func: F, produced: Seq<I::Item>) -> bool {
        pearlite! {
            forall<e: I::Item, i: I>
                #[trigger(iter.produces(Seq::singleton(e), i))]
                inv(e) && iter.produces(Seq::singleton(e), i) ==>
                func.precondition((e, Snapshot::new(produced)))
        }
    }

    #[logic(prophetic)]
    #[ensures(produced == Seq::empty() ==> result == Self::preservation(iter, func))]
    fn preservation_inv(iter: I, func: F, produced: Seq<I::Item>) -> bool {
        proof_assert!(forall<x: Seq<I::Item>> Seq::empty().concat(x) == x);
        pearlite! {
            forall<s: Seq<I::Item>, e1: I::Item, e2: I::Item, f: &mut F, b: B, i: I>
                #[trigger(iter.produces(s.push_back(e1).push_back(e2), i),(*f).postcondition_mut((e1, Snapshot::new(produced.concat(s))), ^f, b))]
                inv(s) && inv(e1) && inv(e2) && inv(f) ==>
                func.hist_inv(*f) ==>
                iter.produces(s.push_back(e1).push_back(e2), i) ==>
                (*f).postcondition_mut((e1, Snapshot::new(produced.concat(s))), ^f, b) ==>
                (^f).precondition((e2, Snapshot::new(produced.concat(s).push_back(e1))))
        }
    }

    #[logic(open, prophetic)]
    pub fn preservation(iter: I, func: F) -> bool {
        pearlite! {
            forall<s: Seq<I::Item>, e1: I::Item, e2: I::Item, f: &mut F, b: B, i: I>
                inv(s) && inv(e1) && inv(e2) && inv(f) ==>
                func.hist_inv(*f) ==>
                iter.produces(s.push_back(e1).push_back(e2), i) ==>
                (*f).postcondition_mut((e1, Snapshot::new(s)), ^f, b) ==>
                (^f).precondition((e2, Snapshot::new(s.push_back(e1))))
        }
    }

    #[logic(open, prophetic)]
    pub fn reinitialize() -> bool {
        pearlite! {
            forall<iter: &mut I, func: F>
                iter.completed() ==>
                Self::next_precondition(^iter, func, Seq::empty()) &&
                Self::preservation(^iter, func)
        }
    }

    #[logic]
    #[requires(inv(e) && inv(f))]
    #[requires(self.invariant())]
    #[requires(self.iter.produces(Seq::singleton(e), iter))]
    #[requires(*f == self.func)]
    #[requires((*f).postcondition_mut((e, self.produced), ^f, r) )]
    #[ensures(Self::preservation_inv(iter, ^f, self.produced.push_back(e)))]
    #[ensures(Self::next_precondition(iter, ^f, self.produced.push_back(e)))]
    fn produces_one_invariant(self, e: I::Item, r: B, f: &mut F, iter: I) {
        proof_assert!(forall<x: Seq<I::Item>, y: Seq<I::Item>, z: Seq<I::Item>> x.concat(y.concat(z)) == x.concat(y).concat(z));
        proof_assert!(forall<x: Seq<I::Item>, y: I::Item> x.push_back(y) == x.concat(Seq::singleton(y)));
        proof_assert!(forall<e2: I::Item> Seq::empty().push_back(e).push_back(e2) == Seq::singleton(e).concat(Seq::singleton(e2)));
        proof_assert!(self.produced.concat(Seq::empty()) == *self.produced);
        proof_assert! {
            forall<s: Seq<I::Item>, e1: I::Item, e2: I::Item, i: I>
                iter.produces(s.push_back(e1).push_back(e2), i) ==>
                self.iter.produces(Seq::singleton(e).concat(s).push_back(e1).push_back(e2), i)
        }
    }

    #[logic(open, prophetic)]
    #[ensures(result == self.produces(Seq::singleton(visited), succ))]
    pub fn produces_one(self, visited: B, succ: Self) -> bool {
        proof_assert!(forall<x: Seq<I::Item>> x.len() == 1 ==> x == Seq::singleton(x[0]));
        proof_assert!(forall<x: Seq<I::Item>, y: I::Item> x.push_back(y) == x.concat(Seq::singleton(y)));
        proof_assert!(forall<x: Seq<I::Item>> x.subsequence(0, 0) == Seq::empty() && x.concat(Seq::empty()) == x);
        pearlite! {
            exists<f: &mut F, e: I::Item>
                #[trigger((*f).postcondition_mut((e, self.produced), ^f, visited))]
                *f == self.func && ^f == succ.func
                && self.iter.produces(Seq::singleton(e), succ.iter)
                && succ.produced.inner() == self.produced.push_back(e)
                && (*f).postcondition_mut((e, self.produced), ^f, visited)
        }
    }
}

impl<I: Iterator, B, F: FnMut(I::Item, Snapshot<Seq<I::Item>>) -> B> Invariant for Map<I, F> {
    #[logic(prophetic)]
    fn invariant(self) -> bool {
        pearlite! {
            Self::reinitialize() &&
            Self::preservation_inv(self.iter, self.func, *self.produced) &&
            Self::next_precondition(self.iter, self.func, *self.produced)
        }
    }
}

#[requires(forall<e: I::Item, i2: I>
                iter.produces(Seq::singleton(e), i2) && inv(e) ==>
                func.precondition((e, Snapshot::new(Seq::empty()))))]
#[requires(Map::<I, F>::reinitialize())]
#[requires(Map::<I, F>::preservation(iter, func))]
#[ensures(result == Map { iter, func, produced: Snapshot::new(Seq::empty()) })]
pub fn map<I: Iterator, B, F: FnMut(I::Item, Snapshot<Seq<I::Item>>) -> B>(
    iter: I,
    func: F,
) -> Map<I, F> {
    Map { iter, func, produced: snapshot! {Seq::empty()} }
}
