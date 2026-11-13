extern crate creusot_std;
use creusot_std::prelude::*;

pub fn extend_index(mut v1: Vec<u32>, v2: Vec<u32>) {
    let oldv1 = snapshot! { v1 };
    let oldv2 = snapshot! { v2 };

    v1.extend(v2.into_iter());

    proof_assert! { v1@.ext_eq(oldv1@.concat(oldv2@)) };
}
