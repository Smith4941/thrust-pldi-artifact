//@check-pass
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

#[thrust_macros::requires(T::inv(x) && n >= 0)]
#[thrust_macros::ensures(result.len() == n && thrust_models::forall(|k: thrust_models::model::Int|
    !(0 <= k && k < result.len()) || T::inv(result[k])))]
fn run<T: Step>(a: &T, x: i64, n: i64) -> Vec<i64> {
    let mut out = Vec::new();
    let mut v = x;
    let mut i = 0;
    while i < n {
        v = a.step(v);
        out.push(v);
        i += 1;
    }
    out
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

#[derive(PartialEq)]
struct S1 {}

impl thrust_models::Model for S1 {
    type Ty = S1;
}

#[thrust_macros::context]
impl Step for S1 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 1
    }
}

#[derive(PartialEq)]
struct S2 {}

impl thrust_models::Model for S2 {
    type Ty = S2;
}

#[thrust_macros::context]
impl Step for S2 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 2
    }
}

#[derive(PartialEq)]
struct S3 {}

impl thrust_models::Model for S3 {
    type Ty = S3;
}

#[thrust_macros::context]
impl Step for S3 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 3
    }
}

fn main() {
    let r0 = run(&S0 {}, 0, 3);
    assert!(r0.len() == 3);
    let r1 = run(&S1 {}, 1, 3);
    assert!(r1.len() == 3);
    let r2 = run(&S2 {}, 2, 3);
    assert!(r2.len() == 3);
    let r3 = run(&S3 {}, 3, 3);
    assert!(r3.len() == 3);
}
