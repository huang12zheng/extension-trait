#[macro_use]
extern crate extension_trait;

extension_trait! { Between for str {
    fn between(&self, front: &str, end: &str) -> Option<&str> {
        self.after(front).and_then(|t| t.before(end))
    }

    fn before(&self, end: &str) -> Option<&str> {
        self.split(end).next()
    }

    fn after(&self, front: &str) -> Option<&str> {
        self.splitn(2, front).nth(1)
    }
} }

#[test]
fn between_works() {
    assert_eq!("<a>".between("<", ">"), Some("a"));
}

#[test]
fn before_works() {
    assert_eq!("<a>".before(">"), Some("<a"));
}

#[test]
fn after_works() {
    assert_eq!("<a>".after("a"), Some(">"));
}

mod x {
    extension_trait! { pub Public for i32 {
        fn method(self) -> i32 {
            42
        }
    } }
}

#[test]
fn pub_extension_trait() {
    use x::Public;
    assert_eq!(24i32.method(), 42);
}

extension_trait! { <T> pub Length for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
} }

#[test]
fn generic_extension_traits() {
    assert_eq!(vec!["q"].size(), 1);
}

extension_trait! { pub ReturnArgument for () {
    fn return_argument<T>(&self, arg: T) -> T
    where
        T: Debug,
    {
        arg
    }
} }

#[test]
fn generic_function_extension_traits() {
    assert_eq!(().return_argument(42), 42);
}
