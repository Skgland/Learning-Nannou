use quote::{quote, quote_spanned};
use proc_macro2::{TokenStream, Span};
use syn::{DeriveInput, Data::*, Ident, Variant, Fields, FieldsNamed, FieldsUnnamed, DataEnum};

pub fn impl_enumerable_trait_derive(ast: &DeriveInput) -> proc_macro::TokenStream{

    let ident = &ast.ident;
    let span = ident.span();

    let next_impl = impl_next_for_ast(ast);

    let result = quote_spanned!{span=>

        impl Enumerable for #ident {

            fn next(&self) -> Option<Self>{
                #next_impl
            }

        }

    };
    result.into()
}

fn impl_next_for_ast(ast:&DeriveInput)-> TokenStream{
    match &ast.data {
        Enum(data) => {
            impl_next_for_enum(ast,data)
        }
        Struct(_) => {
            unimplemented!()
        }
        Union(_) => {
            unimplemented!()
        }
    }
}

fn impl_next_for_enum(ast:&DeriveInput,enum_data:&DataEnum) -> TokenStream{
    let mut variant_iterator = enum_data.variants.iter()
        .filter(|var| !has_skip_attribute(var)).peekable();

    let mut matches = vec![];

    while let Some(current_variant) = variant_iterator.next(){
        let next_variant = variant_iterator.peek().copied();

        let match_block = impl_next_for_enum_variant(&ast.ident,current_variant,next_variant);
        matches.push(match_block);
    }

    quote!{
        match self {
            #(#matches),*
        }
    }
}

fn impl_next_for_enum_variant(enum_ident:&Ident,current_variant:&Variant,next_variant: Option<&Variant>) -> TokenStream {
    let ident = &current_variant.ident;
    let next_min = if let Some(next_variant) = next_variant {
        let min = super::bounded::impl_enum::get_min_for_enum_variant(enum_ident,next_variant);
        quote!{Some(#min)}
    }else{
        quote!{None}
    };
    match &current_variant.fields {
        Fields::Unit => {
            quote!{
                #enum_ident::#ident => {
                    #next_min
                }
            }
        },
        Fields::Named(FieldsNamed{named,..}) => {
            let field_names = named.iter().map(|field| field.ident.as_ref().unwrap()).collect();
            let result = iter_fields_named(enum_ident,ident,&field_names,next_min);
            quote!{
                #enum_ident::#ident{#(#field_names),*} => {
                    #result
                }
            }
        }
        Fields::Unnamed(FieldsUnnamed{unnamed,..}) => {
            let field_names_pre: Vec<Ident> = (0..unnamed.iter().len())
                .map(|index| Ident::new(&format!("index{}",index),Span::call_site()) ).collect();

            let result = iter_fields_unnamed(enum_ident, ident,&field_names_pre.iter().collect() , next_min);
            quote! {
                #enum_ident::#ident(#(#field_names_pre),*) => {
                    #result
                }
            }
        }
    }
}

fn iter_fields_unnamed(enum_ident: &Ident, variant_ident: &Ident, fields: &Vec<&Ident>, last: TokenStream) -> TokenStream{
    let mut prev = last;

    let mut iter_copy = fields.iter().rev();

    while let Some(&current_field) = iter_copy.next() {
        let reset_value = fields.iter()
                                .take_while(|&&a| format!("{}", a) != format!("{}", current_field));
        let reset_value2 = reset_value.clone();
        let keep_value = fields.iter()
                               .skip_while(|a| format!("{}", a) != format!("{}", current_field)).skip(1);
        let keep_value2 = keep_value.clone();

        prev = quote! {
            if let Some(value) = #current_field.next() {
                Some(#enum_ident::#variant_ident(#(#keep_value:Clone::clone(#keep_value2),)* #current_field:value, #(#reset_value:Enumerable::reset(#reset_value2)),*))
            }else{
                #prev
            }
        }
    }

    prev
}

fn iter_fields_named(enum_ident: &Ident,variant_ident: &Ident,fields: &Vec<&Ident>,last:TokenStream) -> TokenStream{
    let mut prev = last;

    let mut iter_copy = fields.iter().rev();

    while let Some(&current_field) = iter_copy.next(){

        let reset_value = fields.iter()
            .take_while(|&&a| format!("{}",a)!=format!("{}",current_field));
        let reset_value2 = reset_value.clone();
        let keep_value = fields.iter()
            .skip_while(|a| format!("{}", a) != format!("{}", current_field)).skip(1);
        let keep_value2 = keep_value.clone();

        prev = quote!{
            if let Some(value) = Enumerable::next(#current_field) {
                Some(#enum_ident::#variant_ident{#(#keep_value:Clone::clone(#keep_value2),)* #current_field:value, #(#reset_value:Enumerable::reset(#reset_value2)),*})
            }else{
                #prev
            }
        }
    }

    prev
}

fn has_skip_attribute(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| attr.path.segments.first().map(|pair| pair.into_value().ident.to_string() == "skip").unwrap_or(false))
}
