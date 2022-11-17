use super::Functor;

impl<A> Functor for Vec<A>
{
    type Val = A;
    type FmapOut<B> = Vec<B>;

    fn fmap<F, B>(self, f: &F) -> Self::FmapOut<B>
        where F: Fn(A) -> B
    {
        let mut b = Vec::with_capacity(self.len());
        for x in self {
            b.push(f(x))
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::functor::*;

    #[test]
    fn mono_fmap_test() {
        let f = |x: i32| x + 1;
        {
            let a = vec![1, 2, 3].fmap(&f);
            assert_eq!(vec![2, 3, 4], a);
        }
        {
            let a = vec![]
                .fmap(&|x: i32| x + 1);
            let z: Vec<i32> = vec![];
            assert_eq!(z, a);
        }
    }

    #[test]
    fn poly_fmap_test() {
        let f = |x: i32| x.to_string();
        {
            let a = vec![1, 2, 3].fmap(&f);
            assert_eq!(vec!["1", "2", "3"], a);
        }
        {
            let a = vec![].fmap(&f);
            let z: Vec<String> = vec![];
            assert_eq!(z, a);
        }
    }
}
