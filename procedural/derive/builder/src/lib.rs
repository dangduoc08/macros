extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};

#[proc_macro_derive(Builder)]
pub fn builder_macro(input: TokenStream) -> TokenStream {
  // Parse tokenstream
  let parsed_input: DeriveInput = parse::<DeriveInput>(input).unwrap();

  // Get struct fields from derived struct
  let _original_struct_fields = if let Data::Struct(DataStruct {
    fields: Fields::Named(FieldsNamed { ref named, .. }),
    ..
  }) = parsed_input.data
  {
    named
  } else {
    unimplemented!()
  };

  // Get current name
  let original_struct_name = &parsed_input.ident;

  // Span for ref to original one
  // when error happen, i think
  let new_struct_name = Ident::new(
    &format!("{}Builder", original_struct_name),
    original_struct_name.span(),
  );

  // Generate new tokenstream
  let expanded = quote! {
    #[derive(Debug)]
    pub struct #new_struct_name {
      // #original_struct_fields
      pub executable: Option::<String>,
      pub args: Option::<Vec::<String>>,
      pub env: Option::<Vec::<&'static str>>,
      pub current_dir: Option::<Option::<String>>,
    }

    impl #original_struct_name {
      pub fn builder() -> #new_struct_name {
        #new_struct_name{
          executable: None,
          args: None,
          env: None,
          current_dir: None,
        }
      }
    }

    impl #new_struct_name {
      pub fn executable(&mut self, new_executable: String) -> &mut Self {
        self.executable = Some(new_executable);
        self
      }

      pub fn args(&mut self, new_args: Vec::<String>) -> &mut Self {
        self.args = Some(new_args);
        self
      }

      pub fn env(&mut self, new_env: Vec::<&'static str>) -> &mut Self {
        self.env = Some(new_env);
        self
      }

      pub fn current_dir(&mut self, new_dir: Option::<String>) -> &mut Self {
        self.current_dir = Some(new_dir);
        self
      }

      pub fn build(&self) -> Result::<#original_struct_name, String> {
        Ok(#original_struct_name{
          executable: self.executable.clone().ok_or("failed to get executable").unwrap(),
          args: self.args.clone().ok_or("failed to get args").unwrap(),
          env: self.env.clone().ok_or("failed to get env").unwrap(),
          current_dir: self.current_dir.clone().ok_or("failed to get current_dir").unwrap(),
        })
      }
    }
  };

  TokenStream::from(expanded)
}
