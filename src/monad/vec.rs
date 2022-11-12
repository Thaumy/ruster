use crate::monad::Monad;

impl<T> Monad for Vec<T> {
    type BindOut<B> = Vec<B>;

    fn bind<F, B>(self, f: F) -> Vec<B>
        where F: Fn(T) -> Vec<B>
    {
        let mut b = Vec::with_capacity(self.len());//TODO
        for x in self {
            b.extend(f(x));
        }
        b
    }
}

#[cfg(test)]
mod test {
    use crate::monad::*;

    #[test]
    fn unit_test() {
        {
            let a = Vec::<i32>::unit(1);
            assert_eq!(vec![1], a)
        }
        {
            let a: Vec<i32> = unit(1);
            assert_eq!(vec![1], a)
        }
    }

    #[test]
    fn mono_bind_test() {
        {
            let a = vec![1, 2, 3]
                .bind(|x| vec![x * 1, x * 2, x * 3]);
            assert_eq!(
                vec![1, 2, 3,
                     2, 4, 6,
                     3, 6, 9],
                a
            )
        }
        {
            let a = Vec::<i32>::new()
                .bind(|x| vec![x * 1, x * 2, x * 3]);
            assert_eq!(Vec::<i32>::new(), a)
        }
        {
            let a = bind(
                vec![1, 2, 3],
                |x| vec![x * 1, x * 2, x * 3],
            );
            assert_eq!(
                vec![1, 2, 3,
                     2, 4, 6,
                     3, 6, 9],
                a
            )
        }
        {
            let a = bind(
                Vec::<i32>::new(),
                |x| vec![x * 1, x * 2, x * 3],
            );
            assert_eq!(Vec::<i32>::new(), a)
        }
    }

    /*
    #[test]
    fn poly_bind_test() {
        {
            let a: Result<String, String> = Ok(1).bind(|x| Ok(1.to_string()));
            assert_eq!(Ok("1".to_string()), a)
        }
        {
            let a = Err("").bind(|x: i32| Ok(x.to_string()));
            assert_eq!(Err(""), a)
        }
        {
            let a: Result<String, String> = bind(Ok(1), |x| Ok(1.to_string()));
            assert_eq!(Ok("1".to_string()), a)
        }
        {
            let a = bind(Err(""), |x: i32| Ok(x.to_string()));
            assert_eq!(Err(""), a)
        }
    }*/
}