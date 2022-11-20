pub trait ResultExt<T, E> {
    fn unwrap_or_eval<F>(self, f: F) -> T
        where F: Fn(E) -> T;
    fn unwrap_or_panic<F>(self, msg: &str) -> T;

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

    fn unwrap_or_panic<F>(self, msg: &str) -> T {
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
