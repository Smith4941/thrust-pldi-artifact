//@check-pass
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant chains_assert, k = 32.

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
    assert!(count(std::iter::zip(0..10i64, 3..5i64)) == 2);
}

fn use_8() {
    assert!(count(0..10i32) == 10);
}

fn use_9() {
    assert!(count((0..10i32).take(3)) == 3);
}

fn use_10() {
    assert!(count((0..10i32).rev()) == 10);
}

fn use_11() {
    assert!(count((0..10i32).by_ref()) == 10);
}

fn use_12() {
    assert!(count((0..10i32).enumerate()) == 10);
}

fn use_13() {
    assert!(count((0..10i32).skip(2)) == 8);
}

fn use_14() {
    assert!(count((0..10i32).skip(2).take(3)) == 3);
}

fn use_15() {
    assert!(count(std::iter::zip(0..10i32, 3..5i32)) == 2);
}

fn use_16() {
    assert!(count(0..10i16) == 10);
}

fn use_17() {
    assert!(count((0..10i16).take(3)) == 3);
}

fn use_18() {
    assert!(count((0..10i16).rev()) == 10);
}

fn use_19() {
    assert!(count((0..10i16).by_ref()) == 10);
}

fn use_20() {
    assert!(count((0..10i16).enumerate()) == 10);
}

fn use_21() {
    assert!(count((0..10i16).skip(2)) == 8);
}

fn use_22() {
    assert!(count((0..10i16).skip(2).take(3)) == 3);
}

fn use_23() {
    assert!(count(std::iter::zip(0..10i16, 3..5i16)) == 2);
}

fn use_24() {
    assert!(count(0..10i8) == 10);
}

fn use_25() {
    assert!(count((0..10i8).take(3)) == 3);
}

fn use_26() {
    assert!(count((0..10i8).rev()) == 10);
}

fn use_27() {
    assert!(count((0..10i8).by_ref()) == 10);
}

fn use_28() {
    assert!(count((0..10i8).enumerate()) == 10);
}

fn use_29() {
    assert!(count((0..10i8).skip(2)) == 8);
}

fn use_30() {
    assert!(count((0..10i8).skip(2).take(3)) == 3);
}

fn use_31() {
    assert!(count(std::iter::zip(0..10i8, 3..5i8)) == 2);
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
    use_8();
    use_9();
    use_10();
    use_11();
    use_12();
    use_13();
    use_14();
    use_15();
    use_16();
    use_17();
    use_18();
    use_19();
    use_20();
    use_21();
    use_22();
    use_23();
    use_24();
    use_25();
    use_26();
    use_27();
    use_28();
    use_29();
    use_30();
    use_31();
}
