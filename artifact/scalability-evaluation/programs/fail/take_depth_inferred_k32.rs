//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_FOR_LOOP_HISTORY=1 THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant take_depth_inferred, k = 32. Fail twin: the loop of `count` adds 2 per item, so its body breaks its postcondition.

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
    count((0..100i64).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1).take(1));
}

fn main() {
    use_0();
}
