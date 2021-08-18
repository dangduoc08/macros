// use macros::declarative;
use derive_macro::Builder;

#[derive(Builder)]
struct Command<'a> {
  executable: String,
  args: Vec<String>,
  env: Vec<String>,
  current_dir: &'a str,
}

fn main() {
  let _docker_cli: Command = Command {
    executable: String::from("docker container run postgres"),
    args: vec![String::from("-d"), String::from("--rm")],
    env: vec![String::from("POSTGRES_PASSWORD=mysecretpassword")],
    current_dir: "./",
  };
}
