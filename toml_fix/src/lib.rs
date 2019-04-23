#![recursion_limit = "128"]
extern crate proc_macro;
extern crate toml_fix_helpers;

use proc_macro::TokenStream;
use syn;

mod derive_macros;

#[proc_macro_derive(TomlFix)]
pub fn toml_fix_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    derive_macros::toml_fix::impl_toml_fix_macro(&ast)
}
