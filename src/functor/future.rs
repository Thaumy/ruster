use std::fmt::Debug;
use std::future::Future;
use super::Functor;
use crate::functor::boxed;

/*
impl<A> Functor for dyn Future<Output=A>
{
    type Val = A;

    type FmapOut<B> = impl Future<Output=B>;

    fn fmap<F, B>(self, f: F) -> Self::FmapOut<B> where F: Fn(Self::Val) -> B {
        todo!()
    }

    /*
    fn fmap<F, B, K>(self, f: F) -> K
        where F: Fn(A) -> B,
              K: Future<Output=B>
    {
        async {
            f(self.await)
        }
    }*/
}
*/

/*
impl<A> Functor for dyn Future<Output=A>
{
    type Val = A;

    type FmapOut<B> = impl Future<Output=B>;

    fn fmap<F, B>(self, f: F) -> Self::FmapOut<B> where F: Fn(Self::Val) -> B {
        todo!()
    }

    /*
    fn fmap<F, B, K>(self, f: F) -> K
        where F: Fn(A) -> B,
              K: Future<Output=B>
    {
        async {
            f(self.await)
        }
    }*/
}*/