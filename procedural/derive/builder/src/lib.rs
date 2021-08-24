extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};

#[proc_macro_derive(Builder)]
pub fn builder_macro(input: TokenStream) -> TokenStream {
  // Parse tokenstream
  let parsed_input: DeriveInput = parse::<DeriveInput>(input).unwrap();

  // Get struct fields from derived struct
  let original_struct_fields = if let Data::Struct(DataStruct {
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
      #original_struct_fields
    }

    impl #original_struct_name {
      pub fn builder() -> #new_struct_name {
        #new_struct_name{
          executable: String::from(""),
          args: vec![String::from("")],
          env: vec![""],
          current_dir: Some(String::from("")),
        }
      }
    }

    impl #new_struct_name {
      pub fn executable(&mut self, new_executable: String) {
        self.executable = new_executable
      }

      pub fn args(&mut self, new_args: Vec::<String>) {
        self.args = new_args
      }

      pub fn env(&mut self, new_env: Vec<&'static str>) {
        self.env = new_env
      }

      pub fn current_dir(&mut self, new_dir: Option<String>) {
        self.current_dir = new_dir
      }
    }
  };

  TokenStream::from(expanded)
}
