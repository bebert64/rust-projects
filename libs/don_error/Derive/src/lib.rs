use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, DeriveInput, Meta},
};

#[proc_macro_derive(ResponseError, attributes(status_code))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let data = match data {
        syn::Data::Enum(data) => data,
        _ => panic!("ResponseError can only be derived for enums"),
    };

    let variants = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = variant.ident.clone();
            let status_code: proc_macro2::TokenStream = match variant
                .attrs
                .iter()
                .filter_map(|attr| match &attr.meta {
                    Meta::NameValue(name_value) if name_value.path.is_ident("status_code") => {
                        Some(name_value.value.clone())
                    }
                    _ => None,
                })
                .next()
                .expect("attr status_code must be provided")
            {
                syn::Expr::Lit(expr) => match expr.lit {
                    syn::Lit::Str(lit_str) => lit_str.value().parse().unwrap(),
                    _ => panic!("status_code must be a Str literal"),
                },
                _ => panic!("status_code must be a literal"),
            };
            quote! {
                #ident::#variant_ident => ::actix_web::http::StatusCode::#status_code,
            }
        })
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        impl ::actix_web::ResponseError for #ident {
            fn status_code(&self) -> ::actix_web::http::StatusCode {
                match self {
                    #(#variants)*
                }
            }
        }
    })
    // output.into()
}
