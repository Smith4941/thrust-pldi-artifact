//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant perinstance, the instantiation at S29 alone; the variant at size k is t0 .. t(k-1). Fail twin: the loop of `repeat_s29` stops one step early, so its body breaks its postcondition.

#[thrust_macros::context]
trait Step {
    #[thrust_macros::requires(Self::inv(x))]
    #[thrust_macros::ensures(Self::inv(result) && result > x)]
    fn step(&self, x: i64) -> i64;

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool;
}

#[derive(PartialEq)]
struct S29 {}

impl thrust_models::Model for S29 {
    type Ty = S29;
}

#[thrust_macros::context]
impl Step for S29 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 29
    }
}

#[thrust_macros::requires(S29::inv(x) && n >= 0)]
#[thrust_macros::ensures(S29::inv(result) && result >= x + n)]
fn repeat_s29(a: &S29, x: i64, n: i64) -> i64 {
    let mut v = x;
    let mut i = 0;
    while i + 1 < n {
        v = a.step(v);
        i += 1;
    }
    v
}

fn main() {
    let r_s29 = repeat_s29(&S29 {}, 29, 3);
    assert!(r_s29 >= 32);
}
