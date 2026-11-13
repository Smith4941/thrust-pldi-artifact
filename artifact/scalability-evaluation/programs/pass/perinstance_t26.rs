//@check-pass
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant perinstance, the instantiation at S26 alone; the variant at size k is t0 .. t(k-1).

#[thrust_macros::context]
trait Step {
    #[thrust_macros::requires(Self::inv(x))]
    #[thrust_macros::ensures(Self::inv(result) && result > x)]
    fn step(&self, x: i64) -> i64;

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool;
}

#[derive(PartialEq)]
struct S26 {}

impl thrust_models::Model for S26 {
    type Ty = S26;
}

#[thrust_macros::context]
impl Step for S26 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 26
    }
}

#[thrust_macros::requires(S26::inv(x) && n >= 0)]
#[thrust_macros::ensures(S26::inv(result) && result >= x + n)]
fn repeat_s26(a: &S26, x: i64, n: i64) -> i64 {
    let mut v = x;
    let mut i = 0;
    while i < n {
        v = a.step(v);
        i += 1;
    }
    v
}

fn main() {
    let r_s26 = repeat_s26(&S26 {}, 26, 3);
    assert!(r_s26 >= 29);
}
