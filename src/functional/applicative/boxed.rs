//TODO deprecated impl
use super::Applicative;

impl<T> Applicative for Box<T> {
    type ApTo<A: Clone> = Box<A>;
    type ApOut<B> = Box<B>;

    fn pure(a: T) -> Self {
        Box::new(a)
    }

    fn ap<A, B>(self, a: Box<A>) -> Box<B>
        where T: Fn(A) -> B
    {
        Box::new(self(*a))
    }
}

#[cfg(test)]
mod tests {
    use crate::functional::applicative::*;

    #[test]
    fn pure_test() {
        {
            let a = Box::<i32>::pure(1);
            assert_eq!(Box::new(1), a);
        }
        {
            let a: Box<i32> = pure(1);
            assert_eq!(Box::new(1), a);
        }
    }

    #[test]
    fn mono_ap_test() {
        {
            let a = Box::new(|x| x + 1).ap(Box::new(1));
            assert_eq!(Box::new(2), a);
        }
        {
            let a = ap(Box::new(|x| x + 1), Box::new(1));
            assert_eq!(Box::new(2), a);
        }
    }

    #[test]
    fn poly_ap_test() {
        {
            let a = Box::new(|x: i32| x.to_string()).ap(Box::new(1));
            assert_eq!(Box::new("1".to_string()), a);
        }
        {
            let a = ap(Box::new(|x: i32| x.to_string()), Box::new(1));
            assert_eq!(Box::new("1".to_string()), a);
        }
    }
}