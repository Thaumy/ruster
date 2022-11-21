pub trait ResultExt<T, E> {
    fn unwrap_or_eval<F>(self, f: F) -> T
        where F: Fn(E) -> T;
    fn unwrap_or_panic(self, msg: &str) -> T;

    fn or_pure<F>(self, f: F) -> Result<T, E>
        where F: Fn(E) -> T;
    fn or_bind<F>(self, f: F) -> Result<T, E>
        where F: Fn(E) -> Result<T, E>;

    //evil ext
    fn from_comma_ok(x: T, ok: bool) -> Result<T, ()>;
    //evil ext
    fn from_ok_comma(ok: bool, x: T) -> Result<T, ()>;

    //TODO is unsafe?
    unsafe fn from_nullable(x: *const T) -> Result<T, ()>
        where T: Copy;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn unwrap_or_eval<F>(self, f: F) -> T
        where F: Fn(E) -> T
    {
        self.unwrap_or_else(f)
    }

    fn unwrap_or_panic(self, msg: &str) -> T {
        match self {
            Ok(x) => x,
            Err(_) => panic!("{}", msg)//TODO {msg}?
        }
    }

    fn or_pure<F>(self, f: F) -> Result<T, E>
        where F: Fn(E) -> T
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Ok(f(e))
        }
    }
    fn or_bind<F>(self, f: F) -> Result<T, E>
        where F: Fn(E) -> Result<T, E>
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => f(e)
        }
    }

    //evil ext
    fn from_comma_ok(x: T, ok: bool) -> Result<T, ()> {
        match ok {
            true => Ok(x),
            false => Err(())
        }
    }
    //evil ext
    fn from_ok_comma(ok: bool, x: T) -> Result<T, ()> {
        Result::<T, E>::from_comma_ok(x, ok)
    }

    unsafe fn from_nullable(x: *const T) -> Result<T, ()>
        where T: Copy
    {
        if x.is_null() {
            Err(())
        } else {
            Ok(*x)
        }
    }
}

#[cfg(test)]
mod test {
    use core::panicking::panic;
    use std::panic;
    use std::ptr::null;
    use crate::ext::result::ResultExt;

    #[test]
    fn test_unwrap_or_eval() {
        let a = Ok::<i32, ()>(1).unwrap_or_eval(|_| 2);
        assert_eq!(1, a);
        let b = Err::<i32, ()>(()).unwrap_or_eval(|_| 2);
        assert_eq!(2, b);
    }

    #[test]
    fn test_unwrap_or_panic() {
        let a = Ok::<i32, ()>(1).unwrap_or_panic("");
        assert_eq!(1, a);
        let b = panic::catch_unwind(|| {
            Err::<i32, ()>(()).unwrap_or_panic("");
        });
        assert!(b.is_err());//TODO error msg test
    }

    #[test]
    fn test_or_pure() {
        let a = Ok(1).or_pure(|_: ()| 2).unwrap();
        assert_eq!(1, a);
        let b = Err(()).or_pure(|_: ()| 2).unwrap();
        assert_eq!(2, b);
    }

    #[test]
    fn test_or_bind() {
        let a = Ok(1).or_bind(|_: ()| Ok(2)).unwrap();
        assert_eq!(1, a);
        let b = Err(()).or_bind(|_: ()| Ok(1)).unwrap();
        assert_eq!(1, b)
    }

    #[test]
    fn test_from_comma_ok() {
        let a = Result::<i32, ()>::from_comma_ok(1, true).unwrap();
        assert_eq!(1, a);
        let b = Result::<i32, ()>::from_comma_ok(1, false);
        assert_eq!(Err(()), b)
    }

    #[test]
    fn test_from_ok_comma() {
        let a = Result::<i32, ()>::from_ok_comma(true, 1).unwrap();
        assert_eq!(1, a);
        let b = Result::<i32, ()>::from_ok_comma(false, 1);
        assert_eq!(Err(()), b);
    }

    #[test]
    fn test_from_nullable() {
        unsafe {
            let a = Result::<i32, ()>::from_nullable(&1 as *const i32).unwrap();
            assert_eq!(1, a);
            let b = Result::<i32, ()>::from_nullable(null::<i32>());
            assert_eq!(Err(()), b);
        }
    }
}