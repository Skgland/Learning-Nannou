#![recursion_limit = "128"]
extern crate proc_macro;
extern crate derive_macros_helpers;

use proc_macro::TokenStream;
use syn;

mod derive_macros;

#[proc_macro_derive(Bounded,attributes(skip))]
pub fn bounded_trait_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    let result = derive_macros::traits::bounded::impl_bounded_trait_derive(&ast);
    //panic!(result.to_string());
    result

}

#[proc_macro_derive(Enumerable, attributes(skip))]
pub fn enumerable_trait_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    let result = derive_macros::traits::enumerable::impl_enumerable_trait_derive(&ast);
    //panic!(result.to_string());
    result
}
