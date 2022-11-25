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

#[cfg(test)]
mod test {
    use crate::ext::partial_ord::*;

    #[test]
    fn gt_test() {
        assert!(gt(&2, &1));
        assert!(!gt(&1, &1));
        assert!(!gt(&1, &2));
    }

    #[test]
    fn lt_test() {
        assert!(lt(&1, &2));
        assert!(!lt(&1, &1));
        assert!(!lt(&2, &1));
    }

    #[test]
    fn ge_test() {
        assert!(ge(&2, &1));
        assert!(ge(&1, &1));
        assert!(!ge(&1, &2));
    }

    #[test]
    fn le_test() {
        assert!(le(&1, &2));
        assert!(le(&1, &1));
        assert!(!le(&2, &1));
    }
}