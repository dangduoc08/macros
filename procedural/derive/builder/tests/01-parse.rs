mod command;

#[test]
fn should_be_parse() {
  let docker_cli: command::Command = command::Command {
    executable: String::from("docker container run postgres"),
    args: vec![String::from("-d"), String::from("--rm")],
    env: vec!["POSTGRES_PASSWORD=psqlpwd"],
    current_dir: Some(String::from("./")),
  };

  assert_eq!(docker_cli.executable, "docker container run postgres");
}
