//! Derive macro for ratatui_router
pub(crate) mod helper;

use crate::helper::{get_attributes, iterate_fields};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, DeriveInput, Error, Ident, parse_macro_input, spanned::Spanned};

/// Derive macro that implements `Routed` from `ratatui_router`
/// for your route enum, automatically generating a router-compatible implementation.
///
/// This macro is the core of [`ratatui_router`]
#[proc_macro_derive(Routes, attributes(event))]
pub fn create_routes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !matches!(input.data, syn::Data::Enum(_)) {
        return syn::Error::new(input.ident.span(), "Only enums supported as path")
            .to_compile_error()
            .into();
    }
    let data = match input.data {
        syn::Data::Enum(x) => x,
        _ => {
            return Error::new(input.ident.span(), "Not an enum type")
                .to_compile_error()
                .into();
        }
    };

    let routed = match impl_routed(data.clone(), &input.ident) {
        Err(err) => err.to_compile_error().into(),
        Ok(ok) => ok,
    };

    let create_router = match impl_routed_events(data.clone(), &input.ident) {
        Err(err) => err.to_compile_error().into(),
        Ok(ok) => ok,
    };
    quote! {
        #routed
        #create_router
    }
    .into()
}

fn impl_routed_events(
    data: syn::DataEnum,
    enum_ident: &proc_macro2::Ident,
) -> Result<TokenStream, Error> {
    let parsed_attrs = get_attributes(data.clone())?;
    let mut enum_arms = Vec::<TokenStream>::new();
    for (i, iter) in data.variants.iter().enumerate() {
        if let Some(attr) = parsed_attrs.get(&i) {
            let route_ident = iter.ident.clone();
            enum_arms.push(quote! {
                #route_ident(#attr)
            });
        }
    }
    let enum_name = format_ident!("{}{}", enum_ident, "Event");
    Ok(quote! {
        pub enum #enum_name {
            #(#enum_arms),*
        }
        impl From<#enum_name> for Events<#enum_name> {
            fn from(value: #enum_name) -> Self {
                Events::Custom(value)
            }
        }
    })
}

fn impl_routed(data: syn::DataEnum, enum_ident: &proc_macro2::Ident) -> Result<TokenStream, Error> {
    let arms_for_render = data.variants.iter().map(|iter| {
        let name = iter.ident.clone();
        match &iter.fields {
            syn::Fields::Unnamed(fields) => {
                let params_len = iter.fields.len();
                if params_len == 0 {
                    quote! {
                        #enum_ident::#name() => {#name(ctx, frame)}
                    }
                } else {
                    let fields = iterate_fields(&fields.unnamed);
                    let cloned_fields = fields.clone();
                    quote! {
                        #enum_ident::#name(#(#fields),*) => {#name(ctx, frame, #(#cloned_fields),*)}
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #enum_ident::#name => {#name(ctx, frame)}
                }
            }
            syn::Fields::Named(fields) => {
                let fields = iterate_fields(&fields.named);
                let cloned_fields = fields.clone();
                quote! {
                    #enum_ident::#name{#(#fields),*} => {#name(ctx, frame, #(#cloned_fields),*)}
                }
            }
        }
    });
    let enum_name = format_ident!("{}{}", enum_ident, "Event");
    Ok(quote! {
        impl Routed for #enum_ident {
            type Ev = #enum_name;
            fn render(&mut self, ctx: &mut Router<#enum_ident>, frame: &mut ::ratatui::Frame) -> () {
                match self {
                    #(#arms_for_render),*
                }
            }
        }
    })
}
