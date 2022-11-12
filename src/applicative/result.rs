use crate::functor::Functor;
use super::Applicative;

impl<T, E> Applicative for Result<T, E> {
    type ApTo<A: Clone> = Result<A, E>;
    type ApOut<B> = Result<B, E>;

    fn pure(a: T) -> Self {
        Ok(a)
    }

    fn ap<A, B>(self, a: Result<A, E>) -> Result<B, E>
        where T: Fn(A) -> B
    {
        match self {
            Ok(f) => a.fmap(f),
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::applicative::*;

    #[test]
    fn pure_test() {
        let a = Result::<i32, String>::pure(1);
        assert_eq!(Ok(1), a);

        let b: Result<i32, String> = pure(1);
        assert_eq!(Ok(1), a);
    }

    #[test]
    fn mono_ap_test() {
        {
            let a: Result<i32, String> = Ok(|x| x + 1).ap(Ok(1));
            assert_eq!(Ok(2), a);

            let b = Result::<fn(i32) -> i32, String>::Err("".to_string()).ap(Ok(1));
            assert_eq!(Err("".to_string()), b);
        }
        {
            let a: Result<i32, String> = ap(Ok(|x| x + 1), Ok(1));
            assert_eq!(Ok(2), a);

            let b = ap(Result::<fn(i32) -> i32, String>::Err("".to_string()), Ok(1));
            assert_eq!(Err("".to_string()), b);
        }
    }

    #[test]
    fn poly_ap_test() {
        {
            let a: Result<String, String> = Ok(|x: i32| x.to_string()).ap(Ok(1));
            assert_eq!(Ok("1".to_string()), a);

            let b: Result<i32, String> = Result::<fn(i32) -> i32, String>::Err("".to_string()).ap(Ok(1));
            assert_eq!(Err("".to_string()), b);
        }
        {
            let a: Result<String, String> = ap(Ok(|x: i32| x.to_string()), Ok(1));
            assert_eq!(Ok("1".to_string()), a);

            let b: Result<String, String> = ap(Result::<fn(i32) -> String, String>::Err("".to_string()), Ok(1));
            assert_eq!(Err("".to_string()), b);
        }
    }
}