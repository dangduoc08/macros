use builder::Builder;

#[derive(Builder)]
pub struct Command {
  pub executable: String,
  pub args: Vec<String>,
  pub env: Vec<&'static str>,
  pub current_dir: Option<String>,
}
