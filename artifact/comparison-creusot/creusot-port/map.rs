// WHY3PROVE
#![feature(unboxed_closures)]
extern crate creusot_std;
use creusot_std::{invariant::Invariant, prelude::*};

mod common;
use common::Iterator;

pub struct Map<I, F> {
    // The inner iterator
    pub iter: I,
    // The mapper
    pub func: F,
}

impl<I: Iterator, B, F: FnMut(I::Item) -> B> Iterator for Map<I, F> {
    type Item = B;

    #[logic(open, prophetic)]
    fn completed(&mut self) -> bool {
        pearlite! { self.iter.completed() && (*self).func == (^self).func }
    }

    #[logic(open, law)]
    #[ensures(self.produces(Seq::empty(), self))]
    fn produces_refl(self) {}

    #[logic(open, law)]
    #[requires(a.produces(ab, b))]
    #[requires(b.produces(bc, c))]
    #[ensures(a.produces(ab.concat(bc), c))]
    fn produces_trans(a: Self, ab: Seq<Self::Item>, b: Self, bc: Seq<Self::Item>, c: Self) {
        proof_assert!(forall<fs1: Seq<&mut F>, s1: Seq<I::Item>, fs2: Seq<&mut F>, s2: Seq<I::Item>>
            a.steps(ab, b, fs1, s1) && b.steps(bc, c, fs2, s2) ==>
            a.steps(ab.concat(bc), c, fs1.concat(fs2), s1.concat(s2)));
        proof_assert!(forall<x: Self, v: Seq<B>, y: Self, fs: Seq<&mut F>, s: Seq<I::Item>>
            x.func.hist_inv(y.func) && x.steps(v, y, fs, s) ==> x.produces(v, y));
    }

    #[logic(open, prophetic, inline)]
    fn produces(self, visited: Seq<Self::Item>, succ: Self) -> bool {
        pearlite! {
            self.func.hist_inv(succ.func)
            && exists<fs: Seq<&mut F>> fs.len() == visited.len()
            && exists<s: Seq<I::Item>>
                #[trigger(self.iter.produces(s, succ.iter))]
                s.len() == visited.len() && self.iter.produces(s, succ.iter)
            && (forall<i> 1 <= i && i < fs.len() ==>  ^fs[i - 1] == *fs[i])
            && if visited.len() == 0 { self.func == succ.func }
               else { *fs[0] == self.func &&  ^fs[visited.len() - 1] == succ.func }
            && forall<i> 0 <= i && i < visited.len() ==>
                 self.func.hist_inv(*fs[i])
                 && (*fs[i]).postcondition_mut((s[i],), ^fs[i], visited[i])
        }
    }

    #[ensures(match result {
      None => self.completed(),
      Some(v) => (*self).produces_one(v, ^self)
    })]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => {
                proof_assert! { self.func.precondition((v,)) };
                snapshot! { Self::produces_one_invariant };
                Some((self.func)(v))
            }
            None => None,
        }
    }
}

impl<I: Iterator, B, F: FnMut(I::Item) -> B> Map<I, F> {
    #[logic(open, prophetic)]
    pub fn steps(self, visited: Seq<B>, succ: Self, fs: Seq<&mut F>, s: Seq<I::Item>) -> bool {
        pearlite! {
            fs.len() == visited.len() && s.len() == visited.len() && self.iter.produces(s, succ.iter)
            && (forall<i> 1 <= i && i < fs.len() ==>  ^fs[i - 1] == *fs[i])
            && if visited.len() == 0 { self.func == succ.func }
               else { *fs[0] == self.func &&  ^fs[visited.len() - 1] == succ.func }
            && forall<i> 0 <= i && i < visited.len() ==>
                 self.func.hist_inv(*fs[i])
                 && (*fs[i]).postcondition_mut((s[i],), ^fs[i], visited[i])
        }
    }

    #[logic(open, prophetic)]
    pub fn next_precondition(iter: I, func: F) -> bool {
        pearlite! {
            forall<e: I::Item, i: I>
                #[trigger(iter.produces(Seq::singleton(e), i))]
                inv(e) && iter.produces(Seq::singleton(e), i) ==>
                func.precondition((e,))
        }
    }

    #[logic(open, prophetic)]
    pub fn preservation(iter: I, func: F) -> bool {
        pearlite! {
            forall<s: Seq<I::Item>, e1: I::Item, e2: I::Item, f: &mut F, b: B, i: I>
                #[trigger(iter.produces(s.push_back(e1).push_back(e2), i), (*f).postcondition_mut((e1,), ^f, b))]
                func.hist_inv(*f) ==>
                inv(s) && inv(e1) && inv(e2) && inv(f) ==>
                iter.produces(s.push_back(e1).push_back(e2), i) ==>
                (*f).postcondition_mut((e1,), ^f, b) ==>
                (^f).precondition((e2, ))
        }
    }

    #[logic(open, prophetic)]
    pub fn reinitialize() -> bool {
        pearlite! {
            forall<iter: &mut I, func: F>
                iter.completed() ==>
                Self::next_precondition(^iter, func) && Self::preservation(^iter, func)
        }
    }

    #[logic]
    #[requires(inv(e) && inv(f))]
    #[requires(self.invariant())]
    #[requires(self.iter.produces(Seq::singleton(e), iter))]
    #[requires(*f == self.func)]
    #[requires((*f).postcondition_mut((e,), ^f, r) )]
    #[ensures(Self::preservation(iter, ^f))]
    #[ensures(Self::next_precondition(iter, ^f))]
    fn produces_one_invariant(self, e: I::Item, r: B, f: &mut F, iter: I) {
        proof_assert!(forall<x: Seq<I::Item>, y: Seq<I::Item>, z: Seq<I::Item>> x.concat(y.concat(z)) == x.concat(y).concat(z));
        proof_assert!(forall<x: Seq<I::Item>, y: I::Item> x.push_back(y) == x.concat(Seq::singleton(y)));
        proof_assert!(forall<e2: I::Item> Seq::empty().push_back(e).push_back(e2) == Seq::singleton(e).concat(Seq::singleton(e2)));
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
        pearlite! {
            exists<f: &mut F, e: I::Item>
                #[trigger((*f).postcondition_mut((e,), ^f, visited))]
                *f == self.func && ^f == succ.func
                && self.iter.produces(Seq::singleton(e), succ.iter)
                && (*f).postcondition_mut((e,), ^f, visited)
        }
    }
}

impl<I: Iterator, B, F: FnMut(I::Item) -> B> Invariant for Map<I, F> {
    #[logic(prophetic)]
    fn invariant(self) -> bool {
        pearlite! {
            Self::reinitialize() &&
            Self::preservation(self.iter, self.func) &&
            Self::next_precondition(self.iter, self.func)
        }
    }
}

#[requires(forall<e: I::Item, i2: I>
                iter.produces(Seq::singleton(e), i2) && inv(e) ==>
                func.precondition((e,)))]
#[requires(Map::<I, F>::reinitialize())]
#[requires(Map::<I, F>::preservation(iter, func))]
#[ensures(result == Map { iter, func })]
pub fn map<I: Iterator, B, F: FnMut(I::Item) -> B>(iter: I, func: F) -> Map<I, F> {
    Map { iter, func }
}
