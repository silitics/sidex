//! Storing strings in SQL databases with [`sqlx`](https://docs.rs/sqlx).
//!
//! Strings are stored like `String`s.

use sqlx_core::database::Database;
use sqlx_core::decode::Decode;
use sqlx_core::encode::Encode;
use sqlx_core::encode::IsNull;
use sqlx_core::error::BoxDynError;
use sqlx_core::types::Type;

use super::Str;

impl<DB: Database> Type<DB> for Str
where
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}

impl<'q, DB: Database> Encode<'q, DB> for Str
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        <String as Encode<'q, DB>>::encode(self.as_str().to_owned(), buf)
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Str
where
    String: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        <String as Decode<'r, DB>>::decode(value).map(Str::from)
    }
}
