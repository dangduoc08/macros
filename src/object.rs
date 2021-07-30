#[derive(Debug)]
pub struct Human<'a> {
  pub name: &'a str,
  pub age: i8,
}

#[derive(Debug)]
struct Animal<'a> {
  pub name: &'a str,
  pub kind: &'a str,
}