//TODO deprecated impl
use super::Functor;

impl<A> Functor for Box<A> {
    type Val = A;
    type FmapOut<B> = Box<B>;

    fn fmap<F, B>(self, f: &F) -> Box<B>
        where F: Fn(A) -> B
    {
        Box::new(f(*self))
    }
}

#[cfg(test)]
mod tests {
    use crate::functional::functor::*;

    #[test]
    fn mono_fmap_test() {
        let f = |x: i32| x + 1;
        {
            let a = Box::new(1).fmap(&f);
            assert_eq!(Box::new(2), a);
        }
        {
            let a = fmap(Box::new(1), &f);
            assert_eq!(Box::new(2), a);
        }
    }

    #[test]
    fn poly_fmap_test() {
        let f = |x: i32| x.to_string();
        {
            let a = Box::new(1).fmap(&f);
            assert_eq!(Box::new("1".to_string()), a);
        }
        {
            let a = fmap(Box::new(1), &f);
            assert_eq!(Box::new("1".to_string()), a);
        }
    }
}
