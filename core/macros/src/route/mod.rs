use proc_macro::TokenStream;
use quote::quote;
use syn::ExprStruct;

pub(crate) fn generate(input: ExprStruct) -> TokenStream {
    let _args = &input.attrs;

    let name = &input.path;
    // let name = &input.ident;

    quote! {
        {
        let route_temp = pillow::http::Route::new(#name::path(), #name::method(),|request| #name::handler(request));
        route_temp
        }
    }
    .into()
}
