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

pub fn impl_toml_fix_macro(ast: &DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let mod_name = syn::Ident::new(&format!("toml_fix_{}", name).to_lowercase(), name.span());

    let content: TokenStream = match &ast.data {
        Struct(..) => {
            unimplemented!()
        }
        Enum(data) => {
            enum_fix::impl_toml_fix_macro_enum(name, data)
        }
        Union(..) => {
            unimplemented!()
        }
    };

    let span = ast.span();

    let result = quote_spanned! {span=>
        mod #mod_name {
            #![allow(unused_imports)]

            //TomlFix imports
            use std::convert::{Into,TryFrom,TryInto};
            use super::*;
            use super::#name::*;
            use ::toml_fix_helpers::{EnumTomlFixed,EnumVariant};
            use ::serde::{Serialize,Deserialize,Serializer,Deserializer,de::Error as _ , ser::Error as _};

            #content
        }
    };

    result.into()
}

pub mod enum_fix {
    use super::*;
    use syn::{FieldsNamed, FieldsUnnamed, Attribute, Path, PathSegment, Field};
    use proc_macro2::Span;

    pub fn impl_toml_fix_macro_enum(ident: &Ident, data: &DataEnum) -> TokenStream {
        let mut impls = Vec::<TokenStream>::new();

        let variants: Vec<Enum> = data.variants.iter()
                                      .map(|v| impl_toml_fix_macro_enum_variant_as_struct(v)).collect();

        let structs = variants.iter().flat_map(|v| v.structs());

        impls.push(impl_toml_fix_macro_enum_into(ident, &variants));
        impls.push(impl_toml_fix_macro_enum_try_from(ident, &variants));

        quote! {
            #(#structs)*

            #(#impls)*

            impl Serialize for #ident where Self: EnumTomlFixed {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error> where
                    S: Serializer {
                    let ev: EnumVariant = self.try_into().map_err(|err| S::Error::custom("Some error occurred while deserializing Enum #ident"))?;
                    ev.serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for #ident where Self: EnumTomlFixed {
               fn deserialize<D>(deserializer: D) -> Result<Self,D::Error> where
                    D: Deserializer<'de> {
                    let ev = EnumVariant::deserialize(deserializer)?;
                    Self::try_from(ev).map_err(|err| D::Error::custom("Some error occurred while deserializing Enum #ident"))
                }
            }

        }
    }

    fn impl_toml_fix_macro_enum_into(ident: &Ident, variants: &Vec<Enum>) -> TokenStream {
        let matches: Vec<_> = variants.iter().map(|v| v.impl_toml_fix_macro_enum_into_match(ident)).collect();

        //println!("Generated {} Matches for Enum with {} Variants", matches.len(), variants.len());

        quote! {
            impl TryInto<EnumVariant> for &#ident {
                type Error = toml::ser::Error;
                fn try_into(self) -> Result<EnumVariant ,<Self as TryInto<EnumVariant>>::Error>{
                    match self {
                        #(#matches),*
                        //_ => panic!("Unhandled Enum Variant")
                    }
                }
            }

            impl TryInto<EnumVariant> for #ident {

                type Error = <&'static Self as TryInto<EnumVariant>>::Error;

                fn try_into(self) -> Result<EnumVariant ,<Self as TryInto<EnumVariant>>::Error>{
                    (&self).try_into()
                }
            }
        }
    }

    fn impl_toml_fix_macro_enum_try_from(ident: &Ident, variants: &Vec<Enum>) -> TokenStream {
        let matches = variants.iter().map(|v| v.impl_toml_fix_macro_enum_from_match(ident));
        quote! {
            impl TryFrom<EnumVariant> for #ident  {
                type Error = toml::de::Error;
                fn try_from(variant:EnumVariant) -> Result<Self,<Self as TryFrom<EnumVariant>>::Error> {
                    match &variant {
                        #(#matches),*
                        _ => Err(<toml::de::Error as serde::de::Error>::custom("Unknown Variant"))
                    }
                }
            }
        }
    }


    enum Enum<'v> {
        Unit(&'v Variant),
        Tuple(&'v Variant, TokenStream, Ident, Vec<Ident>),
        Struct(&'v Variant, TokenStream, Ident),
    }

    impl<'v> Enum<'v> {
        fn structs(&self) -> Option<&TokenStream> {
            match self {
                Enum::Unit(_) => None,
                Enum::Tuple(_, tokens, _, _) => Some(tokens),
                Enum::Struct(_, tokens, _) => Some(tokens),
            }
        }

        fn has_clone_attribute(field:&Field) -> bool {
            field.attrs.iter().any(|attr| attr.path.segments.first().map(|pair| pair.into_value().ident.to_string() == "clone").unwrap_or(false))
        }

        fn impl_toml_fix_macro_enum_into_match(&self, enum_ident: &Ident) -> TokenStream {
            match self {
                Enum::Unit(variant) => {
                    let ident = &variant.ident;
                    let str = format!("{}", ident);
                    quote! {
                        #enum_ident::#ident => {
                            Ok(EnumVariant{variant:String::from(#str),content:String::new()})
                        }
                    }
                }
                Enum::Tuple(variant, _, struct_ident, field_names) => {
                    let fields_struct: Vec<TokenStream> = variant.fields.iter().zip(field_names.iter()).map(|(field,name)| {
                        if Self::has_clone_attribute(field) {
                            quote!(#name: #name.clone())
                        } else {
                            quote!(#name: *#name)
                        }
                    }).collect();

                    let ident = &variant.ident;
                    let str = format!("{}", ident);

                    quote! {
                        #enum_ident::#ident(#(#field_names),*) => {
                            let content = #struct_ident{#(#fields_struct),*};

                            let mut out = String::new();
                            let mut serializer = toml::ser::Serializer::new(&mut out);
                            content.serialize(&mut serializer)?;
                            Ok(EnumVariant{variant:String::from(#str),content:out})
                        }
                    }
                }
                Enum::Struct(variant, _, struct_ident) => {
                    let fields_struct: Vec<TokenStream> = variant.fields.iter().map(|field| {
                        let ident = &field.ident;
                        if Self::has_clone_attribute(field) {
                            quote!(#ident: #ident.clone())
                        } else {
                            quote!(#ident: *#ident)
                        }
                    }).collect();

                    let fields: Vec<Ident> = variant.fields.iter().map(|field| field.ident.clone().unwrap()).collect();

                    let ident = &variant.ident;
                    let str = format!("{}", ident);

                    quote! {
                        #enum_ident::#ident{ #(#fields),* } => {
                            let content = #struct_ident{#(#fields_struct),*};
                            let mut out = String::new();
                            let mut serializer = toml::ser::Serializer::new(&mut out);
                            content.serialize(&mut serializer)?;
                            Ok(EnumVariant{variant:String::from(#str),content:out})
                        }
                    }
                }
            }
        }


        fn impl_toml_fix_macro_enum_from_match(&self, enum_ident:&Ident) -> TokenStream {
            let (match_string, match_content) = match self {
                Enum::Unit(variant) => {
                    let ident = &variant.ident;
                    let content = quote! {
                        Ok(#enum_ident::#ident)
                    };
                    (format!("{}", ident), content)
                }
                Enum::Tuple(variant, _, struct_ident, field_names) => {
                    let ident = &variant.ident;

                    let content = quote! {
                            let mut des = toml::de::Deserializer::new(content);
                            let #struct_ident{#(#field_names),*} = #struct_ident::deserialize(&mut des).map_err(|_| toml::de::Error::custom("Failed to deserialize #enum_ident::#ident"))?;
                            Ok(#enum_ident::#ident(#(#field_names),*))
                    };
                    (format!("{}", ident), content)
                }
                Enum::Struct(variant, _, struct_ident) => {
                    let fields: Vec<Ident> = variant.fields.iter().map(|field| field.ident.clone().unwrap()).collect();

                    let it_fields = fields.iter();
                    let ident = &variant.ident;
                    let content = quote! {
                            let mut des = toml::de::Deserializer::new(content);
                            let #struct_ident{#(#it_fields),*} = #struct_ident::deserialize(&mut des).map_err(|_| toml::de::Error::custom("Failed to deserialize #enum_ident::#ident"))?;
                            Ok(#enum_ident::#ident{#(#fields),*})
                    };
                    (format!("{}", ident), content)
                }
            };
            quote! {
                EnumVariant{variant,content} if variant == #match_string  => {
                    #match_content
                }
            }
        }
    }


    fn impl_toml_fix_macro_enum_variant_as_struct(variant: &Variant) -> Enum {
        let span = variant.span();
        let ident = syn::Ident::new(&format!("Variant{}Struct", &variant.ident), variant.ident.span());

        match &variant.fields {
            Fields::Unit => return Enum::Unit(variant),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let (field_names, field_types): (Vec<_>, Vec<_>) = unnamed.iter().zip(0..).map(|(a, b)| (Ident::new(&format!("index_{}", b), variant.ident.span()), &a.ty)).unzip();

                let it_field_names = field_names.iter();

                let quoted = quote! {
                    #[derive(Serialize,Deserialize)]
                    struct #ident {
                        #(#it_field_names:#field_types),*
                    }
                };

                Enum::Tuple(variant, quoted, ident, field_names)
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let (field_names, field_types): (Vec<_>, Vec<_>) = named.iter().map(|field| {
                    let name = field.ident.clone().unwrap();
                    let tipe = &field.ty;
                    (name, tipe)
                }).unzip();

                let quoted = quote_spanned! {span=>
                    #[derive(Serialize,Deserialize)]
                    struct #ident {
                        #(#field_names:#field_types),*
                    }
                };

                Enum::Struct(variant, quoted, ident)
            }
        }
    }
}
