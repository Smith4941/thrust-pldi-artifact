extern crate creusot_std;
use creusot_std::{logic::Int, prelude::*};

#[ensures((^v)@.len() == v@.len())]
#[ensures(forall<i> 0 <= i && i < v@.len() ==> (^v)[i]@ == 0)]
pub fn all_zero(v: &mut Vec<usize>) {
    #[invariant(forall<i> 0 <= i && i < produced.len() ==> (^produced[i])@ == 0)]
    for x in v.iter_mut() {
        *x = 0;
    }
}
