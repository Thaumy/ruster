pub fn eq<T>(a: &T, b: &T) -> bool
    where T: PartialEq
{
    a.eq(b)
}

pub fn ne<T>(a: &T, b: &T) -> bool
    where T: PartialEq
{
    a.ne(b)
}

#[cfg(test)]
mod test {
    use crate::ext::partial_eq::*;

    #[test]
    fn eq_test() {
        assert!(eq(&1, &1));
        assert!(!eq(&1, &2));
    }

    #[test]
    fn ne_test() {
        assert!(ne(&1, &2));
        assert!(!ne(&1, &1));
    }
}
