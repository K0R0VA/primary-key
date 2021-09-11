use proc_macro::TokenStream;

extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrimaryKey)]
pub fn primary_key_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    TokenStream::from(quote! {
        impl tokio_postgres::types::ToSql for #name {
            fn to_sql(&self, ty: &tokio_postgres::types::Type, out: &mut actix_web::web::BytesMut) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            where
                Self: Sized {
                <i32 as tokio_postgres::types::ToSql>::to_sql(&self.0, ty, out)
            }

            fn accepts(ty: &tokio_postgres::types::Type) -> bool
            where
                Self: Sized {
                <i32 as tokio_postgres::types::ToSql>::accepts(ty)
            }

            fn to_sql_checked(
                &self,
                ty: &tokio_postgres::types::Type,
                out: &mut actix_web::web::BytesMut,
            ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                <i32 as tokio_postgres::types::ToSql>::to_sql_checked(&self.0, ty, out)
            }
        }
    })
}
