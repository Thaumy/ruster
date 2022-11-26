use std::cmp::Ordering;

pub fn gt<T>(a: &T, b: &T) -> bool
    where T: PartialOrd
{
    a.gt(b)
}

pub fn lt<T>(a: &T, b: &T) -> bool
    where T: PartialOrd
{
    a.lt(b)
}

pub fn ge<T>(a: &T, b: &T) -> bool
    where T: PartialOrd
{
    a.ge(b)
}

pub fn le<T>(a: &T, b: &T) -> bool
    where T: PartialOrd
{
    a.le(b)
}

pub fn partial_cmp<T>(a: &T, b: &T) -> Option<Ordering>
    where T: PartialOrd
{
    a.partial_cmp(b)
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use crate::util::partial_ord::*;

    #[test]
    fn gt_test() {
        assert!(gt(&2.0, &1.0));
        assert!(!gt(&1.0, &1.0));
        assert!(!gt(&1.0, &2.0));
    }

    #[test]
    fn lt_test() {
        assert!(lt(&1.0, &2.0));
        assert!(!lt(&1.0, &1.0));
        assert!(!lt(&2.0, &1.0));
    }

    #[test]
    fn ge_test() {
        assert!(ge(&2.0, &1.0));
        assert!(ge(&1.0, &1.0));
        assert!(!ge(&1.0, &2.0));
    }

    #[test]
    fn le_test() {
        assert!(le(&1.0, &2.0));
        assert!(le(&1.0, &1.0));
        assert!(!le(&2.0, &1.0));
    }

    #[test]
    fn partial_cmp_test() {
        assert_eq!(Some(Less), partial_cmp(&1.0, &2.0));
        assert_eq!(Some(Equal), partial_cmp(&1.0, &1.0));
        assert_eq!(Some(Greater), partial_cmp(&2.0, &1.0));
        assert_eq!(None, partial_cmp(&f64::NAN, &f64::NAN));
    }
}