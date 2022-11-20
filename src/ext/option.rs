pub trait OptionExt<T> {
    fn unwrap_or_eval<F>(self, f: F) -> T
        where F: Fn() -> T;
    fn unwrap_or_panic<F>(self, msg: &str) -> T;

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

    fn unwrap_or_panic<F>(self, msg: &str) -> T {
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