mod command;

#[test]
fn should_use_method_chainning() {
  let command = command::Command::builder()
    .executable("cargo".to_owned())
    .args(vec!["build".to_owned(), "--release".to_owned()])
    .env(vec![])
    .current_dir(Some("..".to_owned()))
    .build()
    .unwrap();

  assert_eq!(command.executable, "cargo");
}
