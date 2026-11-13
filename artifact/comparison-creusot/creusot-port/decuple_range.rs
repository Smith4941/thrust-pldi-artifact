extern crate creusot_std;
use creusot_std::prelude::*;

// The client `decuple_range` of the artifact of Denis and Jourdan, "Specifying and Verifying
// Higher-order Rust Iterators" (TACAS 2023), `examples/decuple_range.rs`, in the syntax of this
// version: `map_inv` with a closure contract, then `collect`.
pub fn decuple_range() {
    let v: Vec<u32> = (0..10)
        .map_inv(
            #[requires(x@ < 100)]
            #[ensures(result@ == x@ * 10)]
            |x: u32, _| x * 10,
        )
        .collect();

    proof_assert! { forall<i: Int> 0 <= i && i < v@.len() ==> v[i]@ == i * 10 };
}
