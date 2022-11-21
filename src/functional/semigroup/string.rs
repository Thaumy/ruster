use super::Semigroup;

impl Semigroup for String {
    //TODO extend/append or clone?
    fn sappend(mut self, b: String) -> String {
        self.push_str(b.as_str());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::functional::semigroup::*;

    #[test]
    fn sappend_test() {
        {
            let a = "123".to_string().sappend("456".to_string());
            assert_eq!("123456".to_string(), a);
        }
        {
            let a = sappend("123".to_string(), "456".to_string());
            assert_eq!("123456".to_string(), a);
        }
    }
}
