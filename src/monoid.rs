use crate::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn mempty() -> Self;
    fn mappend(self, b: Self) -> Self where Self: Sized {
        self.sappend(b)
    }
}

pub fn mempty<T>() -> T
    where T: Monoid
{
    T::mempty()
}

pub fn mappend<T>(a: T, b: T) -> T
    where T: Monoid
{
    a.mappend(b)
}

pub mod vec;
pub mod array;
