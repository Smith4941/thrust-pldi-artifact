//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_FOR_LOOP_HISTORY=1 THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant chains_inferred_assert, k = 4. Fail twin: `use_3` asserts one item more than its iterator produces.

use thrust_models::model::{Mut, Seq};
use thrust_models::{exists, FnParam, Ghost, Model};

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
        c += 1;
    }
    c
}

fn use_0() {
    assert!(count(0..10i64) == 10);
}

fn use_1() {
    assert!(count((0..10i64).take(3)) == 3);
}

fn use_2() {
    assert!(count((0..10i64).rev()) == 10);
}

fn use_3() {
    assert!(count((0..10i64).by_ref()) == 11);
}

fn main() {
    use_0();
    use_1();
    use_2();
    use_3();
}
