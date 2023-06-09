use extension_trait::extension_trait;

#[extension_trait]
impl Between for str {
    fn between(&self, front: &str, end: &str) -> Option<&str> {
        self.after(front).and_then(|t| t.before(end))
    }

    fn before(&self, end: &str) -> Option<&str> {
        self.split(end).next()
    }

    fn after(&self, front: &str) -> Option<&str> {
        self.splitn(2, front).nth(1)
    }
}

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
    use super::extension_trait;

    #[extension_trait]
    pub impl Public for i32 {
        fn method(self) -> i32 {
            42
        }
    }
}

#[test]
fn pub_extension_trait() {
    use self::x::Public;
    assert_eq!(24i32.method(), 42);
}

#[extension_trait]
pub impl<T> Length for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

#[test]
fn generic_extension_traits() {
    assert_eq!(vec!["q"].size(), 1);
}

#[extension_trait]
pub impl ReturnArgument for () {
    fn return_argument<T>(&self, arg: T) -> String
    where
        T: ToString,
    {
        arg.to_string()
    }
}

#[test]
fn generic_function_extension_traits() {
    assert_eq!(().return_argument(42), "42");
}

#[extension_trait]
pub impl<T: Copy + Into<String>> DoubleBracketConversion for Vec<T> {
    fn first_into_string(&self) -> String {
        self[0].into()
    }
}

#[test]
fn double_bracket_conversion() {
    assert_eq!(vec!["asdf"].first_into_string(), String::from("asdf"));
}

#[extension_trait]
pub impl<T> DoubleBracketConversionUsingWhere for Vec<T>
where
    T: Copy + Into<String>,
{
    fn first_into_string_using_where(&self) -> String {
        self[0].into()
    }
}

#[test]
fn double_bracket_conversion_using_where() {
    assert_eq!(
        vec!["asdf"].first_into_string_using_where(),
        String::from("asdf")
    );
}

#[extension_trait]
pub impl<'a, T> Unsafe for (&'a mut T, &'a mut T) {
    unsafe fn swap(&mut self) {
        std::ptr::swap(self.0, self.1)
    }
}

#[test]
fn unsafe_method() {
    let mut a = 1;
    let mut b = 2;
    unsafe {
        (&mut a, &mut b).swap();
    }
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}

#[extension_trait]
pub impl<T: Copy> SliceMapExt<T> for [T] {
    fn map_in_place<F: FnMut(T) -> T>(&mut self, mut f: F) {
        for v in self {
            *v = f(*v);
        }
    }
}

#[test]
fn slice_map_ext() {
    let mut values = [1, 2, 3];
    values.map_in_place(|x| x + 1);
    assert_eq!(values, [2, 3, 4]);
}

/// This extension trait is documented
#[extension_trait]
pub impl Documented for () {
    /// A function is documented too.
    fn documented(self) {}
}

#[test]
fn documented() {
    assert_eq!(().documented(), ());
}

#[extension_trait]
impl PlusTwoReceiver for i32 {
    fn plus_two(mut self: Self) -> i32 {
        self += 2;
        self
    }
}

#[test]
fn receiver() {
    assert_eq!(2.plus_two(), 4);
}

#[extension_trait]
impl EmptyTuple for i32 {
    fn hi((): ()) {}
}

#[test]
fn empty_tuple() {
    i32::hi(());
}

#[extension_trait]
impl SingleElementTuple for i32 {
    fn hello((r#if,): (i32,)) -> i32 {
        r#if
    }
}

#[test]
fn single_element_tuple() {
    assert_eq!(i32::hello((4,)), 4);
}

#[extension_trait]
impl Tuple for i32 {
    fn tuple((r#as, r#if): (i32, i32)) -> i32 {
        r#as + r#if
    }
}

#[test]
fn two_element_tuple() {
    assert_eq!(i32::tuple((4, 5)), 9);
}

#[extension_trait]
impl TupleHygiene for i32 {
    fn tuple_higiene((a, b): (i32, i32), a_b: i32) -> i32 {
        a + b + a_b
    }
}

#[test]
fn tuple_higiene() {
    assert_eq!(i32::tuple_higiene((1, 2), 4), 7);
}

struct X {
    a: i32,
    b: i32,
}

#[extension_trait]
impl StructPattern for i32 {
    #[intrait]
    fn x(X { a, b }: X) -> i32 {
        a + b
    }
}

#[test]
fn struct_pattern() {
    assert_eq!(i32::x(X { a: 1, b: 2 }), 3);
}

#[extension_trait(Clone)]
pub impl IUncontrolledProviderScope for u32 {
    type ProviderContainer: PartialEq = u32;
}
