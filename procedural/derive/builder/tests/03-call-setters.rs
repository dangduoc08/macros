mod command;

#[test]
fn should_call_setters() {
  let mut builder = command::Command::builder();
  builder.executable("cargo".to_owned());
  builder.args(vec!["build".to_owned(), "--release".to_owned()]);
  builder.env(vec!["production"]);
  builder.current_dir(Some("..".to_owned()));
}
