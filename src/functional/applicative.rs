use crate::functional::functor::Functor;

pub trait Applicative: Functor {
    type ApTo<A: Clone>: Applicative<Val=A>;
    type ApOut<B>: Applicative<Val=B>;

    fn pure(x: Self::Val) -> Self;
    fn ap<A: Copy, B>(self, a: Self::ApTo<A>) -> Self::ApOut<B>
        where Self::Val: Fn(<Self::ApTo<A> as Functor>::Val) -> <Self::ApOut<B> as Functor>::Val;
}

pub fn pure<T, X>(x: X) -> T
    where T: Applicative<Val=X>
{
    T::pure(x)
}

pub fn ap<T, A: Copy, B>(t: T, a: T::ApTo<A>) -> T::ApOut<B>
    where
        T: Applicative,
        T::Val: Fn(<T::ApTo<A> as Functor>::Val) -> <T::ApOut<B> as Functor>::Val
{
    t.ap(a)
}

pub mod boxed;
pub mod option;
pub mod result;
pub mod vec;
