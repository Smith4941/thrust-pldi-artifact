//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant nested, k = 32. Fail twin: the loop of `repeat` stops one step early, so its body breaks its postcondition.

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

#[derive(PartialEq)]
struct W16<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W16<T> {
    type Ty = W16<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W16<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 17
    }
}

#[derive(PartialEq)]
struct W17<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W17<T> {
    type Ty = W17<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W17<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 18
    }
}

#[derive(PartialEq)]
struct W18<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W18<T> {
    type Ty = W18<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W18<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 19
    }
}

#[derive(PartialEq)]
struct W19<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W19<T> {
    type Ty = W19<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W19<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 20
    }
}

#[derive(PartialEq)]
struct W20<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W20<T> {
    type Ty = W20<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W20<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 21
    }
}

#[derive(PartialEq)]
struct W21<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W21<T> {
    type Ty = W21<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W21<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 22
    }
}

#[derive(PartialEq)]
struct W22<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W22<T> {
    type Ty = W22<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W22<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 23
    }
}

#[derive(PartialEq)]
struct W23<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W23<T> {
    type Ty = W23<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W23<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 24
    }
}

#[derive(PartialEq)]
struct W24<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W24<T> {
    type Ty = W24<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W24<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 25
    }
}

#[derive(PartialEq)]
struct W25<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W25<T> {
    type Ty = W25<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W25<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 26
    }
}

#[derive(PartialEq)]
struct W26<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W26<T> {
    type Ty = W26<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W26<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 27
    }
}

#[derive(PartialEq)]
struct W27<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W27<T> {
    type Ty = W27<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W27<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 28
    }
}

#[derive(PartialEq)]
struct W28<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W28<T> {
    type Ty = W28<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W28<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 29
    }
}

#[derive(PartialEq)]
struct W29<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W29<T> {
    type Ty = W29<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W29<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 30
    }
}

#[derive(PartialEq)]
struct W30<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W30<T> {
    type Ty = W30<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W30<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 31
    }
}

#[derive(PartialEq)]
struct W31<T>(T);

impl<T: thrust_models::Model> thrust_models::Model for W31<T> {
    type Ty = W31<<T as thrust_models::Model>::Ty>;
}

#[thrust_macros::context]
impl<T: Step + thrust_models::Model> Step for W31<T>
where
    <T as thrust_models::Model>::Ty: PartialEq,
{
    fn step(&self, x: i64) -> i64 {
        self.0.step(x)
    }

    #[thrust_macros::predicate]
    fn inv(x: i64) -> bool {
        T::inv(x) && x >= 32
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
    let c16 = W16(c15);
    let c17 = W17(c16);
    let c18 = W18(c17);
    let c19 = W19(c18);
    let c20 = W20(c19);
    let c21 = W21(c20);
    let c22 = W22(c21);
    let c23 = W23(c22);
    let c24 = W24(c23);
    let c25 = W25(c24);
    let c26 = W26(c25);
    let c27 = W27(c26);
    let c28 = W28(c27);
    let c29 = W29(c28);
    let c30 = W30(c29);
    let c31 = W31(c30);
    let r_all = repeat(&c31, 32, 3);
    assert!(r_all >= 35);
}
