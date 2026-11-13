//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant nested, k = 1. Fail twin: the loop of `repeat` stops one step early, so its body breaks its postcondition.

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
    while i + 1 < n {
        v = a.step(v);
        i += 1;
    }
    v
}

#[derive(PartialEq)]
struct Base {}

impl thrust_models::Model for Base {
    type Ty = Base;
}

#[thrust_macros::context]
impl Step for Base {
    fn step(&self, x: i64) -> i64 {
        x + 1
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        x >= 0
    }
}

#[derive(PartialEq)]
struct W0<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W0<T> {
    type Ty = W0<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W0<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 1
    }
}

fn main() {
    let c0 = W0(Base {});
    let r_all = repeat(&c0, 1, 3);
    assert!(r_all >= 4);
}
