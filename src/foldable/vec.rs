use std::borrow::Borrow;
use crate::foldable::Foldable;
use crate::function::flip;

impl<A> Foldable for Vec<A> {
    type Item = A;

    fn foldl<F, ACC>(&self, f: &F, acc: ACC) -> ACC
        where F: Fn(ACC, &A) -> ACC
    {
        self.into_iter().fold(acc, f)
    }

    fn foldr<F, ACC>(&self, f: &F, acc: ACC) -> ACC
        where F: Fn(&A, ACC) -> ACC
    {
        self.into_iter().rev().fold(acc, flip(f))
    }
}

#[cfg(test)]
mod test {
    use crate::foldable::*;
    use crate::monoid::*;

    #[test]
    fn foldl_test() {
        {
            let a = vec![1, 2, 3]
                .foldl(&|acc, x| format!("{}{}", acc, x), "".to_string());
            assert_eq!("123".to_string(), a);
        }
        {
            let a = vec![1, 2, 3];
            let b = foldl(&a, &|acc, x| format!("{}{}", acc, x), "".to_string());
            assert_eq!("123".to_string(), b);
        }
    }

    #[test]
    fn foldr_test() {
        {
            let a = vec![1, 2, 3]
                .foldr(&|x, acc| format!("{}{}", acc, x), "".to_string());
            assert_eq!("321".to_string(), a);
        }
        {
            let a = vec![1, 2, 3];
            let b = foldr(&a, &|x, acc| format!("{}{}", acc, x), "".to_string());
            assert_eq!("321".to_string(), b);
        }
    }

    #[test]
    fn fold_map_test() {
        {
            let a = vec![vec![1], vec![1, 2], vec![1, 2, 3]];
            let b = a.fold_map(&|x| {
                vec![x.len(), x.len()]
            });
            assert_eq!(vec![1, 1, 2, 2, 3, 3], b);
        }
        {
            let a = vec![vec![1], vec![1, 2], vec![1, 2, 3]];
            let b = fold_map(&a, &|x| {
                vec![x.len(), x.len()]
            });
            assert_eq!(vec![1, 1, 2, 2, 3, 3], b);
        }
    }
}
