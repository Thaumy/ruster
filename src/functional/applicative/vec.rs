use super::Applicative;

impl<T> Applicative for Vec<T> {
    type ApTo<A: Clone> = Vec<A>;
    type ApOut<B> = Vec<B>;

    fn pure(a: T) -> Self {
        vec![a]
    }

    fn ap<A: Clone, B>(self, a: Vec<A>) -> Vec<B>
        where T: Fn(A) -> B
    {
        let mut b: Vec<B> = Vec::with_capacity(self.len() * a.len());
        for f in self {
            for x in &a {
                b.push(f(x.clone()));
            }
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::functional::applicative::*;

    #[test]
    fn pure_test() {
        let a = Vec::<i32>::pure(1);
        assert_eq!(vec![1], a);

        let b: Vec<i32> = pure(1);
        assert_eq!(vec![1], b);
    }

    #[test]
    fn mono_ap_test() {
        {
            let a: Vec<Box<dyn Fn(i32) -> i32>> =
                vec![
                    Box::new(|x| x + 1),
                    Box::new(|x| x * 2),
                    Box::new(|x| x - 1),
                ];
            assert_eq!(vec![2, 3, 4, 2, 4, 6, 0, 1, 2], a.ap(vec![1, 2, 3]));

            let b = Vec::<fn(i32) -> i32>::new().ap(vec![1]);
            assert_eq!(Vec::<i32>::new(), b);
        }
        {
            let a: Vec<Box<dyn Fn(i32) -> i32>> =
                vec![
                    Box::new(|x| x + 1),
                    Box::new(|x| x * 2),
                    Box::new(|x| x - 1),
                ];
            assert_eq!(vec![2, 3, 4, 2, 4, 6, 0, 1, 2], ap(a, vec![1, 2, 3]));

            let b = ap(Vec::<fn(i32) -> i32>::new(), vec![1]);
            assert_eq!(Vec::<i32>::new(), b);
        }
    }

    #[test]
    fn poly_ap_test() {
        {
            let a: Vec<Box<dyn Fn(i32) -> String>> =
                vec![
                    Box::new(|x| {
                        let mut s = x.to_string();
                        s.push('a');
                        s
                    }),
                    Box::new(|x| {
                        let mut s = x.to_string();
                        s.push('b');
                        s
                    }),
                    Box::new(|x| {
                        let mut s = x.to_string();
                        s.push('c');
                        s
                    }),
                ];
            assert_eq!(
                vec!["1a", "2a", "3a",
                     "1b", "2b", "3b",
                     "1c", "2c", "3c"],
                a.ap(vec![1, 2, 3])
            );

            let b = Vec::<fn(i32) -> String>::new().ap(vec![1]);
            assert_eq!(Vec::<String>::new(), b);
        }
        {
            let a: Vec<Box<dyn Fn(i32) -> String>> =
                vec![
                    Box::new(|x| {
                        let mut s = x.to_string();
                        s.push('a');
                        s
                    }),
                    Box::new(|x| {
                        let mut s = x.to_string();
                        s.push('b');
                        s
                    }),
                    Box::new(|x| {
                        let mut s = x.to_string();
                        s.push('c');
                        s
                    }),
                ];
            assert_eq!(
                vec!["1a", "2a", "3a",
                     "1b", "2b", "3b",
                     "1c", "2c", "3c"],
                ap(a, vec![1, 2, 3])
            );

            let b = ap(Vec::<fn(i32) -> String>::new(), vec![1]);
            assert_eq!(Vec::<String>::new(), b);
        }
    }
}
