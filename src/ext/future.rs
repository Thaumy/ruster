use std::future::Future;

pub fn pure<T>(t: T) -> impl Future<Output=T>
{
    async {
        t
    }
}

pub trait FutureExt: Future + Sized {
    /*TODO
    fn pure<T>(t: T) -> impl Future<Output=T>
    {
        async {
            t
        }
    }*/

    fn fmap<F, T2>(self, f: F) -> impl Future<Output=T2>
        where F: FnOnce(Self::Output) -> T2
    {
        async {
            f(self.await)
        }
    }

    fn bind<F, R, T2>(self, f: F) -> impl Future<Output=T2>
        where F: FnOnce(Self::Output) -> R,
              R: Future<Output=T2>
    {
        async {
            f(self.await).await
        }
    }
}

impl<T> FutureExt for T where T: Future {}

#[cfg(test)]
mod test {
    use std::future::Future;
    use crate::ext::future::*;
    use futures::executor::block_on;

    #[test]
    fn pure_test() {
        let x = pure(1);
        assert_eq!(1, block_on(x));
    }

    #[test]
    fn fmap_test() {
        let x = pure(1);
        let x2 = x.fmap(|x| x + 1);
        assert_eq!(2, block_on(x2));
    }

    #[test]
    fn bind_test() {
        let x = pure(1);
        assert_eq!(2, block_on(x.bind(|x| async move { x + 1 })));
    }
}
