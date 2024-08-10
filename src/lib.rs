#![crate_name = "small_iter_fields"]
//! ## Example
//! 
//! ```rust
//! use std::collections::HashMap;
//! use small_iter_fields::IterFields;
//! 
//! #[derive(IterFields, Hash, PartialEq, Eq)]
//! enum Stage {
//!     Start,
//!     Middle,
//!     End,
//! }
//!
//! let mut map: HashMap<Stage, Vec<i32>> = HashMap::new();
//! for stage in Stage::iter_fields() {
//!     map.insert(stage, Vec::new());
//! };
//! 
//! assert!(map.contains_key(&Stage::Start));
//! assert!(map.contains_key(&Stage::Middle));
//! assert!(map.contains_key(&Stage::End));
//! ```
//! ## Enums must have no data associated with it
//! ```compile_fail
//! use small_iter_fields::IterFields;
//! 
//! #[derive(IterFields)]
//! enum DataEnum {
//!     Data(bool),
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// To iterate over the fields of an enum.<br>
// -- want to add an exclude attribute
// -- want to do structs
#[proc_macro_derive(IterFields)]
pub fn derive_iter_fields(input: TokenStream) -> TokenStream {
   let input = parse_macro_input!(input as DeriveInput);
   let name = input.ident;

   let expanded = match input.data {
    Data::Enum(e) => {
        let num_fields = e.variants.len();
        let variants: Vec<_> = e.variants.into_iter().map(|v| v.ident).collect();
        quote! {
            impl #name {
                pub fn iter_fields() -> impl Iterator<Item = #name> {
                    vec![
                        #(#name::#variants), *
                    ].into_iter()
                }
            }
        }
    },
    _ => {
        panic!("Can only be used on enums");
    },
   };

   expanded.into()
}