use crate::functor::Functor;
use super::Applicative;

impl<T> Applicative for Option<T> {
    type ApTo<A: Clone> = Option<A>;
    type ApOut<B> = Option<B>;

    fn pure(a: T) -> Self {
        Some(a)
    }

    fn ap<A, B>(self, a: Option<A>) -> Option<B>
        where T: Fn(A) -> B
    {
        match self {
            Some(f) => a.fmap(&f),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::applicative::*;

    #[test]
    fn pure_test() {
        let a = Option::<i32>::pure(1);
        assert_eq!(Some(1), a);

        let b: Option<i32> = pure(1);
        assert_eq!(Some(1), b);
    }

    #[test]
    fn mono_ap_test() {
        {
            let a = Some(|x| x + 1).ap(Some(1));
            assert_eq!(Some(2), a);

            let b = None::<fn(i32) -> i32>.ap(Some(1));
            assert_eq!(None, b);
        }
        {
            let a = ap(Some(|x| x + 1), Some(1));
            assert_eq!(Some(2), a);

            let b = ap(None::<fn(i32) -> i32>, Some(1));
            assert_eq!(None, b);
        }
    }

    #[test]
    fn poly_ap_test() {
        {
            let a = Some(|x: i32| x.to_string()).ap(Some(1));
            assert_eq!(Some("1".to_string()), a);

            let b = None::<fn(i32) -> i32>.ap(Some(1));
            assert_eq!(None, b);
        }
        {
            let a = ap(Some(|x: i32| x.to_string()), Some(1));
            assert_eq!(Some("1".to_string()), a);

            let b = ap(None::<fn(i32) -> i32>, Some(1));
            assert_eq!(None, b);
        }
    }
}