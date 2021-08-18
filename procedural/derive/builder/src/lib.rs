extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn builder_macro(input: TokenStream) -> TokenStream {
  let input = parse::<DeriveInput>(input).unwrap();
  let name = &input.ident;
  let new_name = Ident::new(&format!("{}Builder", name), name.span());

  let expanded = quote! {
    struct #new_name {

    }
  };

  TokenStream::from(expanded)
  // expanded.into()
}
