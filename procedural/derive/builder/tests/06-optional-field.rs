mod command;

#[test]
fn should_has_optional_fields() {
  let command = command::Command::builder()
    .executable("cargo".to_owned())
    .args(vec!["build".to_owned(), "--release".to_owned()])
    .env(vec![])
    .current_dir(Some(String::from("current")))
    .build()
    .unwrap();

  assert!(command.current_dir.is_none());

  let command = command::Command::builder()
    .executable("cargo".to_owned())
    .args(vec!["build".to_owned(), "--release".to_owned()])
    .env(vec![])
    .current_dir(Some("..".to_owned()))
    .build()
    .unwrap();
  assert!(command.current_dir.is_some());
}
