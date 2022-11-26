use std::cmp::Ordering;

pub fn cmp<T>(a: &T, b: &T) -> Ordering
    where T: Ord
{
    a.cmp(b)
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use crate::util::ord::*;

    #[test]
    fn cmp_test() {
        assert_eq!(Less, cmp(&1, &2));
        assert_eq!(Equal, cmp(&1, &1));
        assert_eq!(Greater, cmp(&2, &1));
    }
}