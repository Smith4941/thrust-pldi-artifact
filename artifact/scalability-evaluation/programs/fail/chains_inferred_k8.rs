//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_FOR_LOOP_HISTORY=1 THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant chains_inferred, k = 8. Fail twin: the loop of `count` adds 2 per item, so its body breaks its postcondition.

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
        c += 2;
    }
    c
}

fn use_0() {
    count(0..10i64);
}

fn use_1() {
    count((0..10i64).take(3));
}

fn use_2() {
    count((0..10i64).rev());
}

fn use_3() {
    count((0..10i64).by_ref());
}

fn use_4() {
    count((0..10i64).enumerate());
}

fn use_5() {
    count((0..10i64).skip(2));
}

fn use_6() {
    count((0..10i64).skip(2).take(3));
}

fn use_7() {
    count(std::iter::zip(0..10i64, 3..5i64));
}

fn main() {
    use_0();
    use_1();
    use_2();
    use_3();
    use_4();
    use_5();
    use_6();
    use_7();
}
