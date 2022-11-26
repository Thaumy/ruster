pub trait Functor {
    type Val;
    type FmapOut<B>: Functor<Val=B>;

    fn fmap<F, B>(self, f: &F) -> Self::FmapOut<B>
        where F: Fn(Self::Val) -> B;
}

pub fn fmap<T, F, B>(t: T, f: &F) -> T::FmapOut<B>
    where T: Functor,
          F: Fn(T::Val) -> B
{
    t.fmap(f)
}

//pub mod boxed;
pub mod option;
pub mod result;
pub mod vec;

