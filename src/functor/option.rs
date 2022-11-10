use super::Functor;

impl<A> Functor for Option<A> {
    type Val = A;
    type FmapOut<B> = Option<B>;

    fn fmap<F, B>(self, f: F) -> Option<B>
        where F: Fn(A) -> B
    {
        match self {
            Some(x) => Some(f(x)),
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::functor::*;

    #[test]
    fn mono_fmap_test() {
        {
            let a = Some(1).fmap(|x| x + 1);
            assert_eq!(Some(2), a);

            let b = None.fmap(|x: i32| x + 1);
            assert_eq!(None, b);
        }
        {
            let a = fmap(Some(1), |x| x + 1);
            assert_eq!(Some(2), a);

            let b = fmap(None, |x: i32| x + 1);
            assert_eq!(None, b);
        }
    }

    #[test]
    fn poly_fmap_test() {
        let a = fmap(Some(1), |x| x.to_string());
        assert_eq!(Some("1".to_string()), a);

        let b = fmap(None, |x: i32| x.to_string());
        assert_eq!(None, b);
    }
}