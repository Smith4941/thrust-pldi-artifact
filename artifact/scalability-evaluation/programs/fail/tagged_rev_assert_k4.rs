//@error-in-other-file: Unsat
//@compile-flags: -C debug-assertions=off
//@rustc-env: THRUST_SOLVER=tests/thrust-pcsat-wrapper THRUST_SOLVER_TIMEOUT_SECS=300

// Scalability evaluation, variant tagged_rev_assert, k = 4. Fail twin: `use_3` asserts one item more than its iterator produces.

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
        thrust_macros::invariant!(|iter: I, iter_old: Ghost<I>, produced: Ghost<Seq<<I::Item as Model>::Ty>>, c: usize, it: FnParam<I>|
            iter_old == it.at_entry() && c == produced.len());
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

fn use_0() {
    assert!(count(W((0..10i64).rev(), Tag0)) == 10);
}

fn use_1() {
    assert!(count(W((0..10i64).rev(), Tag1)) == 10);
}

fn use_2() {
    assert!(count(W((0..10i64).rev(), Tag2)) == 10);
}

fn use_3() {
    assert!(count(W((0..10i64).rev(), Tag3)) == 11);
}

fn main() {
    use_0();
    use_1();
    use_2();
    use_3();
}
