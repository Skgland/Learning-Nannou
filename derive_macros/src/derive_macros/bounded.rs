use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data::*, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Variant};

pub fn impl_bounded_trait_derive(ast: &DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let span = ast.span();

    let min = get_min_for_ast(ast);
    let max = get_max_for_ast(ast);

    let result = quote_spanned! { span=>

        impl derive_macros_helpers::Bounded for #name {

            fn minimum() -> Self {
                #min
            }

            fn maximum() -> Self {
                #max
            }

        }
    };
    result.into()
}

fn get_min_for_ast(ast: &DeriveInput) -> TokenStream {
    match &ast.data {
        Enum(data) => {
            if let Some(variant) = data.variants.iter().find(|var| !has_skip_attribute(var)) {
                impl_enum::get_min_for_enum_variant(variant)
            } else {
                panic!(
                    "Could not determine minimum for {} all Variants were skipped!",
                    &ast.ident
                )
            }
        }
        Struct(data) => impl_struct::get_min_for_struct(&ast.ident, data),
        Union(..) => panic!("Derive of Bounded not available for Unions!"),
    }
}

fn get_max_for_ast(ast: &DeriveInput) -> TokenStream {
    match &ast.data {
        Enum(data) => {
            if let Some(variant) = data
                .variants
                .iter()
                .collect::<Vec<&Variant>>()
                .iter()
                .rev()
                .find(|var| !has_skip_attribute(var))
            {
                impl_enum::get_max_for_enum_variant(variant)
            } else {
                panic!(
                    "Could not determine maximum for {} all Variants were skipped!",
                    &ast.ident
                )
            }
        }
        Struct(data) => impl_struct::get_max_for_struct(&ast.ident, data),
        Union(..) => panic!("Derive of Bounded not available for Unions!"),
    }
}

pub mod impl_enum {
    use super::*;
    pub fn get_min_for_enum_variant(variant: &Variant) -> TokenStream {
        let name = &variant.ident;

        match &variant.fields {
            Fields::Unit => {
                quote! {
                    Self::#name
                }
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let (idents, types): (Vec<_>, Vec<_>) = named
                    .iter()
                    .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
                    .unzip();
                quote! {
                    Self::#name{#(#idents: <#types as Bounded>::minimum()),*}
                }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let types = unnamed.iter().map(|field| &field.ty);
                quote! {
                    Self::#name( #(<#types as Bounded>::minimum()),* )
                }
            }
        }
    }

    pub fn get_max_for_enum_variant(variant: &Variant) -> TokenStream {
        let name = &variant.ident;

        match &variant.fields {
            Fields::Unit => {
                quote! {
                    Self::#name
                }
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let (idents, types): (Vec<_>, Vec<_>) = named
                    .iter()
                    .map(|field| (field.ident.as_ref().unwrap().clone(), field.ty.clone()))
                    .unzip();
                quote! {
                    Self::#name{#(#idents: <#types as Bounded>::maximum()),*}
                }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let types = unnamed.iter().map(|field| &field.ty);
                quote! {
                    Self::#name( #(<#types as Bounded>::maximum()),* )
                }
            }
        }
    }
}

mod impl_struct {
    use super::*;

    pub fn get_min_for_struct(struct_ident: &Ident, struct_data: &DataStruct) -> TokenStream {
        match &struct_data.fields {
            Fields::Unit => {
                quote! {
                    #struct_ident
                }
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let (names, types): (Vec<_>, Vec<_>) = named
                    .iter()
                    .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
                    .unzip();
                quote! {
                    #struct_ident{#(#names:<#types as Bounded>::minimum()),*}
                }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let types = unnamed.iter().map(|field| &field.ty);
                quote! {
                    #struct_ident(#(<#types as Bounded>::minimum()),*)
                }
            }
        }
    }

    pub fn get_max_for_struct(struct_ident: &Ident, struct_data: &DataStruct) -> TokenStream {
        match &struct_data.fields {
            Fields::Unit => {
                quote! {
                    #struct_ident
                }
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let (names, types): (Vec<_>, Vec<_>) = named
                    .iter()
                    .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
                    .unzip();
                quote! {
                    #struct_ident{#(#names:<#types as Bounded>::maximum()),*}
                }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let types = unnamed.iter().map(|field| &field.ty);
                quote! {
                    #struct_ident(#(<#types as Bounded>::maximum()),*)
                }
            }
        }
    }
}

fn has_skip_attribute(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| {
        attr.path()
            .segments
            .first()
            .map(|pair| pair.ident == "skip")
            .unwrap_or(false)
    })
}
