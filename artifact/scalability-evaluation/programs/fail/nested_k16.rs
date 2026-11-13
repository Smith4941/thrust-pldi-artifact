//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant nested, k = 16. Fail twin: the loop of `repeat` stops one step early, so its body breaks its postcondition.

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

#[derive(PartialEq)]
struct W8<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W8<T> {
    type Ty = W8<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W8<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 9
    }
}

#[derive(PartialEq)]
struct W9<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W9<T> {
    type Ty = W9<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W9<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 10
    }
}

#[derive(PartialEq)]
struct W10<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W10<T> {
    type Ty = W10<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W10<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 11
    }
}

#[derive(PartialEq)]
struct W11<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W11<T> {
    type Ty = W11<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W11<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 12
    }
}

#[derive(PartialEq)]
struct W12<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W12<T> {
    type Ty = W12<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W12<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 13
    }
}

#[derive(PartialEq)]
struct W13<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W13<T> {
    type Ty = W13<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W13<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 14
    }
}

#[derive(PartialEq)]
struct W14<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W14<T> {
    type Ty = W14<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W14<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 15
    }
}

#[derive(PartialEq)]
struct W15<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W15<T> {
    type Ty = W15<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W15<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 16
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
    let c8 = W8(c7);
    let c9 = W9(c8);
    let c10 = W10(c9);
    let c11 = W11(c10);
    let c12 = W12(c11);
    let c13 = W13(c12);
    let c14 = W14(c13);
    let c15 = W15(c14);
    let r_all = repeat(&c15, 16, 3);
    assert!(r_all >= 19);
}
