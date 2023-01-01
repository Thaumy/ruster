use crate::functional::applicative::Applicative;
use crate::functional::functor::Functor;

pub trait Monad: Applicative {
    type BindOut<B>: Monad<Val=B>;

    fn unit(x: <Self as Functor>::Val) -> Self
        where Self: Sized
    {
        <Self as Applicative>::pure(x)
    }

    fn bind<F, B>(self, f: &F) -> Self::BindOut<B>
        where F: Fn(Self::Val) -> Self::BindOut<B>;
}

pub fn unit<T, X>(x: X) -> T
    where T: Monad<Val=X>
{
    T::unit(x)
}

pub fn bind<T, F, B>(t: T, f: &F) -> T::BindOut<B>
    where T: Monad,
          F: Fn(T::Val) -> T::BindOut<B>
{
    t.bind(f)
}

pub trait MonadExt: Sized {
    fn unit_to<T>(self) -> T
        where T: Monad<Val=Self>,
    {
        T::unit(self)
    }
}

impl<T: Sized> MonadExt for T {}

//pub mod boxed;
pub mod option;
pub mod result;
pub mod vec;
