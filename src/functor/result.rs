use super::Functor;

impl<A, E> Functor for Result<A, E> {
    type Val = A;
    type FmapOut<B> = Result<B, E>;

    fn fmap<F, B>(self, f: F) -> Result<B, E>
        where F: Fn(A) -> B
    {
        match self {
            Ok(x) => Ok(f(x)),
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::functor::*;

    #[test]
    fn mono_fmap_test() {
        let a: Result<i32, String> = Ok(1).fmap(|x| x + 1);
        assert_eq!(Ok(2), a);

        let b = Err(0).fmap(|x: i32| x + 1);
        assert_eq!(Err(0), b);
    }

    #[test]
    fn poly_fmap_test() {
        let a: Result<String, String> = Ok(1).fmap(|x| x.to_string());
        assert_eq!(Ok("1".to_string()), a);

        let b = Err(0).fmap(|x: i32| x.to_string());
        assert_eq!(Err(0), b);
    }
}
