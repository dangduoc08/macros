use super::object::{Human, Object};

// expr: an expression
// ty: a type
// block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
// ident: an identifier
// tt: a single token tree

// item: an item, like a function, struct, module, etc.
// stmt: a statement
// pat: a pattern
// path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
// meta: a meta item; the things that go inside #[...] and #![...] attributes
// vis:
// literal:
// lifetime:

#[macro_export]
macro_rules! with_expr {
  ($arg:expr) => {
    $arg + 10
  };
}

#[macro_export]
macro_rules! with_ident {
  ($arg:ident) => {
    println!("this is ident {}", $arg)
  };
}

#[macro_export]
macro_rules! with_ty {
  ($arg_1:ty, $arg_2:expr) => {
    <$arg_1>::from($arg_2)
  };
}

#[macro_export]
macro_rules! with_block {
  ($arg:block) => {
    $arg
  };
}

#[macro_export]
macro_rules! with_tt {
  ($arg:ty, $a:ident) => {
    <$arg>::call_self(&$a);
  };
}

#[macro_export]
macro_rules! with_path {
  ($arg:path) => {
    // return $arg {
    //   name: "Maxx",
    //   kind: "Dog",
    // }
  };
}

#[macro_export]
macro_rules! make_struct {
  ($item:tt, $($key:ident, $type:ty),*) => {
    #[derive(Debug)]
     struct $item {
      $($key: $type,)*
    }
  };
}

make_struct!(Book, kind, String);

pub fn run() {
  let sum: i32 = with_expr!(20);
  println!("sum with_expr! {}", sum);

  with_ident!(sum);

  let a_string = with_ty!(String, "Hello");
  println!("a_string with_ty! {}", a_string);

  with_block!({
    let number = 10;
    println!("number with_block! {}", number);
  });

  let someone = Human {
    name: "John Doe",
    age: 27,
  };

  with_tt!(Human, someone);

  let b = Book {
    kind: String::from("Novel"),
  };

  println!("{:?}", b);
}
