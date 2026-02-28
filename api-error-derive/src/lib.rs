use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ApiError, attributes(response))]
pub fn derive_api_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let Data::Enum(data_enum) = input.data else {
        return quote! {
            compile_error!("ApiError can only be derived for enums");
        }
        .into();
    };

    let mut response_arms = Vec::new();

    for variant in data_enum.variants {
        let variant_ident = variant.ident;
        let mut response_ident = None;

        for attr in &variant.attrs {
            if !attr.path().is_ident("response") {
                continue;
            }

            if response_ident.is_some() {
                let msg = format!(
                    "duplicate #[response(...)] on variant '{}::{}'",
                    enum_ident, variant_ident
                );
                return quote! { compile_error!(#msg); }.into();
            }

            match attr.parse_args::<syn::Ident>() {
                Ok(ident) => response_ident = Some(ident),
                Err(_) => {
                    let msg = format!(
                        "invalid #[response(...)] on variant '{}::{}', expected #[response(VariantName)]",
                        enum_ident, variant_ident
                    );
                    return quote! { compile_error!(#msg); }.into();
                }
            }
        }

        let Some(response_ident) = response_ident else {
            let msg = format!(
                "missing #[response(...)] on variant '{}::{}'",
                enum_ident, variant_ident
            );
            return quote! { compile_error!(#msg); }.into();
        };

        let pat = match variant.fields {
            Fields::Unit => quote! { Self::#variant_ident },
            Fields::Unnamed(_) => quote! { Self::#variant_ident(..) },
            Fields::Named(_) => quote! { Self::#variant_ident { .. } },
        };

        response_arms.push(quote! {
            #pat => api_error::ErrorResponse::#response_ident { error: err_string.clone() }
        });
    }

    quote! {
        impl core::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{self:?}")
            }
        }

        impl std::error::Error for #enum_ident {}

        impl axum::response::IntoResponse for #enum_ident {
            fn into_response(self) -> axum::response::Response {
                let err_string = self.to_string();
                tracing::error!(error = %err_string);

                let body = match self {
                    #(#response_arms,)*
                };

                let status = body.status_code();
                (status, axum::Json(body)).into_response()
            }
        }
    }
    .into()
}
