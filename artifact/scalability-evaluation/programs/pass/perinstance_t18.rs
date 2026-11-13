//@check-pass
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant perinstance, the instantiation at S18 alone; the variant at size k is t0 .. t(k-1).

#[thrust_macros::context]
trait Step {
    #[thrust_macros::requires(Self::inv(x))]
    #[thrust_macros::ensures(Self::inv(result) && result > x)]
    fn step(&self, x: i64) -> i64;

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool;
}

#[derive(PartialEq)]
struct S18 {}

impl thrust_models::Model for S18 {
    type Ty = S18;
}

#[thrust_macros::context]
impl Step for S18 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 18
    }
}

#[thrust_macros::requires(S18::inv(x) && n >= 0)]
#[thrust_macros::ensures(S18::inv(result) && result >= x + n)]
fn repeat_s18(a: &S18, x: i64, n: i64) -> i64 {
    let mut v = x;
    let mut i = 0;
    while i < n {
        v = a.step(v);
        i += 1;
    }
    v
}

fn main() {
    let r_s18 = repeat_s18(&S18 {}, 18, 3);
    assert!(r_s18 >= 21);
}
