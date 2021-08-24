mod command;

#[test]
fn should_call_build() {
  let mut builder = command::Command::builder();
  builder.executable("cargo".to_owned());
  builder.args(vec!["build".to_owned(), "--release".to_owned()]);
  builder.env(vec![]);
  builder.current_dir(Some("..".to_owned()));

  let command = builder.build().unwrap();
  assert_eq!(command.executable, "cargo");
}
