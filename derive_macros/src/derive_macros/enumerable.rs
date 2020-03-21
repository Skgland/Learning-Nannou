use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Data::*, DataEnum, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Variant};

pub fn impl_enumerable_trait_derive(ast: &DeriveInput) -> proc_macro::TokenStream {
    let ident = &ast.ident;
    let span = ident.span();

    let next_impl = impl_next_for_ast(ast);

    let result = quote_spanned! {span=>

        impl Enumerable for #ident {

            fn next(&self) -> Option<Self>{
                #next_impl
            }

        }

    };
    result.into()
}

fn impl_next_for_ast(ast: &DeriveInput) -> TokenStream {
    match &ast.data {
        Enum(data) => impl_next_for_enum(data),
        Struct(_) => unimplemented!("Enumerable is currently not derivable for Struct Types"),
        Union(_) => panic!("Enumerable is not derivable for Union Types"),
    }
}

fn impl_next_for_enum(enum_data: &DataEnum) -> TokenStream {
    let variant_iterator: Vec<_> = enum_data
        .variants
        .iter()
        .filter(|var| !has_skip_attribute(var))
        .collect();

    let mut matches = vec![];

    let mut current = variant_iterator.as_slice();

    while let [current_variant, remaining_variants @ ..] = current {
        let match_block = impl_next_for_enum_variant(current_variant, remaining_variants);
        matches.push(match_block);

        current = remaining_variants;
    }

    let result = quote! {
        match self {
            #(#matches),*
        }
    };

    result
}

fn impl_next_for_enum_variant(current_variant: &Variant, next_variant: &[&Variant]) -> TokenStream {
    let ident = &current_variant.ident;

    //minimum value for next variant if a next variant exists
    let next_min = if let [next_variant, ..] = next_variant {
        let min = super::bounded::impl_enum::get_min_for_enum_variant(next_variant);
        quote! {Some(#min)}
    } else {
        quote! {None}
    };

    match &current_variant.fields {
        Fields::Unit => {
            //For Unit Variant directly return next Variants first value
            quote! {
                Self::#ident => {
                    #next_min
                }
            }
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let field_names: Vec<_> = named
                .iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect();
            let result = iter_fields_named(ident, &field_names, next_min);
            quote! {
                //Deconstruct all Fields and increment starting at the first field carrying and rolling over to the next variant if necessary
                Self::#ident{#(#field_names),*} => {
                    #result
                }
            }
        }
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let field_names_pre: Vec<Ident> = (0..unnamed.iter().len())
                .map(|index| Ident::new(&format!("index{}", index), Span::call_site()))
                .collect();

            let field_names_pre: Vec<_> = field_names_pre.iter().collect();

            let result = iter_fields_unnamed(ident, &field_names_pre, next_min);
            quote! {
                //Deconstruct all Fields and increment starting at the first field carrying and rolling over to the next variant if necessary
                Self::#ident(#(#field_names_pre),*) => {
                    #result
                }
            }
        }
    }
}

fn iter_fields_unnamed(variant_ident: &Ident, fields: &[&Ident], last: TokenStream) -> TokenStream {
    let mut prev = last;

    for i in 0..fields.len() {
        if let (prior, [current, behind @ ..]) = fields.split_at(i) {
            //Elements that keep their current value
            let keep = prior.iter();

            //Elements that get reset to their minimum
            let reset = behind.iter().map(|_| quote! {Bounded::minimum()});

            prev = quote! {
                if let Some(value) = Enumerable::next(#current) {
                    Some(Self::#variant_ident(#(Clone::clone(#keep),)* value, #(#reset),*))
                }else{
                    #prev
                }
            }
        } else {
            // i < fields.len() is true for all iterations
            // as such the right halve of the always contains at least one value
            unreachable!()
        }
    }

    prev
}

fn iter_fields_named(variant_ident: &Ident, fields: &[&Ident], last: TokenStream) -> TokenStream {
    let mut prev = last;

    for i in 0..fields.len() {
        if let (prior, [current, behind @ ..]) = fields.split_at(i) {
            //Elements that keep their current value
            let keep = prior.iter();
            let keep2 = keep.clone();

            //Elements that get reset to their minimum
            let reset = behind.iter();

            prev = quote! {
                if let Some(value) = Enumerable::next(#current) {
                    Some(Self::#variant_ident{#(#keep:Clone::clone(#keep2),)* #current:value, #(#reset:Bounded::minimum()),*})
                }else{
                    #prev
                }
            }
        } else {
            // i < fields.len() is true for all iterations
            // as such the right halve of the always contains at least one value
            unreachable!()
        }
    }

    prev
}

fn has_skip_attribute(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| {
        attr.path
            .segments
            .first()
            .map(|pair| pair.ident == "skip")
            .unwrap_or(false)
    })
}
