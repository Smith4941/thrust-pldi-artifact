//@check-pass
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

fn main() {
    let r0 = accumulate(&I8 {}, -128, 1);
    assert!(r0 >= -128);
}
