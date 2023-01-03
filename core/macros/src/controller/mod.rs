use proc_macro::TokenStream;
use quote::quote;
use syn::Item;

pub(crate) fn generate(input: Item) -> syn::Result<TokenStream> {
    match input {
        Item::Fn(item_fn) => {
            let attrs = &item_fn.attrs;
            let vis = &item_fn.vis;

            let sig = &item_fn.sig;
            let body = &item_fn.block;
            let name = &sig.ident;

            let struct_fn = quote! {
                #vis struct #name;

                impl #name {
                    #[allow(non_camel_case_types)]
                    #[derive(Debug)]
                    #(#attrs)*
                    #sig {
                        #body
                    }
                }
            };

            println!("struct_fn: {}", &struct_fn);

            let handler_struct_fn = quote! {
                #struct_fn;

                impl Controller for #name{
                    fn #name
                }
            };

            println!("{}", &handler_struct_fn);

            let f_n = quote! {
                fn #name (request: pillow::http::request::Request,response: pillow::http::response::Response) -> String
                    #body

            };

            println!("f_n: {}", &f_n);

            Ok(f_n.into())
        }
        _ => Err(syn::Error::new_spanned(
            input,
            "#[controller] must added to `fn`",
        )),
    }
}
