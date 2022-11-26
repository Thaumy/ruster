pub trait ClosureFlipExt<X, Y, Z> {
    fn flip(self) -> impl FnOnce(Y, X) -> Z;
}

impl<F, X, Y, Z> ClosureFlipExt<X, Y, Z> for F
    where F: FnOnce(X, Y) -> Z
{
    fn flip(self) -> impl FnOnce(Y, X) -> Z {
        |y, x| self(x, y)
    }
}

pub fn flip<X, Y, Z>(f: impl Fn(X, Y) -> Z) -> impl Fn(Y, X) -> Z
{
    //TODO suitable move?
    move |y, x| f(x, y)
}

#[cfg(test)]
mod test {
    use crate::ext::closure::*;

    #[test]
    fn flip_test() {
        let f1 = |a, b| a - b;
        assert_eq!(1, flip(f1)(1, 2));
        let f2 = |a, b| a - b;
        assert_eq!(1, f2.flip()(1, 2));
    }
}