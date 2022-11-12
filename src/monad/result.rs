use crate::monad::Monad;

impl<T, E> Monad for Result<T, E> {
    type BindOut<B> = Result<B, E>;

    fn bind<F, B>(self, f: F) -> Result<B, E>
        where F: Fn(T) -> Result<B, E>
    {
        match self {
            Ok(x) => f(x),
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::monad::*;

    #[test]
    fn unit_test() {
        {
            let a = Result::<i32, String>::unit(1);
            assert_eq!(Ok(1), a)
        }
        {
            let a: Result<i32, String> = unit(1);
            assert_eq!(Ok(1), a)
        }
    }

    #[test]
    fn mono_bind_test() {
        {
            let a: Result<i32, String> = Ok(1).bind(|x| Ok(x + 1));
            assert_eq!(Ok(2), a)
        }
        {
            let a = Err("").bind(|x: i32| Ok(x + 1));
            assert_eq!(Err(""), a)
        }
        {
            let a: Result<i32, String> = bind(Ok(1), |x| Ok(x + 1));
            assert_eq!(Ok(2), a)
        }
        {
            let a = bind(Err(""), |x: i32| Ok(x + 1));
            assert_eq!(Err(""), a)
        }
    }

    #[test]
    fn poly_bind_test() {
        {
            let a: Result<String, String> = Ok(1).bind(|x| Ok(x.to_string()));
            assert_eq!(Ok("1".to_string()), a)
        }
        {
            let a = Err("").bind(|x: i32| Ok(x.to_string()));
            assert_eq!(Err(""), a)
        }
        {
            let a: Result<String, String> = bind(Ok(1), |x| Ok(x.to_string()));
            assert_eq!(Ok("1".to_string()), a)
        }
        {
            let a = bind(Err(""), |x: i32| Ok(x.to_string()));
            assert_eq!(Err(""), a)
        }
    }
}