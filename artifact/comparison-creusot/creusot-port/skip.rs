// WHY3PROVE
extern crate creusot_std;
use creusot_std::prelude::*;

mod common;
use common::Iterator;

pub struct Skip<I: Iterator> {
    pub iter: I,
    pub n: usize,
}

impl<I> Iterator for Skip<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[logic(open, prophetic)]
    fn completed(&mut self) -> bool {
        pearlite! {
            (^self).n@ == 0 &&
            exists<s: Seq<Self::Item>, i: &mut I>
                   s.len() <= self.n@
                && self.iter.produces(s, *i)
                && (forall<i> 0 <= i && i < s.len() ==> resolve(s[i]))
                && i.completed()
                && ^i == (^self).iter
        }
    }

    #[logic(open, prophetic)]
    fn produces(self, visited: Seq<Self::Item>, o: Self) -> bool {
        pearlite! {
            visited == Seq::empty() && self == o ||
            o.n@ == 0 && visited.len() > 0
            && exists<s: Seq<Self::Item>>
                   s.len() == self.n@
                && self.iter.produces(s.concat(visited), o.iter)
                && forall<i> 0 <= i && i < s.len() ==> resolve(s[i])
        }
    }

    #[logic(open, law)]
    #[ensures(self.produces(Seq::empty(), self))]
    fn produces_refl(self) {}

    #[logic(open, law)]
    #[requires(a.produces(ab, b))]
    #[requires(b.produces(bc, c))]
    #[ensures(a.produces(ab.concat(bc), c))]
    fn produces_trans(a: Self, ab: Seq<Self::Item>, b: Self, bc: Seq<Self::Item>, c: Self) {
        proof_assert!(forall<x: Seq<Self::Item>, y: Seq<Self::Item>, z: Seq<Self::Item>> x.concat(y.concat(z)) == x.concat(y).concat(z));
        proof_assert!(forall<x: Seq<Self::Item>> Seq::empty().concat(x) == x && x.concat(Seq::empty()) == x);
        proof_assert!(forall<x: Seq<Self::Item>> x.len() == 0 ==> x == Seq::empty());
    }

    #[ensures(match result {
      None => self.completed(),
      Some(v) => (*self).produces(Seq::singleton(v), ^self)
    })]
    fn next(&mut self) -> Option<I::Item> {
        let old_self = snapshot! { self };
        let mut n = std::mem::take(&mut self.n);
        let mut skipped = snapshot! { Seq::empty() };

        #[invariant(skipped.len() + n@ == old_self.n@)]
        #[invariant(old_self.iter.produces(skipped.inner(), self.iter))]
        #[invariant(forall<i> 0 <= i && i < skipped.len() ==> resolve(skipped[i]))]
        loop {
            let r = self.iter.next();
            if n == 0 || r.is_none() {
                return r;
            }
            skipped = snapshot! { skipped.concat(Seq::singleton(r.unwrap_logic())) };
            n -= 1
        }
    }
}
