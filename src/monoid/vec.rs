use super::Monoid;

impl<T> Monoid for Vec<T> {
    fn mempty() -> Vec<T> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::*;

    #[test]
    fn mempty_test() {
        {
            let a = Vec::<i32>::mempty();
            assert_eq!(Vec::<i32>::new(), a);
        }
        {
            let a: Vec::<i32> = mempty();
            assert_eq!(Vec::<i32>::new(), a);
        }
    }

    #[test]
    fn mappend_test() {
        {
            let a = vec![1, 2, 3].mappend(vec![4, 5, 6]);
            assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        }
        {
            let a = mappend(vec![1, 2, 3], vec![4, 5, 6]);
            assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        }
    }
}