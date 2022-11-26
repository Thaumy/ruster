//TODO deprecated impl
use crate::functional::monad::Monad;

impl<T> Monad for Box<T> {
    type BindOut<B> = Box<B>;

    fn bind<F, B>(self, f: &F) -> Box<B>
        where F: Fn(T) -> Box<B>
    {
        f(*self)
    }
}

#[cfg(test)]
mod test {
    use crate::functional::monad::*;

    #[test]
    fn unit_test() {
        {
            let a = Box::<i32>::unit(1);
            assert_eq!(Box::new(1), a)
        }
        {
            let a: Box<i32> = unit(1);
            assert_eq!(Box::new(1), a)
        }
    }

    #[test]
    fn mono_bind_test() {
        let f = |x: i32| Box::new(x + 1);
        {
            let a = Box::new(1).bind(&f);
            assert_eq!(Box::new(2), a)
        }
        {
            let a = bind(Box::new(1), &f);
            assert_eq!(Box::new(2), a)
        }
    }

    #[test]
    fn poly_bind_test() {
        let f = |x: i32| Box::new(x.to_string());
        {
            let a = Box::new(1).bind(&f);
            assert_eq!(Box::new("1".to_string()), a)
        }
        {
            let a = bind(Box::new(1), &f);
            assert_eq!(Box::new("1".to_string()), a)
        }
    }
}