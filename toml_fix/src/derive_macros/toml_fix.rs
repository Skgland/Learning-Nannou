#![allow(unused_imports)]

use quote::{quote, quote_spanned};
use syn::{DeriveInput, Data::*, Ident, DataStruct, DataEnum};
use syn::Type;
use std::convert::identity;
use syn::Data;
use syn::spanned::Spanned;
use proc_macro2::TokenStream;
use syn::Variant;
use syn::Fields;
use quote::ToTokens;
use toml_fix_helpers::*;

const VARIANT: &'static str = "variant";
const CONTENT: &'static str = "content";

pub fn impl_toml_fix_macro(ast: &DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let mod_name = syn::Ident::new(&format!("toml_fix_{}", name), name.span());

    let content: TokenStream = match &ast.data {
        Struct(..) => {
            unimplemented!()
        }
        Enum(data) => {
            enum_fix::impl_toml_fix_macro_enum(&ast.ident, data)
        }
        Union(..) => {
            unimplemented!()
        }
    };

    let span = ast.span();


    let result = quote_spanned! {span=>
        mod #mod_name {
            #content
        }
    };
    result.into()
}

pub mod enum_fix {
    use super::*;
    use syn::{FieldsNamed, FieldsUnnamed};

    pub fn impl_toml_fix_macro_enum(ident: &Ident, data: &DataEnum) -> TokenStream {
        let mut impls = Vec::<TokenStream>::new();

        let variants: Vec<Enum> = data.variants.iter()
                                      .filter(|v| match v.fields {
                                          Fields::Unit => false,
                                          _ => true
                                      })
                                      .map(|v| impl_toml_fix_macro_enum_variant_as_struct(v)).collect();

        let structs = variants.iter().flat_map(|v| v.structs());

        impls.push(impl_toml_fix_macro_enum_serialize_impl(ident, &variants, data));
        impls.push(impl_toml_fix_macro_enum_deserialize_impl(ident, &variants, data));
        impls.push(impl_toml_fix_macro_enum_visitable_impl(ident,&variants,data));

        quote! {

            #(#structs)*

            #(#impls)*
        }
    }

    fn impl_toml_fix_macro_enum_deserialize_impl(ident: &Ident, variants: &Vec<Enum>, data: &DataEnum) -> TokenStream {

        let matches: Vec<TokenStream> = variants.iter().map(|v| v.impl_toml_fix_macro_enum_serialize_match(data)).collect();

        quote!{
            impl<'de> Deserialize<'de> for #ident {

                fn deserialize<D>(deserialize: D) -> Result<Self,D::Error> where D: Deserialize<'de> {
                    use toml_fix_helpers::EnumVisitor;

                    let visitor = EnumVisitor<#ident>;

                    deserializer.deserialize_struct("#ident","2",&[#VARIANT,#CONTENT],visitor);
                }
            }
        }

    }




fn impl_toml_fix_macro_enum_serialize_impl(ident: &Ident, variants: &Vec<Enum>, data: &DataEnum) -> TokenStream {
    let matches: Vec<TokenStream> = variants.iter()
                                            .map(|v| v.impl_toml_fix_macro_enum_serialize_match( data))
                                            .collect();

    quote! {
            impl Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {

                    let mut s = serializer.serialize_struct("#ident",2)?;

                    match self {
                        #(#matches),*
                    }

                    s.end()
                }
            }
        }
}

enum Enum<'v> {
    Unit(&'v Variant),
    Tuple(&'v Variant, TokenStream, Ident),
    //TODO needs a map for index to field
    Struct(&'v Variant, TokenStream, Ident),
}

impl <'v> Enum<'v> {

    fn structs(&self) -> Option<&TokenStream> {
        match self {
            Enum::Unit(..) => None,
            Enum::Tuple(_,tokens,_) => Some(tokens),
            Enum::Struct(_,tokens,_) => Some(tokens),
        }
    }

    fn impl_toml_fix_macro_enum_serialize_match(&self, data: &DataEnum) -> TokenStream {

        match self {
            Enum::Unit(variant) => {
                let ident = &variant.ident;
                quote! {
                    #ident => {
                        s.serialize_field(#VARIANT, #ident)?;
                        s.skip_field(#CONTENT)?;
                    }

                }
            },
            Enum::Tuple(variant,_,struct_ident) => {
                TokenStream::new()
            },
            Enum::Struct(variant,_,struct_ident) => {
                let fields_struct:Vec<TokenStream> = variant.fields.iter().map(|field| {
                    let ident = &field.ident;
                    quote!(#ident: *#ident)
                }).collect();

                let fields:Vec<&Ident> = variant.fields.iter().map(|field|  &field.ident.clone().unwrap()).collect();

                let ident = &variant.ident;
                quote! {
                    #ident{ #(#fields),* } => {
                        s.serialize_field(#VARIANT, #ident)?;
                        s.serialize_field(#CONTENT,&struct_ident{
                            #(#fields_struct),*
                        })
                    }
                }
            }
        }
    }
}

fn impl_toml_fix_macro_enum_variant_as_struct(variant: &Variant) -> Enum {
    let span = variant.span();
    let ident = syn::Ident::new(&format!("Variant{}Struct", &variant.ident), variant.ident.span());

    match &variant.fields {
        Fields::Unit => return Enum::Unit(variant),
        Fields::Unnamed(FieldsUnnamed { unnamed: _, .. }) => unimplemented!() /* TODO generated named Fields*/,
        Fields::Named(FieldsNamed { named, .. }) => {
            let named_fields = named.iter().map(|field| {
                let name = field.ident.clone().unwrap();
                let tipe = &field.ty;

                quote! {#name:#tipe}
            });

            Enum::Struct(variant,
            quote_spanned! {span=>

                #[derive(Serialize,Deserialize)]
                struct #ident {
                    #(#named_fields),*
                }

            },ident)
        },
    }
}
}
