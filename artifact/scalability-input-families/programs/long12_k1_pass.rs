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

#[thrust_macros::requires(T::inv(x))]
#[thrust_macros::ensures(T::inv(result) && result >= x + 12)]
fn run<T: Step>(a: &T, x: i64) -> i64 {
    let v0 = x;
    let v1 = a.step(v0);
    let d1 = v1 - x;
    assert!(d1 >= 1);
    let v2 = a.step(v1);
    let d2 = v2 - x;
    assert!(d2 >= 2);
    let v3 = a.step(v2);
    let d3 = v3 - x;
    assert!(d3 >= 3);
    let v4 = a.step(v3);
    let d4 = v4 - x;
    assert!(d4 >= 4);
    let v5 = a.step(v4);
    let d5 = v5 - x;
    assert!(d5 >= 5);
    let v6 = a.step(v5);
    let d6 = v6 - x;
    assert!(d6 >= 6);
    let v7 = a.step(v6);
    let d7 = v7 - x;
    assert!(d7 >= 7);
    let v8 = a.step(v7);
    let d8 = v8 - x;
    assert!(d8 >= 8);
    let v9 = a.step(v8);
    let d9 = v9 - x;
    assert!(d9 >= 9);
    let v10 = a.step(v9);
    let d10 = v10 - x;
    assert!(d10 >= 10);
    let v11 = a.step(v10);
    let d11 = v11 - x;
    assert!(d11 >= 11);
    let v12 = a.step(v11);
    let d12 = v12 - x;
    assert!(d12 >= 12);
    v12
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
    let r0 = run(&S0 {}, 0);
    assert!(r0 >= 12);
}
