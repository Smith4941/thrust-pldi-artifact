//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper
#[thrust_macros::context]
trait Step {
    #[thrust_macros::requires(Self::inv(x))]
    #[thrust_macros::ensures(Self::inv(result) && result > x)]
    fn step(&self, x: i64) -> i64;

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool;
}

#[thrust_macros::requires(T::inv(x) && T::inv(y) && x <= y && n >= 0)]
#[thrust_macros::ensures(T::inv(result.0) && T::inv(result.1) && result.0 >= x + n && result.1 >= y + n)]
fn run<T: Step>(a: &T, x: i64, y: i64, n: i64) -> (i64, i64) {
    let mut v = x;
    let mut w = y;
    let mut i = 0;
    while i < n {
        v = a.step(v);
        w = a.step(w);
        i += 1;
    }
    (v, w)
}

#[derive(PartialEq)]
struct S0 {}

impl thrust_models::Model for S0 {
    type Ty = S0;
}

#[thrust_macros::context]
impl Step for S0 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 0
    }
}

fn main() {
    let r0 = run(&S0 {}, -1, 1, 3);
    assert!(r0.0 >= 3 && r0.1 >= 4);
}
