pub trait Semigroup {
    fn sappend(self, b: Self) -> Self;
}

pub fn sappend<T>(a: T, b: T) -> T
    where T: Semigroup
{
    a.sappend(b)
}

pub mod array;
pub mod string;
pub mod vec;
