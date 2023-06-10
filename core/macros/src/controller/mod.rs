use proc_macro::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemFn};

pub(crate) fn generate(input: ItemFn, method: String, path: String) -> TokenStream {
    let vis = &input.vis;
    let _args = &input.attrs;

    let body = &input.block;

    let name = &input.sig.ident;

    let struct_fn = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        #vis struct #name;

        impl #name {
            pub fn method() -> pillow::http::HttpMethods {
                pillow::http::from_str_to_http_method(#method).unwrap()
            }

            pub fn path() -> String {
                #path.to_string()
            }
        }


        impl pillow::http::Handler for #name {
            fn handler(request: &pillow::http::Request) -> pillow::http::Response
            where
                Self: std::fmt::Debug + Sized + Send + Sync
            {
                #body
            }
        }

        impl From<#name> for pillow::http::Route {
            fn from (_: #name) -> Self {
                pillow::http::Route::new(#name::path(), #name::method(), |request| #name::handler(request))
            }
        }
    };

    struct_fn.into()
}

pub(crate) fn generate_attrs(vec_attrs: AttributeArgs) -> (String, String) {
    if vec_attrs.len() != 2 {
        panic!("This attribute need 2 attributes: http method and the route");
    }

    let method = match vec_attrs[0].clone() {
        // When is controller("GET", "/")
        syn::NestedMeta::Lit(lit) => {
            if let syn::Lit::Str(lit_str) = lit {
                lit_str.value().to_string()
            } else {
                panic!("Method is not correct")
            }
        }
        syn::NestedMeta::Meta(meta) => {
            // When is controller(method = "GET")
            if let syn::Meta::NameValue(meta_name) = meta {
                match meta_name.lit {
                    syn::Lit::Str(str) => str.value().to_string(),

                    _ => panic!("Method is not correct"),
                }
            } else {
                panic!("Method is not correct")
            }
        }
    };
    let path = match vec_attrs[1].clone() {
        // When is controller("GET, "/")
        syn::NestedMeta::Lit(lit) => {
            if let syn::Lit::Str(lit_str) = lit {
                lit_str.value().to_string()
            } else {
                panic!("Path is not correct");
            }
        }
        syn::NestedMeta::Meta(meta) => {
            // When is controller(path = "/")
            if let syn::Meta::NameValue(meta_name) = meta {
                match meta_name.lit {
                    syn::Lit::Str(str) => str.value().to_string(),

                    _ => panic!("Path is not correct"),
                }
            } else {
                panic!("Path is not correct")
            }
        }
    };

    (method, path)
}
