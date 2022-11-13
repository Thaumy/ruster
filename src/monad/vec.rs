use crate::monad::Monad;

impl<T> Monad for Vec<T> {
    type BindOut<B> = Vec<B>;

    fn bind<F, B>(self, f: &F) -> Vec<B>
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
                .bind(&|x| vec![x * 1, x * 2, x * 3]);
            assert_eq!(
                vec![1, 2, 3,
                     2, 4, 6,
                     3, 6, 9],
                a
            )
        }
        {
            let a = Vec::<i32>::new()
                .bind(&|x| vec![x * 1, x * 2, x * 3]);
            assert_eq!(Vec::<i32>::new(), a)
        }
        {
            let a = bind(
                vec![1, 2, 3],
                &|x| vec![x * 1, x * 2, x * 3],
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
                &|x| vec![x * 1, x * 2, x * 3],
            );
            assert_eq!(Vec::<i32>::new(), a)
        }
    }

    #[test]
    fn poly_bind_test() {
        let f = |x: i32| {
            let g = |x: &i32, str: &str| {
                let mut s = x.to_string();
                s.push_str(str);
                s
            };
            vec![g(&x, "a"), g(&x, "b"), g(&x, "c")]
        };
        {
            let a = vec![1, 2, 3].bind(&f);
            assert_eq!(
                vec!["1a", "1b", "1c",
                     "2a", "2b", "2c",
                     "3a", "3b", "3c"],
                a
            )
        }
        {
            let a = Vec::<i32>::new().bind(&f);
            assert_eq!(Vec::<String>::new(), a)
        }
        {
            let a = bind(vec![1, 2, 3], &f);
            assert_eq!(
                vec!["1a", "1b", "1c",
                     "2a", "2b", "2c",
                     "3a", "3b", "3c"],
                a
            )
        }
        {
            let a = bind(Vec::<i32>::new(), &f);
            assert_eq!(Vec::<String>::new(), a)
        }
    }
}