#![deny(missing_docs)]

//! This crate provides a procedural derive macro for constant `From` trait implementations for
//! enumerations based on their `repr` type.
//!
//! Due to offering const support, this library requires the usage of Rust nightly.
//! Additionally, you must add the following feature flags to your crate root:
//!
//! ```rust
//! #![feature(const_trait_impl)]   // always required
//! ```
//!
//! This is required as some features are currently gated behind these flags.
//! Further documentation about usage can be found in the individual macro.

extern crate proc_macro;

use proc_macro::TokenStream as NativeTokenStream;
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::quote;

struct EnumVariant {
    name: syn::Ident,
    value: syn::Expr,
}

/// This function defines the procedural derive macro `ConstEnum`.
/// It can be used on any enum with a `repr` type, e.g. `repr(u8)`.
///
/// When being used on an enum, this will automatically add the following implementations:
///
/// - `From<repr_type> for <enum_type>`: Convert repr type to enum, panic if invalid value
/// - `From<enum_type> for <repr_type>`: Convert enum to repr type
///
/// # Example
/// ```rust
/// #![feature(const_trait_impl)]
///
/// use const_enum::ConstEnum;
///
/// #[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
/// #[repr(u8)]
/// enum Test {
///     A = 0b010,
///     B = 0b100,
///     C = 0b001
/// }
///
/// pub fn example() {
///     println!("{:?}", Test::from(0b010 as u8));
///     println!("{:?}", u8::from(Test::A));
/// }
/// ```
#[proc_macro_derive(ConstEnum)]
pub fn const_enum(input: NativeTokenStream) -> NativeTokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let enum_name = input.ident;
    let enum_variants = get_enum_variants(&input.data);
    let enum_type = get_enum_repr_type(&input.attrs);
    let match_impl = build_from_match(&enum_name, &enum_variants);

    let expanded = quote! {
        impl core::convert::From<#enum_name> for #enum_type {
            fn from(value: #enum_name) -> Self {
                value as Self
            }
        }

        impl core::convert::From<#enum_type> for #enum_name {
            fn from(value: #enum_type) -> Self {
                #match_impl
            }
        }
    };

    NativeTokenStream::from(expanded)
}

fn get_enum_repr_type(attrs: &Vec<syn::Attribute>) -> syn::Ident {
    let repr = syn::Ident::new("repr", Span::call_site());
    let repr_attr = attrs.iter().find(|attr| match attr.style {
        syn::AttrStyle::Outer => attr.path.is_ident(&repr),
        _ => false,
    }).unwrap_or_else(|| panic!("repr attribute not found on enum"));

    let repr_tokens = repr_attr.tokens.clone();
    let mut repr_tokens_iter = repr_tokens.into_iter();

    let first_token = repr_tokens_iter.next();
    if first_token.is_none() || repr_tokens_iter.next().is_some() {
        panic!("malformed repr attribute, expected repr(TYPE)");
    }

    match first_token.unwrap().clone() {
        TokenTree::Group(repr_items) => {
            if repr_items.delimiter() != Delimiter::Parenthesis {
                panic!("malformed repr attribute, expected repr(TYPE)");
            }

            let mut repr_types_iter = repr_items.stream().into_iter();
            let first_repr_item = repr_types_iter.next().unwrap();

            if let Some(_) = repr_types_iter.next() {
                panic!("malformed repr attribute, expected single type");
            }

            match first_repr_item.clone() {
                TokenTree::Ident(repr_type) => repr_type,
                _ => panic!("malformed repr attribute, unexpected type"),
            }
        },
        _ => panic!("malformed repr attribute, unexpected token"),
    }
}

fn get_enum_variants(data: &syn::Data) -> Vec<EnumVariant> {
    match *data {
        syn::Data::Enum(ref data) => {
            data.variants.iter().map(|variant| {
                let pair = variant.discriminant.as_ref().unwrap();
                let name = variant.ident.clone();
                let value = pair.1.clone();

                EnumVariant { name, value }
            }).collect()
        }
        syn::Data::Struct(_) => panic!("unexpected struct, const-enum only supports enums"),
        syn::Data::Union(_) => panic!("unexpected union, const-enum only supports enums"),
    }
}

fn build_from_match(enum_name: &syn::Ident, variants: &Vec<EnumVariant>) -> TokenStream {
    let mut match_arms = TokenStream::new();

    // Generate a match arm for each variant
    variants.iter().for_each(|variant| {
        let (name, value) = (&variant.name, &variant.value);

        match_arms.extend(quote! {
            #value => #enum_name::#name,
        });
    });

    // Add exhaustive default match arm resulting in error
    match_arms.extend(quote! {
        any => panic!("invalid value provided:{:?}",any),
    });

    return quote! {
        match value {
            #match_arms
        }
    };
}
