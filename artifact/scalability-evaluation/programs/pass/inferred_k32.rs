//@check-pass
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant inferred, k = 32.

#[thrust_macros::context]
trait Step {
    #[thrust_macros::requires(Self::inv(x))]
    #[thrust_macros::ensures(Self::inv(result) && result > x)]
    fn step(&self, x: i64) -> i64;

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool;
}

#[thrust_macros::requires(T::inv(x) && n >= 0)]
#[thrust_macros::ensures(T::inv(result) && result >= x + n)]
fn repeat<T: Step>(a: &T, x: i64, n: i64) -> i64 {
    let mut v = x;
    let mut i = 0;
    while i < n {
        v = a.step(v);
        i += 1;
    }
    v
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

#[derive(PartialEq)]
struct S8 {}

impl thrust_models::Model for S8 {
    type Ty = S8;
}

#[thrust_macros::context]
impl Step for S8 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 8
    }
}

#[derive(PartialEq)]
struct S9 {}

impl thrust_models::Model for S9 {
    type Ty = S9;
}

#[thrust_macros::context]
impl Step for S9 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 9
    }
}

#[derive(PartialEq)]
struct S10 {}

impl thrust_models::Model for S10 {
    type Ty = S10;
}

#[thrust_macros::context]
impl Step for S10 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 10
    }
}

#[derive(PartialEq)]
struct S11 {}

impl thrust_models::Model for S11 {
    type Ty = S11;
}

#[thrust_macros::context]
impl Step for S11 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 11
    }
}

#[derive(PartialEq)]
struct S12 {}

impl thrust_models::Model for S12 {
    type Ty = S12;
}

#[thrust_macros::context]
impl Step for S12 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 12
    }
}

#[derive(PartialEq)]
struct S13 {}

impl thrust_models::Model for S13 {
    type Ty = S13;
}

#[thrust_macros::context]
impl Step for S13 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 13
    }
}

#[derive(PartialEq)]
struct S14 {}

impl thrust_models::Model for S14 {
    type Ty = S14;
}

#[thrust_macros::context]
impl Step for S14 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 14
    }
}

#[derive(PartialEq)]
struct S15 {}

impl thrust_models::Model for S15 {
    type Ty = S15;
}

#[thrust_macros::context]
impl Step for S15 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 15
    }
}

#[derive(PartialEq)]
struct S16 {}

impl thrust_models::Model for S16 {
    type Ty = S16;
}

#[thrust_macros::context]
impl Step for S16 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 16
    }
}

#[derive(PartialEq)]
struct S17 {}

impl thrust_models::Model for S17 {
    type Ty = S17;
}

#[thrust_macros::context]
impl Step for S17 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 17
    }
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

#[derive(PartialEq)]
struct S19 {}

impl thrust_models::Model for S19 {
    type Ty = S19;
}

#[thrust_macros::context]
impl Step for S19 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 19
    }
}

#[derive(PartialEq)]
struct S20 {}

impl thrust_models::Model for S20 {
    type Ty = S20;
}

#[thrust_macros::context]
impl Step for S20 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 20
    }
}

#[derive(PartialEq)]
struct S21 {}

impl thrust_models::Model for S21 {
    type Ty = S21;
}

#[thrust_macros::context]
impl Step for S21 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 21
    }
}

#[derive(PartialEq)]
struct S22 {}

impl thrust_models::Model for S22 {
    type Ty = S22;
}

#[thrust_macros::context]
impl Step for S22 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 22
    }
}

#[derive(PartialEq)]
struct S23 {}

impl thrust_models::Model for S23 {
    type Ty = S23;
}

#[thrust_macros::context]
impl Step for S23 {
    fn step(&self, x: i64) -> i64 {
        x + 3
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 23
    }
}

#[derive(PartialEq)]
struct S24 {}

impl thrust_models::Model for S24 {
    type Ty = S24;
}

#[thrust_macros::context]
impl Step for S24 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 24
    }
}

#[derive(PartialEq)]
struct S25 {}

impl thrust_models::Model for S25 {
    type Ty = S25;
}

#[thrust_macros::context]
impl Step for S25 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 25
    }
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

#[derive(PartialEq)]
struct S27 {}

impl thrust_models::Model for S27 {
    type Ty = S27;
}

#[thrust_macros::context]
impl Step for S27 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 27
    }
}

#[derive(PartialEq)]
struct S28 {}

impl thrust_models::Model for S28 {
    type Ty = S28;
}

#[thrust_macros::context]
impl Step for S28 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 28
    }
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

#[derive(PartialEq)]
struct S30 {}

impl thrust_models::Model for S30 {
    type Ty = S30;
}

#[thrust_macros::context]
impl Step for S30 {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 30
    }
}

#[derive(PartialEq)]
struct S31 {}

impl thrust_models::Model for S31 {
    type Ty = S31;
}

#[thrust_macros::context]
impl Step for S31 {
    fn step(&self, x: i64) -> i64 {
        x + 2
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 31
    }
}

fn main() {
    let r_s0 = repeat(&S0 {}, 0, 3);
    assert!(r_s0 >= 3);
    let r_s1 = repeat(&S1 {}, 1, 3);
    assert!(r_s1 >= 4);
    let r_s2 = repeat(&S2 {}, 2, 3);
    assert!(r_s2 >= 5);
    let r_s3 = repeat(&S3 {}, 3, 3);
    assert!(r_s3 >= 6);
    let r_s4 = repeat(&S4 {}, 4, 3);
    assert!(r_s4 >= 7);
    let r_s5 = repeat(&S5 {}, 5, 3);
    assert!(r_s5 >= 8);
    let r_s6 = repeat(&S6 {}, 6, 3);
    assert!(r_s6 >= 9);
    let r_s7 = repeat(&S7 {}, 7, 3);
    assert!(r_s7 >= 10);
    let r_s8 = repeat(&S8 {}, 8, 3);
    assert!(r_s8 >= 11);
    let r_s9 = repeat(&S9 {}, 9, 3);
    assert!(r_s9 >= 12);
    let r_s10 = repeat(&S10 {}, 10, 3);
    assert!(r_s10 >= 13);
    let r_s11 = repeat(&S11 {}, 11, 3);
    assert!(r_s11 >= 14);
    let r_s12 = repeat(&S12 {}, 12, 3);
    assert!(r_s12 >= 15);
    let r_s13 = repeat(&S13 {}, 13, 3);
    assert!(r_s13 >= 16);
    let r_s14 = repeat(&S14 {}, 14, 3);
    assert!(r_s14 >= 17);
    let r_s15 = repeat(&S15 {}, 15, 3);
    assert!(r_s15 >= 18);
    let r_s16 = repeat(&S16 {}, 16, 3);
    assert!(r_s16 >= 19);
    let r_s17 = repeat(&S17 {}, 17, 3);
    assert!(r_s17 >= 20);
    let r_s18 = repeat(&S18 {}, 18, 3);
    assert!(r_s18 >= 21);
    let r_s19 = repeat(&S19 {}, 19, 3);
    assert!(r_s19 >= 22);
    let r_s20 = repeat(&S20 {}, 20, 3);
    assert!(r_s20 >= 23);
    let r_s21 = repeat(&S21 {}, 21, 3);
    assert!(r_s21 >= 24);
    let r_s22 = repeat(&S22 {}, 22, 3);
    assert!(r_s22 >= 25);
    let r_s23 = repeat(&S23 {}, 23, 3);
    assert!(r_s23 >= 26);
    let r_s24 = repeat(&S24 {}, 24, 3);
    assert!(r_s24 >= 27);
    let r_s25 = repeat(&S25 {}, 25, 3);
    assert!(r_s25 >= 28);
    let r_s26 = repeat(&S26 {}, 26, 3);
    assert!(r_s26 >= 29);
    let r_s27 = repeat(&S27 {}, 27, 3);
    assert!(r_s27 >= 30);
    let r_s28 = repeat(&S28 {}, 28, 3);
    assert!(r_s28 >= 31);
    let r_s29 = repeat(&S29 {}, 29, 3);
    assert!(r_s29 >= 32);
    let r_s30 = repeat(&S30 {}, 30, 3);
    assert!(r_s30 >= 33);
    let r_s31 = repeat(&S31 {}, 31, 3);
    assert!(r_s31 >= 34);
}
