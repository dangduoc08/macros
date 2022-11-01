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

  // Generate Option value from orignal fields
  let trans_fields = original_struct_fields.iter().map(|field| {
    let ident = &field.ident;
    let ty = &field.ty;

    quote! { pub #ident: Option::<#ty> }
  });

  // tạo struct field động chỗ này, init value = None
  let none_fiels = original_struct_fields;

  let trans_methods = original_struct_fields.iter().map(|field| {
    let ident = &field.ident;
    let ty = &field.ty;
    let arg = Ident::new(
      &format!("new_{}", original_struct_name),
      original_struct_name.span(),
    );

    quote! {
      pub fn #ident(&mut self, #arg: #ty) -> &mut Self {
        self.#ident = Some(#arg);
        self
      }
    }
  });

  // Generate new tokenstream
  let expanded = quote! {
      #[derive(Debug)]
      pub struct #new_struct_name {
        #(#trans_fields,)*
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
      #(#trans_methods)*
      
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
