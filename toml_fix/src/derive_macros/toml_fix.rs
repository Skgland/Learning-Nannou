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
            use std::convert::{Into,TryFrom};
            use super::*;
            use ::toml_fix_helpers::{EnumTomlFixed,EnumVariant};
            use ::serde::{Serialize,Deserialize,Serializer,Deserializer};

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

        impls.push(impl_toml_fix_macro_enum_into(ident, &variants, data));
        impls.push(impl_toml_fix_macro_enum_try_from(ident, &variants, data));

        quote! {
            #(#structs)*

            #(#impls)*

            impl Serialize for #ident where Self: EnumTomlFixed {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error> where
                    S: Serializer {
                    let ev: EnumVariant<Self> = self.into();
                    ev.serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for #ident where Self: EnumTomlFixed {
               fn deserialize<D>(deserializer: D) -> Result<Self,D::Error> where
                    D: Deserializer<'de> {
                    let ev = EnumVariant::<Self>::deserialize(deserializer)?;
                    Self::try_from(ev)
                }
            }

        }
    }

    fn impl_toml_fix_macro_enum_into(ident:&Ident,variants:&Vec<Enum>,data:&DataEnum) -> TokenStream{

        quote!{
            impl Into<EnumVariant> for #ident {
                fn into(self) -> EnumVariant{
                    unimplemented!()
                }
            }
        }
    }

    fn impl_toml_fix_macro_enum_try_from(ident:&Ident,variants:&Vec<Enum>,data:&DataEnum) -> TokenStream{

        quote!{
            impl TryFrom<EnumVariant> for #ident  {
                type Error = Result<>;
                fn try_from(variant:EnumVariant) -> Result<Self,<Self as TryFrom<EnumVariant>>::Error> {
                    unimplemented!()
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

    impl<'v> Enum<'v> {
        fn structs(&self) -> Option<&TokenStream> {
            match self {
                Enum::Unit(..) => None,
                Enum::Tuple(_, tokens, _) => Some(tokens),
                Enum::Struct(_, tokens, _) => Some(tokens),
            }
        }

        fn impl_toml_fix_macro_enum_into_match(&self, data: &DataEnum) -> TokenStream {
            match self {
                Enum::Unit(variant) => {
                    let ident = &variant.ident;
                    let str = format!("{}",ident);
                    quote! {
                        #ident => {
                            EnumVariant(String::new(#str,""))
                        }

                    }
                }
                Enum::Tuple(variant, _, struct_ident) => {


                    let ident = &variant.ident;
                    let str = format!("{}",ident);

                    unimplemented!();

                    quote!{
                        #ident(...) => {
                            let content = #struct_ident{}
                            EnumVariant(String::new(#str, ... ))
                        }
                    }
                }
                Enum::Struct(variant, _, struct_ident) => {
                    let fields_struct: Vec<TokenStream> = variant.fields.iter().map(|field| {
                        let ident = &field.ident;
                        quote!(#ident: *#ident)
                    }).collect();

                    let fields: Vec<Ident> = variant.fields.iter().map(|field| field.ident.clone().unwrap()).collect();

                    let ident = &variant.ident;
                    let str = format!("{}",ident);

                    quote! {
                        #ident{ #(#fields),* } => {
                            let content = #struct_ident{#(#fields_struct),*}
                            EnumVariant(String::new(#str, content.serialize(unimplemented!()) ))
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

            }, ident)
            }
        }
    }
}
