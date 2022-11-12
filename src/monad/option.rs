use crate::monad::Monad;

impl<T> Monad for Option<T> {
    type BindOut<B> = Option<B>;

    fn bind<F, B>(self, f: F) -> Option<B>
        where F: Fn(T) -> Option<B>
    {
        match self {
            Some(x) => f(x),
            None => None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::monad::*;

    #[test]
    fn unit_test() {
        {
            let a = Option::<i32>::unit(1);
            assert_eq!(Some(1), a)
        }
        {
            let a: Option<i32> = unit(1);
            assert_eq!(Some(1), a)
        }
    }

    #[test]
    fn mono_bind_test() {
        {
            let a = Some(1).bind(|x| Some(x + 1));
            assert_eq!(Some(2), a)
        }
        {
            let a = None.bind(|x: i32| Some(x + 1));
            assert_eq!(None, a)
        }
        {
            let a = bind(Some(1), |x| Some(x + 1));
            assert_eq!(Some(2), a)
        }
        {
            let a = bind(None, |x: i32| Some(x + 1));
            assert_eq!(None, a)
        }
    }

    #[test]
    fn poly_bind_test() {
        {
            let a = Some(1).bind(|x| Some(x.to_string()));
            assert_eq!(Some("1".to_string()), a)
        }
        {
            let a = None.bind(|x: i32| Some(x.to_string()));
            assert_eq!(None, a)
        }
        {
            let a = bind(Some(1), |x| Some(x.to_string()));
            assert_eq!(Some("1".to_string()), a)
        }
        {
            let a = bind(None, |x: i32| Some(x.to_string()));
            assert_eq!(None, a)
        }
    }
}
