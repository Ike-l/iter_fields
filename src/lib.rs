#![crate_name = "small_iter_fields"]
//! ## Example
//! 
//! ```rust
//! use std::collections::HashMap;
//! use small_iter_fields::{IterFields, LenFields};
//! 
//! #[derive(IterFields, LenFields, Hash, PartialEq, Eq)]
//! enum Stage {
//!     Start,
//!     Middle,
//!     End,
//! }
//!
//! let mut vec: Vec<Stage> = Vec::with_capacity(Stage::len());
//! assert!(vec.capacity() >= 3);
//! 
//! for stage in Stage::iter_fields() {
//!     vec.push(stage);
//! };
//! 
//! assert!(vec.contains(&Stage::Start));
//! assert!(vec.contains(&Stage::Middle));
//! assert!(vec.contains(&Stage::End));
//! 
//! let map: HashMap<Stage, Vec<i32>> = Stage::to_hashmap(Vec::new());
//! assert!(map.capacity() >= 3);
//! 
//! assert_eq!(map.get(&Stage::Start), Some(&Vec::new()));
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
use syn::{parse_macro_input, Data, DeriveInput};

/// Iterate over the fields of an enum.<br>
// -- want to add an exclude attribute
// -- want to do structs
#[proc_macro_derive(IterFields)]
pub fn derive_iter_fields(input: TokenStream) -> TokenStream {
   let input = parse_macro_input!(input as DeriveInput);
   let name = input.ident;

   let expanded = match input.data {
    Data::Enum(e) => {
        let variants: Vec<_> = e.variants.into_iter().map(|v| v.ident).collect();
        quote! {
            impl #name {
                /// # Examples
                /// 
                /// ```
                /// use std::collections::HashMap;
                /// use small_iter_fields::IterFields;
                /// 
                /// #[derive(IterFields, Hash, PartialEq, Eq)]
                /// enum Stage {
                ///     Start,
                ///     Middle,
                ///     End,
                /// }
                /// 
                /// let map: HashMap<Stage, Vec<i32>> = Stage::to_hashmap(Vec::new());
                /// ```
                pub fn to_hashmap<T: Clone>(value: T) -> HashMap<Self, T> {
                    HashMap::from_iter(Self::iter_fields().map(|field| (field, value.clone())))
                }

                /// # Examples
                /// 
                /// ```
                /// use small_iter_fields::{IterFields, LenFields};
                /// 
                /// #[derive(IterFields, LenFields)]
                /// enum Stage {
                ///     Start,
                ///     Middle,
                ///     End,
                /// }
                /// 
                /// let mut vec: Vec<Stage> = Vec::with_capacity(Stage::len());
                /// for stage in Stage::iter_fields() {
                ///     vec.push(stage);
                /// };
                /// ```
                pub fn iter_fields() -> impl Iterator<Item = #name> {
                    vec![
                        #(#name::#variants), *
                    ].into_iter()
                }
            }
        }
    },
    _ => panic!("Can only be used on enums")
   };

   expanded.into()
}

/// Get how many variants in an enum as usize.<br>
#[proc_macro_derive(LenFields)]
pub fn derive_len_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = match input.data {
        Data::Enum(e) => {
            let num = e.variants.len();
            quote! {
                impl #name {
                    /// # Examples
                    /// 
                    /// ```
                    /// use small_iter_fields::LenFields;
                    /// 
                    /// #[derive(LenFields)]
                    /// enum Stage {
                    ///     Start,
                    ///     Middle,
                    ///     End,
                    /// }
                    /// 
                    /// let mut vec: Vec<Stage> = Vec::with_capacity(Stage::len());
                    /// ```
                    pub fn len() -> usize {
                        #num
                    }
                }
            }
        },
        _ => panic!("Can only be used on enums")
    };

    expanded.into()
}

