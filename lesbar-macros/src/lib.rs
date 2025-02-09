#![no_std]

extern crate proc_macro;

use lesbar_text::StrExt as _;
use proc_macro::TokenStream;
use syn::LitStr;

#[proc_macro]
pub fn pstr(input: TokenStream) -> TokenStream {
    let literal = syn::parse_macro_input!(input as LitStr);
    if literal.value().has_printable_text() {
        quote::quote! {
            // SAFETY: The procedural macro that generated this code has established that the
            //         string literal is non-empty. See `lesbar_text::StrExt::has_printable_text`.
            ::lesbar::pstr::PStr::from_str1_unchecked(unsafe {
                ::mitsein::str1::Str1::from_str_unchecked(#literal)
            })
        }
    }
    else {
        quote::quote! {
            ::core::compile_error!("string literal is not printable")
        }
    }
    .into()
}

#[proc_macro]
pub fn str1(input: TokenStream) -> TokenStream {
    let literal = syn::parse_macro_input!(input as LitStr);
    // TODO: Share the predicate code with `mitsein` instead of reimplementing it here.
    if literal.value().chars().take(1).count() != 0 {
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
