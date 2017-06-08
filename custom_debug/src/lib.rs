
// `quote` relies on macro recursion, so it is likely to hit the normal cap.
#![recursion_limit = "256"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use quote::Tokens;

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive_debug(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();

    match expand_debug(&input) {
        Ok(tokens) => tokens.parse().unwrap(),
        Err(err) => panic!("Error: {}", err),
    }
}

fn expand_debug(input: &syn::DeriveInput) -> Result<Tokens, String> {
    let name = &input.ident;

    let expanded = quote! {
        impl ::std::fmt::Debug for #name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, stringify!(#name))
            }
        }
    };

    Ok(expanded)
}
