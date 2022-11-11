/*
use std::default::default;
use super::Semigroup;

impl<T> Semigroup for &[T] where T:Default {
    fn sappend(self, b: &[T]) -> &[T] {
        let len = self.len() + b.len();
        let c=[T::default();len];
        self.()
        self.extend(b);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::semigroup::*;

    #[test]
    fn sappend_test() {
        {
            let a = vec![1, 2, 3].sappend(vec![4, 5, 6]);
            assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        }
        {
            let a = sappend(vec![1, 2, 3], vec![4, 5, 6]);
            assert_eq!(vec![1, 2, 3, 4, 5, 6], a);
        }
    }
}*/
