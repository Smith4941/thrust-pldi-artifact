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

#[thrust_macros::requires(T::inv(x))]
#[thrust_macros::ensures(T::inv(result) && result >= x + 8)]
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
    v8
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

#[derive(PartialEq)]
struct S4 {}

impl thrust_models::Model for S4 {
    type Ty = S4;
}

#[thrust_macros::context]
impl Step for S4 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 4
    }
}

#[derive(PartialEq)]
struct S5 {}

impl thrust_models::Model for S5 {
    type Ty = S5;
}

#[thrust_macros::context]
impl Step for S5 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 5
    }
}

#[derive(PartialEq)]
struct S6 {}

impl thrust_models::Model for S6 {
    type Ty = S6;
}

#[thrust_macros::context]
impl Step for S6 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 6
    }
}

#[derive(PartialEq)]
struct S7 {}

impl thrust_models::Model for S7 {
    type Ty = S7;
}

#[thrust_macros::context]
impl Step for S7 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 7
    }
}

fn main() {
    let r0 = run(&S0 {}, 0);
    assert!(r0 >= 8);
    let r1 = run(&S1 {}, 1);
    assert!(r1 >= 9);
    let r2 = run(&S2 {}, 2);
    assert!(r2 >= 10);
    let r3 = run(&S3 {}, 3);
    assert!(r3 >= 11);
    let r4 = run(&S4 {}, 4);
    assert!(r4 >= 12);
    let r5 = run(&S5 {}, 5);
    assert!(r5 >= 13);
    let r6 = run(&S6 {}, 6);
    assert!(r6 >= 14);
    let r7 = run(&S7 {}, 6);
    assert!(r7 >= 15);
}
