pub trait OptionExt<T> {
    fn unwrap_or_eval<F>(self, f: F) -> T
        where F: Fn() -> T;
    fn unwrap_or_panic(self, msg: &str) -> T;

    fn or_pure<F>(self, f: F) -> Option<T>
        where F: Fn() -> T;
    fn or_bind<F>(self, f: F) -> Option<T>
        where F: Fn() -> Option<T>;

    //evil ext
    fn from_comma_ok(x: T, ok: bool) -> Option<T>;
    //evil ext
    fn from_ok_comma(ok: bool, x: T) -> Option<T>;

    //TODO is unsafe?
    unsafe fn from_nullable(x: *const T) -> Option<T>
        where T: Copy;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_or_eval<F>(self, f: F) -> T
        where F: Fn() -> T
    {
        self.unwrap_or_else(f)
    }

    fn unwrap_or_panic(self, msg: &str) -> T {
        match self {
            Some(x) => x,
            None => panic!("{}", msg)//TODO {msg}?
        }
    }

    fn or_pure<F>(self, f: F) -> Option<T>
        where F: Fn() -> T
    {
        match self {
            Some(x) => Some(x),
            None => Some(f())
        }
    }
    fn or_bind<F>(self, f: F) -> Option<T>
        where F: Fn() -> Option<T>
    {
        match self {
            Some(x) => Some(x),
            None => f()
        }
    }

    //evil ext
    fn from_comma_ok(x: T, ok: bool) -> Option<T> {
        match ok {
            true => Some(x),
            false => None
        }
    }
    //evil ext
    fn from_ok_comma(ok: bool, x: T) -> Option<T> {
        Option::<T>::from_comma_ok(x, ok)
    }

    unsafe fn from_nullable(x: *const T) -> Option<T>
        where T: Copy
    {
        if x.is_null() {
            None
        } else {
            Some(*x)
        }
    }
}

#[cfg(test)]
mod test {
    use core::panicking::panic;
    use std::panic;
    use std::ptr::null;
    use crate::ext::option::OptionExt;

    #[test]
    fn test_unwrap_or_eval() {
        let a = Some(1).unwrap_or_eval(|| 2);
        assert_eq!(1, a);
        let b = None.unwrap_or_eval(|| 2);
        assert_eq!(2, b);
    }

    #[test]
    fn test_unwrap_or_panic() {
        let a = Some(1).unwrap_or_panic("");
        assert_eq!(1, a);
        let b = panic::catch_unwind(|| {
            None::<i32>.unwrap_or_panic("");
        });
        assert!(b.is_err());//TODO error msg test
    }

    #[test]
    fn test_or_pure() {
        let a = Some(1).or_pure(|| 2).unwrap();
        assert_eq!(1, a);
        let b = None::<i32>.or_pure(|| 2).unwrap();
        assert_eq!(2, b);
    }

    #[test]
    fn test_or_bind() {
        let a = Some(1).or_bind(|| Some(2)).unwrap();
        assert_eq!(1, a);
        let b = None.or_bind(|| Some(1)).unwrap();
        assert_eq!(1, a);
    }

    #[test]
    fn test_from_comma_ok() {
        let a = Option::<i32>::from_comma_ok(1, true).unwrap();
        assert_eq!(1, a);
        let b = Option::<i32>::from_comma_ok(1, false);
        assert_eq!(None, b);
    }

    #[test]
    fn test_from_ok_comma() {
        let a = Option::<i32>::from_ok_comma(true, 1).unwrap();
        assert_eq!(1, a);
        let b = Option::<i32>::from_ok_comma(false, 1);
        assert_eq!(None, b);
    }

    #[test]
    fn test_from_nullable() {
        unsafe {
            let a = Option::<i32>::from_nullable(&1 as *const i32).unwrap();
            assert_eq!(1, a);
            let b = Option::<i32>::from_nullable(null::<i32>());
            assert_eq!(None, b);
        }
    }
}
