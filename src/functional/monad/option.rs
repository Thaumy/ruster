use crate::functional::monad::Monad;

impl<T> Monad for Option<T> {
    type BindOut<B> = Option<B>;

    fn bind<F, B>(self, f: &F) -> Option<B>
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
    use crate::functional::monad::*;

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
        let f = |x: i32| Some(x + 1);
        {
            let a = Some(1).bind(&f);
            assert_eq!(Some(2), a);

            let b = None.bind(&f);
            assert_eq!(None, b)
        }
        {
            let a = bind(Some(1), &f);
            assert_eq!(Some(2), a);

            let b = bind(None, &f);
            assert_eq!(None, b)
        }
    }

    #[test]
    fn poly_bind_test() {
        let f = |x: i32| Some(x.to_string());
        {
            let a = Some(1).bind(&f);
            assert_eq!(Some("1".to_string()), a);

            let b = None.bind(&f);
            assert_eq!(None, b)
        }
        {
            let a = bind(Some(1), &f);
            assert_eq!(Some("1".to_string()), a);

            let b = bind(None, &f);
            assert_eq!(None, b)
        }
    }
}
