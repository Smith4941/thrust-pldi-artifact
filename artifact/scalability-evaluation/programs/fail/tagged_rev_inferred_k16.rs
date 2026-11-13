//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_FOR_LOOP_HISTORY=1 THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant tagged_rev_inferred, k = 16. Fail twin: the loop of `count` adds 2 per item, so its body breaks its postcondition.

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
        c += 2;
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

fn use_0() {
    count(W((0..10i64).rev(), Tag0));
}

fn use_1() {
    count(W((0..10i64).rev(), Tag1));
}

fn use_2() {
    count(W((0..10i64).rev(), Tag2));
}

fn use_3() {
    count(W((0..10i64).rev(), Tag3));
}

fn use_4() {
    count(W((0..10i64).rev(), Tag4));
}

fn use_5() {
    count(W((0..10i64).rev(), Tag5));
}

fn use_6() {
    count(W((0..10i64).rev(), Tag6));
}

fn use_7() {
    count(W((0..10i64).rev(), Tag7));
}

fn use_8() {
    count(W((0..10i64).rev(), Tag8));
}

fn use_9() {
    count(W((0..10i64).rev(), Tag9));
}

fn use_10() {
    count(W((0..10i64).rev(), Tag10));
}

fn use_11() {
    count(W((0..10i64).rev(), Tag11));
}

fn use_12() {
    count(W((0..10i64).rev(), Tag12));
}

fn use_13() {
    count(W((0..10i64).rev(), Tag13));
}

fn use_14() {
    count(W((0..10i64).rev(), Tag14));
}

fn use_15() {
    count(W((0..10i64).rev(), Tag15));
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
}
