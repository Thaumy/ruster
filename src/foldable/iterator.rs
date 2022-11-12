use super::Foldable;
use crate::function::*;

impl<T, I> Foldable<T> for I
    where I: Iterator<Item=T> + DoubleEndedIterator
{
    fn foldl<ACC, F>(self, f: F, acc: ACC) -> ACC where F: Fn(ACC, T) -> ACC {
        self.fold(acc, f)
    }

    fn foldr<ACC, F>(self, f: F, acc: ACC) -> ACC where F: Fn(T, ACC) -> ACC {
        self.rev().fold(acc, flip(f))
    }
}

#[cfg(test)]
mod test {
    use crate::foldable::*;

    #[test]
    fn foldl_test() {
        {
            let a = vec![1, 2, 3]
                .into_iter()
                .foldl(|acc, x| format!("{}{}", acc, x), "".to_string());
            assert_eq!("123".to_string(), a);
        }
        {
            let a = vec![1, 2, 3].into_iter();
            let b = foldl(a, |acc, x| format!("{}{}", acc, x), "".to_string());
            assert_eq!("123".to_string(), b);
        }
    }

    #[test]
    fn foldr_test() {
        {
            let a = vec![1, 2, 3]
                .into_iter()
                .foldr(|x, acc| format!("{}{}", acc, x), "".to_string());
            assert_eq!("321".to_string(), a);
        }
        {
            let a = vec![1, 2, 3].into_iter();
            let b = foldr(a, |x, acc| format!("{}{}", acc, x), "".to_string());
            assert_eq!("321".to_string(), b);
        }
    }

    #[test]
    fn fold_map_test() {
        {
            let a = vec![vec![1], vec![2], vec![3]]
                .into_iter()
                .fold_map(|mut x| {
                    x[0] = x[0] + 1;
                    x
                });
            assert_eq!(vec![4, 3, 2], a);
        }
        {
            let a = vec![vec![1], vec![2], vec![3]].into_iter();
            let b =
                fold_map(a, |mut x| {
                    x[0] = x[0] + 1;
                    x
                });
            assert_eq!(vec![4, 3, 2], b);
        }
    }
}