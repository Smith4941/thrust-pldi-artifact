//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant chains_assert, k = 8. Fail twin: `use_7` asserts one item more than its iterator produces.

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
        thrust_macros::invariant!(|iter: I, iter_old: Ghost<I>, produced: Ghost<Seq<<I::Item as Model>::Ty>>, c: usize, it: FnParam<I>|
            iter_old == it.at_entry() && c == produced.len());
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
    assert!(count((0..10i64).by_ref()) == 10);
}

fn use_4() {
    assert!(count((0..10i64).enumerate()) == 10);
}

fn use_5() {
    assert!(count((0..10i64).skip(2)) == 8);
}

fn use_6() {
    assert!(count((0..10i64).skip(2).take(3)) == 3);
}

fn use_7() {
    assert!(count(std::iter::zip(0..10i64, 3..5i64)) == 3);
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
