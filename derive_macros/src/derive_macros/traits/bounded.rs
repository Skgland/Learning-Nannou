use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use proc_macro2::TokenStream;
use syn::{DeriveInput, Data::*, Ident, DataStruct, DataEnum, Variant, Fields, FieldsNamed, FieldsUnnamed};

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
            if let Some(variant) = data.variants.iter()
                .find(|var| !has_skip_attribute(var)) {
                impl_enum::get_min_for_enum_variant(&ast.ident, variant)
            } else {
                panic!("Could not determine minimum for {}!", &ast.ident)
            }
        }
        Struct(data) => {
            impl_struct::get_min_for_struct(data)
        }
        Union(..) => {
            unimplemented!()
        }
    }
}


fn get_max_for_ast(ast: &DeriveInput) -> TokenStream {
    match &ast.data {
        Enum(data) => {
            if let Some(variant) = data.variants.iter().collect::<Vec<&Variant>>().iter().rev()
                .find(|var| !has_skip_attribute(var)) {
                impl_enum::get_max_for_enum_variant(&ast.ident, variant)
            } else {
                panic!("Could not determine minimum for {}!", &ast.ident)
            }
        }
        Struct(data) => {
            impl_struct::get_max_for_struct(data)
        }
        Union(..) => {
            panic!("Derive of Bounded not available for Unions!")
        }
    }
}

mod impl_enum {
    use super::*;
    pub fn get_min_for_enum_variant(enum_ident: &Ident, variant: &Variant) -> TokenStream {
        let name = &variant.ident;

        match &variant.fields {
            Fields::Unit => {
                quote! {
                #enum_ident::#name
            }
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let (idents, types): (Vec<_>, Vec<_>) = named.iter()
                    .map(|field| (field.ident.as_ref().unwrap().clone(), field.ty.clone())).unzip();
                quote! {
                #enum_ident::#name{#(#idents: <#types as Bounded>::minimum()),*}
            }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let types = unnamed.iter().map(|field| &field.ty);
                quote! {
                #enum_ident::#name( #(<#types as Bounded>::minimum()),* )
            }
            }
        }
    }


    pub fn get_max_for_enum_variant(enum_ident: &Ident, variant: &Variant) -> TokenStream {
        let name = &variant.ident;

        match &variant.fields {
            Fields::Unit => {
                quote! {
                #enum_ident::#name
            }
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let (idents, types): (Vec<_>, Vec<_>) = named.iter()
                    .map(|field| (field.ident.as_ref().unwrap().clone(), field.ty.clone())).unzip();
                quote! {
                #enum_ident::#name{#(#idents: <#types as Bounded>::maximum()),*}
            }
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let types = unnamed.iter().map(|field| &field.ty);
                quote! {
                #enum_ident::#name( #(<#types as Bounded>::maximum()),* )
            }
            }
        }
    }
}

mod impl_struct {
    use super::*;

    pub fn get_min_for_struct(struct_data:&DataStruct)-> TokenStream{}

    pub fn get_max_for_struct(struct_data:&DataStruct)-> TokenStream{}

}

fn has_skip_attribute(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| attr.path.segments.first().map(|pair| pair.into_value().ident.to_string() == "skip").unwrap_or(false))
}