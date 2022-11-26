pub trait SizedExt {
    fn id<X>(self) -> impl FnOnce(X) -> Self
        where Self: Sized
    {
        |_: X| self
    }

    fn always<X>(self, x: X) -> X
        where Self: Sized
    {
        x
    }

    fn effect<R>(self, f: impl Fn(&Self) -> R) -> Self
        where Self: Sized
    {
        f(&self);
        self
    }
}

pub fn id<A>(a: A) -> A {
    a
}

pub fn always<X, Y>(x: X) -> impl Fn(Y) -> X
    where X: Copy
{
    return move |_| x;
}

pub fn effect<X, R>(x: X, f: impl Fn(&X) -> R) -> X
{
    f(&x);
    return x;
}

impl<T> SizedExt for T where T: Sized {}

#[cfg(test)]
mod test {
    use crate::ext::sized::*;
    use crate::ext::closure::*;

    #[test]
    fn id_test() {
        let a = 1;
        assert_eq!(1, id(a));
        let b = 1;
        assert_eq!(1, b.id()(2));
    }

    #[test]
    fn always_test() {
        let a = 1;
        assert_eq!(2, a.always(2));
        let b = 1;
        assert_eq!(2, always(2)(b));
    }

    #[test]
    fn effect_test() {
        let a = 1;
        a.effect(|x| { assert_eq!(1, *x) });
        let b = 1;
        effect(b, |x| { assert_eq!(1, *x) });
    }
}