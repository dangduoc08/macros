#[derive(Debug)]
pub struct Human<'a> {
  pub name: &'a str,
  pub age: i8,
}

#[derive(Debug)]
pub struct Animal<'a> {
  pub name: &'a str,
  pub kind: &'a str,
}

pub trait Object {
  fn call_self(&self) {
    println!("Default call itself");
  }
}

impl<'a> Object for Human<'a> {
  fn call_self(&self) {
    println!("Implemented call itself {}", &self.name);
  }
}

impl<'a> Object for Animal<'a> {}
