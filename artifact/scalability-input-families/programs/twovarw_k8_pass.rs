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

#[thrust_macros::context]
#[thrust_macros::requires(T::inv(x) && T::inv(y) && x <= y && n >= 0)]
#[thrust_macros::ensures(T::inv(result.0) && T::inv(result.1) && result.0 >= x + n && result.1 >= y + n)]
fn run<T: Step>(a: &T, x: i64, y: i64, n: i64) -> (i64, i64) {
    let mut v = x;
    let mut w = y;
    let mut i = 0;
    while i < n {
        thrust_macros::invariant!(
            |v: i64, w: i64, i: i64, x: thrust_models::FnParam<i64>, y: thrust_models::FnParam<i64>|
            T::inv(v) && T::inv(w) && v >= x.at_entry() + i && w >= y.at_entry() + i
        );
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
    let r0 = run(&S0 {}, 0, 1, 3);
    assert!(r0.0 >= 3 && r0.1 >= 4);
    let r1 = run(&S1 {}, 1, 2, 3);
    assert!(r1.0 >= 4 && r1.1 >= 5);
    let r2 = run(&S2 {}, 2, 3, 3);
    assert!(r2.0 >= 5 && r2.1 >= 6);
    let r3 = run(&S3 {}, 3, 4, 3);
    assert!(r3.0 >= 6 && r3.1 >= 7);
    let r4 = run(&S4 {}, 4, 5, 3);
    assert!(r4.0 >= 7 && r4.1 >= 8);
    let r5 = run(&S5 {}, 5, 6, 3);
    assert!(r5.0 >= 8 && r5.1 >= 9);
    let r6 = run(&S6 {}, 6, 7, 3);
    assert!(r6.0 >= 9 && r6.1 >= 10);
    let r7 = run(&S7 {}, 7, 8, 3);
    assert!(r7.0 >= 10 && r7.1 >= 11);
}
