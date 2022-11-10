use crate::monoid::Monoid;

pub trait Foldable<T> {
    //TODO foldr impl foldl
    fn foldl<ACC, F>(self, f: F, acc: ACC) -> ACC
        where F: Fn(ACC, T) -> ACC;

    fn foldr<ACC, F>(self, f: F, acc: ACC) -> ACC
        where F: Fn(T, ACC) -> ACC;

    fn fold_map<F, M>(self, f: F) -> M
        where F: Fn(T) -> M,
              M: Monoid,
              Self: Sized
    {
        self.foldr(
            |x, acc| acc.mappend(f(x)),
            M::mempty(),
        )
    }

    //TODO fold
}

fn foldl<T, X, ACC, F>(t: T, f: F, acc: ACC) -> ACC
    where F: Fn(ACC, X) -> ACC,
          T: Foldable<X>
{
    t.foldl(f, acc)
}

fn foldr<T, X, ACC, F>(t: T, f: F, acc: ACC) -> ACC
    where F: Fn(X, ACC) -> ACC,
          T: Foldable<X>
{
    t.foldr(f, acc)
}

fn fold_map<T, X, F, M>(t: T, f: F) -> M
    where F: Fn(X) -> M,
          M: Monoid,
          T: Foldable<X> + Sized
{
    t.foldr(
        |x, acc| acc.mappend(f(x)),
        M::mempty(),
    )
}

pub mod iterator;