extern crate creusot_std;
use creusot_std::{logic::Int, prelude::*};

pub fn counter(v: Vec<u32>) {
    let mut cnt: usize = 0;

    let x: Vec<u32> = v
        .iter()
        .map_inv(|x, _prod| {
            proof_assert!(cnt@ == _prod.len());
            cnt += 1;
            *x
        })
        .collect();

    proof_assert! { x@.len() == v@.len() };
    proof_assert! { x@.ext_eq(v@) };
    proof_assert! { cnt@ == x@.len() };
}

