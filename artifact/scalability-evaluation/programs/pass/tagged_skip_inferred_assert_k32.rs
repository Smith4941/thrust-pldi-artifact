//@check-pass
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_FOR_LOOP_HISTORY=1 THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant tagged_skip_inferred_assert, k = 32.

use thrust_models::model::{Mut, Seq};
use thrust_models::{exists, FnParam, Ghost, Model};

#[thrust_macros::context]
#[thrust_macros::requires(I::inv(it))]
#[thrust_macros::ensures(exists(|s: Seq<<I::Item as Model>::Ty>, last: I::Ty, fin: I::Ty|
    I::produces(it, s, last) && I::completed(Mut::new(last, fin)) && result == s.len()))]
fn count<I: IteratorSpec>(it: I) -> usize
where
    I::Item: Model,
{
    let mut c = 0;
    for _x in it {
        c += 1;
    }
    c
}

#[derive(PartialEq)]
struct W<I, M>(I, M);

impl<I: Model, M: Model> Model for W<I, M> {
    type Ty = W<I::Ty, M::Ty>;
}

impl<I: IteratorSpec, M> Iterator for W<I, M>
where
    I::Item: Model,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.0.next()
    }
}

#[thrust_macros::context]
impl<I: IteratorSpec, M: Model> IteratorSpec for W<I, M>
where
    I::Item: Model,
{
    #[thrust_macros::predicate]
    fn inv(self) -> bool {
        I::inv(self.0)
    }

    #[thrust_macros::predicate]
    fn produces(self, visited: Vec<I::Item>, o: Self) -> bool {
        I::produces(self.0, visited, o.0)
    }

    #[thrust_macros::predicate]
    fn completed(&mut self) -> bool {
        I::completed(Mut::new((*self).0, (!self).0))
    }

    fn produces_refl(a: &Self) {}

    fn produces_trans(
        a: &Self,
        ab: Seq<<Self::Item as Model>::Ty>,
        b: &Self,
        bc: Seq<<Self::Item as Model>::Ty>,
        c: &Self,
    ) {
    }
}

#[derive(PartialEq)]
struct Tag0;

impl Model for Tag0 {
    type Ty = Tag0;
}

#[derive(PartialEq)]
struct Tag1;

impl Model for Tag1 {
    type Ty = Tag1;
}

#[derive(PartialEq)]
struct Tag2;

impl Model for Tag2 {
    type Ty = Tag2;
}

#[derive(PartialEq)]
struct Tag3;

impl Model for Tag3 {
    type Ty = Tag3;
}

#[derive(PartialEq)]
struct Tag4;

impl Model for Tag4 {
    type Ty = Tag4;
}

#[derive(PartialEq)]
struct Tag5;

impl Model for Tag5 {
    type Ty = Tag5;
}

#[derive(PartialEq)]
struct Tag6;

impl Model for Tag6 {
    type Ty = Tag6;
}

#[derive(PartialEq)]
struct Tag7;

impl Model for Tag7 {
    type Ty = Tag7;
}

#[derive(PartialEq)]
struct Tag8;

impl Model for Tag8 {
    type Ty = Tag8;
}

#[derive(PartialEq)]
struct Tag9;

impl Model for Tag9 {
    type Ty = Tag9;
}

#[derive(PartialEq)]
struct Tag10;

impl Model for Tag10 {
    type Ty = Tag10;
}

#[derive(PartialEq)]
struct Tag11;

impl Model for Tag11 {
    type Ty = Tag11;
}

#[derive(PartialEq)]
struct Tag12;

impl Model for Tag12 {
    type Ty = Tag12;
}

#[derive(PartialEq)]
struct Tag13;

impl Model for Tag13 {
    type Ty = Tag13;
}

#[derive(PartialEq)]
struct Tag14;

impl Model for Tag14 {
    type Ty = Tag14;
}

#[derive(PartialEq)]
struct Tag15;

impl Model for Tag15 {
    type Ty = Tag15;
}

#[derive(PartialEq)]
struct Tag16;

impl Model for Tag16 {
    type Ty = Tag16;
}

#[derive(PartialEq)]
struct Tag17;

impl Model for Tag17 {
    type Ty = Tag17;
}

#[derive(PartialEq)]
struct Tag18;

impl Model for Tag18 {
    type Ty = Tag18;
}

#[derive(PartialEq)]
struct Tag19;

impl Model for Tag19 {
    type Ty = Tag19;
}

#[derive(PartialEq)]
struct Tag20;

impl Model for Tag20 {
    type Ty = Tag20;
}

#[derive(PartialEq)]
struct Tag21;

impl Model for Tag21 {
    type Ty = Tag21;
}

#[derive(PartialEq)]
struct Tag22;

impl Model for Tag22 {
    type Ty = Tag22;
}

#[derive(PartialEq)]
struct Tag23;

impl Model for Tag23 {
    type Ty = Tag23;
}

#[derive(PartialEq)]
struct Tag24;

impl Model for Tag24 {
    type Ty = Tag24;
}

#[derive(PartialEq)]
struct Tag25;

impl Model for Tag25 {
    type Ty = Tag25;
}

#[derive(PartialEq)]
struct Tag26;

impl Model for Tag26 {
    type Ty = Tag26;
}

#[derive(PartialEq)]
struct Tag27;

impl Model for Tag27 {
    type Ty = Tag27;
}

#[derive(PartialEq)]
struct Tag28;

impl Model for Tag28 {
    type Ty = Tag28;
}

#[derive(PartialEq)]
struct Tag29;

impl Model for Tag29 {
    type Ty = Tag29;
}

#[derive(PartialEq)]
struct Tag30;

impl Model for Tag30 {
    type Ty = Tag30;
}

#[derive(PartialEq)]
struct Tag31;

impl Model for Tag31 {
    type Ty = Tag31;
}

fn use_0() {
    assert!(count(W((0..10i64).skip(2), Tag0)) == 8);
}

fn use_1() {
    assert!(count(W((0..10i64).skip(2), Tag1)) == 8);
}

fn use_2() {
    assert!(count(W((0..10i64).skip(2), Tag2)) == 8);
}

fn use_3() {
    assert!(count(W((0..10i64).skip(2), Tag3)) == 8);
}

fn use_4() {
    assert!(count(W((0..10i64).skip(2), Tag4)) == 8);
}

fn use_5() {
    assert!(count(W((0..10i64).skip(2), Tag5)) == 8);
}

fn use_6() {
    assert!(count(W((0..10i64).skip(2), Tag6)) == 8);
}

fn use_7() {
    assert!(count(W((0..10i64).skip(2), Tag7)) == 8);
}

fn use_8() {
    assert!(count(W((0..10i64).skip(2), Tag8)) == 8);
}

fn use_9() {
    assert!(count(W((0..10i64).skip(2), Tag9)) == 8);
}

fn use_10() {
    assert!(count(W((0..10i64).skip(2), Tag10)) == 8);
}

fn use_11() {
    assert!(count(W((0..10i64).skip(2), Tag11)) == 8);
}

fn use_12() {
    assert!(count(W((0..10i64).skip(2), Tag12)) == 8);
}

fn use_13() {
    assert!(count(W((0..10i64).skip(2), Tag13)) == 8);
}

fn use_14() {
    assert!(count(W((0..10i64).skip(2), Tag14)) == 8);
}

fn use_15() {
    assert!(count(W((0..10i64).skip(2), Tag15)) == 8);
}

fn use_16() {
    assert!(count(W((0..10i64).skip(2), Tag16)) == 8);
}

fn use_17() {
    assert!(count(W((0..10i64).skip(2), Tag17)) == 8);
}

fn use_18() {
    assert!(count(W((0..10i64).skip(2), Tag18)) == 8);
}

fn use_19() {
    assert!(count(W((0..10i64).skip(2), Tag19)) == 8);
}

fn use_20() {
    assert!(count(W((0..10i64).skip(2), Tag20)) == 8);
}

fn use_21() {
    assert!(count(W((0..10i64).skip(2), Tag21)) == 8);
}

fn use_22() {
    assert!(count(W((0..10i64).skip(2), Tag22)) == 8);
}

fn use_23() {
    assert!(count(W((0..10i64).skip(2), Tag23)) == 8);
}

fn use_24() {
    assert!(count(W((0..10i64).skip(2), Tag24)) == 8);
}

fn use_25() {
    assert!(count(W((0..10i64).skip(2), Tag25)) == 8);
}

fn use_26() {
    assert!(count(W((0..10i64).skip(2), Tag26)) == 8);
}

fn use_27() {
    assert!(count(W((0..10i64).skip(2), Tag27)) == 8);
}

fn use_28() {
    assert!(count(W((0..10i64).skip(2), Tag28)) == 8);
}

fn use_29() {
    assert!(count(W((0..10i64).skip(2), Tag29)) == 8);
}

fn use_30() {
    assert!(count(W((0..10i64).skip(2), Tag30)) == 8);
}

fn use_31() {
    assert!(count(W((0..10i64).skip(2), Tag31)) == 8);
}

fn main() {
    use_0();
    use_1();
    use_2();
    use_3();
    use_4();
    use_5();
    use_6();
    use_7();
    use_8();
    use_9();
    use_10();
    use_11();
    use_12();
    use_13();
    use_14();
    use_15();
    use_16();
    use_17();
    use_18();
    use_19();
    use_20();
    use_21();
    use_22();
    use_23();
    use_24();
    use_25();
    use_26();
    use_27();
    use_28();
    use_29();
    use_30();
    use_31();
}
