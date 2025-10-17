use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, Attribute, DeriveInput, Error, Field, Ident, Token};
use std::collections::HashMap;

pub fn iterate_fields(fields: &Punctuated<Field, Token![,]>) -> impl Iterator<Item = proc_macro2::TokenStream> + Clone {
    let iter = fields.iter().enumerate().map(|(i, iter)| {
        if let Some(ident) = iter.clone().ident {
            quote! { #ident }
        } else {
            let ident = format_ident!("field_{}", i);
            quote! { #ident }
        }
    });
    iter.clone()
}

pub fn get_attributes(data: syn::DataEnum) -> Result<HashMap<usize, Ident>, Error> {
    let attribute_name = "event";
    let mut items = HashMap::<usize, Ident>::new();
    for (i, iter) in data.variants.iter().enumerate() {
        let attr = iter
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident(attribute_name));
        if let Some(attr) = attr {

            let list = attr
                .meta
                .require_list()?;
            list.parse_nested_meta(|meta| {
                let ident = meta.path.require_ident()?;
                if items.contains_key(&i) {
                    return Err(syn::Error::new(ident.span(), "Event macro only accepts one type"));
                }
                items.insert(i, ident.clone());
                Ok(())
            })?;
            if !items.contains_key(&i) {
                return Err(syn::Error::new(list.span(), "No type was passed"));
            }
        };
    }
    Ok(items)
}