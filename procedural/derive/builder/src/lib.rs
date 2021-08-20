extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn builder_macro(input: TokenStream) -> TokenStream {
  let input = parse::<DeriveInput>(input).unwrap();

  // Get current name
  let name = &input.ident;
  let new_name = Ident::new(&format!("{}Builder", name), name.span());

  // Generate new code
  let expanded = quote! {
    #[derive(Debug)]
    pub struct #new_name {

    }

    impl #name {
      pub fn builder() -> #new_name {
        #new_name{}
      }
    }
  };

  TokenStream::from(expanded)
}
