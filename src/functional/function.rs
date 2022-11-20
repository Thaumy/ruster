pub fn id<A>(a: A) -> A {
    a
}

//TODO T.flip() ext?
pub fn flip<X, Y, Z>(f: impl Fn(X, Y) -> Z) -> impl Fn(Y, X) -> Z
{
    //TODO suitable move?
    move |y, x| f(x, y)
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

pub trait Ext {
    fn always<X>(self, x: X) -> X
        where Self: Sized
    {
        return x;
    }
    fn effect<R>(self, f: impl Fn(&Self) -> R) -> Self
        where Self: Sized
    {
        f(&self);
        return self;
    }
}

pub mod sized;

#[cfg(test)]
mod test {
    use crate::functional::function::*;

    #[test]
    fn id_test() {
        let a = 1;
        assert_eq!(1, id(a))
    }

    #[test]
    fn flip_test() {
        let f = |a, b| a - b;
        assert_eq!(1, flip(f)(1, 2))
    }

    #[test]
    fn always_test() {
        {
            let a = 1;
            assert_eq!(2, a.always(2))
        }
        {
            let a = 1;
            assert_eq!(2, always(2)(a))
        }
    }

    #[test]
    fn effect_test() {
        {
            let a = 1;
            a.effect(|x| { assert_eq!(1, *x) });
        }
        {
            let a = 1;
            effect(a, |x| { assert_eq!(1, *x) });
        }
    }
}