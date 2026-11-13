//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper
#[thrust_macros::context]
trait SatInt {
    #[thrust_macros::requires(Self::ok(x) && Self::ok(y))]
    #[thrust_macros::ensures(Self::ok(result) && (Self::ok(x + y) ==> result == x + y) && (y >= 0 ==> result >= x))]
    fn sat_add(&self, x: i64, y: i64) -> i64;

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool;
}

#[thrust_macros::requires(T::ok(x) && T::ok(d) && d >= 0)]
#[thrust_macros::ensures(T::ok(result) && result >= x)]
fn accumulate<T: SatInt>(a: &T, x: i64, d: i64) -> i64 {
    let v0 = x;
    let v1 = a.sat_add(v0, d);
    assert!(v1 >= x);
    let v2 = a.sat_add(v1, d);
    assert!(v2 >= x);
    let v3 = a.sat_add(v2, d);
    assert!(v3 >= x);
    let v4 = a.sat_add(v3, d);
    assert!(v4 >= x);
    let v5 = a.sat_add(v4, d);
    assert!(v5 >= x);
    let v6 = a.sat_add(v5, d);
    assert!(v6 >= x);
    let v7 = a.sat_add(v6, d);
    assert!(v7 >= x);
    let v8 = a.sat_add(v7, d);
    assert!(v8 >= x);
    v8
}

#[derive(PartialEq)]
struct I8 {}

impl thrust_models::Model for I8 {
    type Ty = I8;
}

#[thrust_macros::context]
impl SatInt for I8 {
    fn sat_add(&self, x: i64, y: i64) -> i64 {
        let s = x + y;
        if s > 127 {
            127
        } else if s < -128 {
            -128
        } else {
            s
        }
    }

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool {
        -128 <= x && x <= 127
    }
}

#[derive(PartialEq)]
struct U8 {}

impl thrust_models::Model for U8 {
    type Ty = U8;
}

#[thrust_macros::context]
impl SatInt for U8 {
    fn sat_add(&self, x: i64, y: i64) -> i64 {
        let s = x + y;
        if s > 255 {
            255
        } else if s < 0 {
            0
        } else {
            s
        }
    }

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool {
        0 <= x && x <= 255
    }
}

#[derive(PartialEq)]
struct I16 {}

impl thrust_models::Model for I16 {
    type Ty = I16;
}

#[thrust_macros::context]
impl SatInt for I16 {
    fn sat_add(&self, x: i64, y: i64) -> i64 {
        let s = x + y;
        if s > 32767 {
            32767
        } else if s < -32768 {
            -32768
        } else {
            s
        }
    }

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool {
        -32768 <= x && x <= 32767
    }
}

#[derive(PartialEq)]
struct U16 {}

impl thrust_models::Model for U16 {
    type Ty = U16;
}

#[thrust_macros::context]
impl SatInt for U16 {
    fn sat_add(&self, x: i64, y: i64) -> i64 {
        let s = x + y;
        if s > 65535 {
            65535
        } else if s < 0 {
            0
        } else {
            s
        }
    }

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool {
        0 <= x && x <= 65535
    }
}

#[derive(PartialEq)]
struct I32 {}

impl thrust_models::Model for I32 {
    type Ty = I32;
}

#[thrust_macros::context]
impl SatInt for I32 {
    fn sat_add(&self, x: i64, y: i64) -> i64 {
        let s = x + y;
        if s > 2147483647 {
            2147483647
        } else if s < -2147483647 {
            -2147483647
        } else {
            s
        }
    }

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool {
        -2147483647 <= x && x <= 2147483647
    }
}

#[derive(PartialEq)]
struct U31 {}

impl thrust_models::Model for U31 {
    type Ty = U31;
}

#[thrust_macros::context]
impl SatInt for U31 {
    fn sat_add(&self, x: i64, y: i64) -> i64 {
        let s = x + y;
        if s > 2147483647 {
            2147483647
        } else if s < 0 {
            0
        } else {
            s
        }
    }

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool {
        0 <= x && x <= 2147483647
    }
}

#[derive(PartialEq)]
struct Pct {}

impl thrust_models::Model for Pct {
    type Ty = Pct;
}

#[thrust_macros::context]
impl SatInt for Pct {
    fn sat_add(&self, x: i64, y: i64) -> i64 {
        let s = x + y;
        if s > 100 {
            100
        } else if s < 0 {
            0
        } else {
            s
        }
    }

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool {
        0 <= x && x <= 100
    }
}

#[derive(PartialEq)]
struct Deg {}

impl thrust_models::Model for Deg {
    type Ty = Deg;
}

#[thrust_macros::context]
impl SatInt for Deg {
    fn sat_add(&self, x: i64, y: i64) -> i64 {
        let s = x + y;
        if s > 359 {
            359
        } else if s < 0 {
            0
        } else {
            s
        }
    }

    #[thrust_macros::predicate]
    fn ok(x: i64) -> bool {
        0 <= x && x <= 359
    }
}

fn main() {
    let r0 = accumulate(&I8 {}, -128, 1);
    assert!(r0 >= -128);
    let r1 = accumulate(&U8 {}, 0, 1);
    assert!(r1 >= 0);
    let r2 = accumulate(&I16 {}, -32768, 1);
    assert!(r2 >= -32768);
    let r3 = accumulate(&U16 {}, 0, 1);
    assert!(r3 >= 0);
    let r4 = accumulate(&I32 {}, -2147483647, 1);
    assert!(r4 >= -2147483647);
    let r5 = accumulate(&U31 {}, 0, 1);
    assert!(r5 >= 0);
    let r6 = accumulate(&Pct {}, 0, 1);
    assert!(r6 >= 0);
    let r7 = accumulate(&Deg {}, -1, 1);
    assert!(r7 >= 0);
}
