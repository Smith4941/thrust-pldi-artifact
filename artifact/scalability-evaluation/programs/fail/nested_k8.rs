//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant nested, k = 8. Fail twin: the loop of `repeat` stops one step early, so its body breaks its postcondition.

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

#[derive(PartialEq)]
struct W1<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W1<T> {
    type Ty = W1<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W1<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 2
    }
}

#[derive(PartialEq)]
struct W2<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W2<T> {
    type Ty = W2<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W2<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 3
    }
}

#[derive(PartialEq)]
struct W3<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W3<T> {
    type Ty = W3<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W3<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 4
    }
}

#[derive(PartialEq)]
struct W4<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W4<T> {
    type Ty = W4<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W4<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 5
    }
}

#[derive(PartialEq)]
struct W5<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W5<T> {
    type Ty = W5<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W5<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 6
    }
}

#[derive(PartialEq)]
struct W6<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W6<T> {
    type Ty = W6<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W6<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 7
    }
}

#[derive(PartialEq)]
struct W7<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W7<T> {
    type Ty = W7<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W7<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 8
    }
}

fn main() {
    let c0 = W0(Base {});
    let c1 = W1(c0);
    let c2 = W2(c1);
    let c3 = W3(c2);
    let c4 = W4(c3);
    let c5 = W5(c4);
    let c6 = W6(c5);
    let c7 = W7(c6);
    let r_all = repeat(&c7, 8, 3);
    assert!(r_all >= 11);
}
