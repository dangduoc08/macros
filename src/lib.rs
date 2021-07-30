pub mod human;

// expr: an expression
// ty: a type

// item: an item, like a function, struct, module, etc.
// block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
// stmt: a statement
// pat: a pattern
// ident: an identifier
// path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
// meta: a meta item; the things that go inside #[...] and #![...] attributes
// tt: a single token tree

trait Object {
  fn call_self(&self) {
    println!("Default call self");
  }
}

impl<'a> Object for human::Human<'a> {
  fn call_self(&self) {
    println!("Call myself {}", &self.name);
  }
}

impl<'a> Object for Animal<'a> {}

#[macro_export]
macro_rules! with_expr {
  ($arg:expr) => {
    $arg + 10
  };
}

#[macro_export]
macro_rules! with_ty {
  ($arg_1:ty, $arg_2:expr) => {
    <$arg_1>::from($arg_2)
  };
}

#[macro_export]
macro_rules! with_item {
  ($($args:item),*) => {
    // type SW = $args;
    // $(
    //   // <$args>::call_self($args);
    // )*

    10
  };
}

#[test]
fn macr() {
  let sum: i32 = with_expr!(20);
  assert_eq!(30, sum);

  let a_string = with_ty!(String, "Hello");
  assert_eq!(String::from("Hello"), a_string);

  let a_dog = Animal {
    name: "Maxx",
    kind: "Dog",
  };

  let a_person = human::Human {
    name: "John Doe",
    age: 27,
  };
  with_item!();

  assert_eq!(
    10,
    with_item!(
      fn test() -> isize {
        1 + 2
      }
    )
  )
}
