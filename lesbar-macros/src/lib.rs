#![no_std]

extern crate proc_macro;

use lesbar_text::StrExt as _;
use mitsein::str1::Str1;
use proc_macro::TokenStream;
use syn::LitStr;

#[proc_macro]
pub fn tstr(input: TokenStream) -> TokenStream {
    let literal = syn::parse_macro_input!(input as LitStr);
    if literal.value().has_text() {
        quote::quote! {
            // SAFETY: The procedural macro that generated this code has established that the
            //         string literal is non-empty. See `lesbar_text::StrExt::has_text`.
            ::lesbar::tstr::TStr::from_str1_unchecked(unsafe {
                ::mitsein::str1::Str1::from_str_unchecked(#literal)
            })
        }
    }
    else {
        quote::quote! {
            ::core::compile_error!("string literal has no legible text")
        }
    }
    .into()
}

#[proc_macro]
pub fn str1(input: TokenStream) -> TokenStream {
    let literal = syn::parse_macro_input!(input as LitStr);
    if Str1::try_from_str(literal.value().as_ref()).is_ok() {
        quote::quote! {
            // SAFETY: The procedural macro that generated this code has established that the
            //         string literal is non-empty.
            unsafe {
                ::mitsein::str1::Str1::from_str_unchecked(#literal)
            }
        }
    }
    else {
        quote::quote! {
            ::core::compile_error!("string literal is empty")
        }
    }
    .into()
}
