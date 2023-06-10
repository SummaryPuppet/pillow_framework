use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

pub(crate) fn generate(input: Expr) -> TokenStream {
    let s = match input {
        Expr::Struct(expr_struct) => expr_struct,
        _ => panic!("Expected ExprStruct"),
    };

    let name = &s.path;

    quote! {
        {
        let route_temp = pillow::http::Route::new(#name::path(), #name::method(),|request| #name::handler(request));
        route_temp
        }
    }
    .into()
}
