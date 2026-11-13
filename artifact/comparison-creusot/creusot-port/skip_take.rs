extern crate creusot_std;
use creusot_std::{logic::Int, prelude::*};

pub fn skip_take<I: IteratorSpec>(iter: I, n: usize) {
    let res = iter.take(n).skip(n).next();

    proof_assert! { res == None };
}
