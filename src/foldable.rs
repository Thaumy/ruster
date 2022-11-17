use crate::monoid::Monoid;

pub trait Foldable {
    type Item;

    //TODO foldr impl foldl
    fn foldl<F, ACC>(&self, f: &F, acc: ACC) -> ACC
        where F: Fn(ACC, &Self::Item) -> ACC;

    fn foldr<F, ACC>(&self, f: &F, acc: ACC) -> ACC
        where F: Fn(&Self::Item, ACC) -> ACC;

    fn fold_map<F, M>(&self, f: &F) -> M
        where F: Fn(&Self::Item) -> M,
              M: Monoid,
    {
        self.foldr(
            &|x, acc: M| f(x).mappend(acc),
            M::mempty(),
        )
    }

    //TODO fold
}

pub fn foldl<T, F, ACC>(t: &T, f: &F, acc: ACC) -> ACC
    where F: Fn(ACC, &T::Item) -> ACC,
          T: Foldable
{
    t.foldl(f, acc)
}

pub fn foldr<T, F, ACC>(t: &T, f: &F, acc: ACC) -> ACC
    where F: Fn(&T::Item, ACC) -> ACC,
          T: Foldable
{
    t.foldr(f, acc)
}

pub fn fold_map<T, F, M>(t: &T, f: &F) -> M
    where F: Fn(&T::Item) -> M,
          M: Monoid,
          T: Foldable
{
    t.fold_map(f)
}

pub mod vec;
