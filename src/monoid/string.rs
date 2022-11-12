use super::Monoid;

impl Monoid for String {
    fn mempty() -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::monoid::*;

    #[test]
    fn mempty_test() {
        {
            let a = String::mempty();
            assert_eq!("".to_string(), a);
        }
        {
            let a: String = mempty();
            assert_eq!("".to_string(), a);
        }
    }

    #[test]
    fn mappend_test() {
        {
            let a = "123".to_string().mappend("456".to_string());
            assert_eq!("123456".to_string(), a);
        }
        {
            let a = mappend("123".to_string(), "456".to_string());
            assert_eq!("123456".to_string(), a);
        }
    }
}
