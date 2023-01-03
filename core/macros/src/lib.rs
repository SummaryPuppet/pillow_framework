use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

mod controller;
mod shared;

#[proc_macro_attribute]
pub fn controller(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    match controller::generate(input) {
        Ok(i) => i,
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse2;

    #[test]
    fn test_controller() {
        let input = quote! {
            #[controller]
            fn ctrl() {
                res.text("hello");
            }
        };

        assert_eq!(
            input.to_string(),
            quote! {
                fn ctrl (&self, _request: Request, response: Response) -> String {
                    res.text("hello")
                }
            }
            .to_string()
        );
    }
}
