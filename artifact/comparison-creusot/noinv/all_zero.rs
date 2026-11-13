//@check-pass
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper

// Creusot's `all_zero`: every element of the vector is set to zero by a `for` loop over
// `iter_mut`. The loop's iterator `iter` holds the vector's entry and final sequences, so the
// invariant quantifies over the positions already passed and the postcondition over all of them;
// `slice::IterMut` has no `IteratorSpec`: std.rs gives it its own `next` contract, which states the
// same step with the preserved sequences and the cursor's step as conjuncts of each case (see the
// comment above `_extern_spec_slice_iter_mut_next` in std.rs). So the loop has no history, and the
// invariant names `iter` and not Creusot's `produced`.
use thrust_models::forall;
use thrust_models::model::Int;

#[thrust_macros::context]
#[thrust_macros::ensures(
    (!v).len() == (*v).len()
        && forall(|i: Int| (0 <= i && i < (!v).len()) ==> ((!v)[i] == 0))
)]
fn all_zero(v: &mut Vec<usize>) {
    for x in v.iter_mut() {
        *x = 0;
    }
}
