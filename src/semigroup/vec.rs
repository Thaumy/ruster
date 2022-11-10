use super::Semigroup;

impl<T> Semigroup for Vec<T> {
    //TODO extend/append or clone?
    fn sappend(mut self, b: Vec<T>) -> Vec<T> {
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
}