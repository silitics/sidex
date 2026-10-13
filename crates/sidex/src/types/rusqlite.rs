//! Storing strings in SQLite databases with [`rusqlite`](https://docs.rs/rusqlite).
//!
//! Strings are stored like `String`s.

use rusqlite::types::FromSql;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSql;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;

use super::Str;

impl ToSql for Str {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(self.as_bytes())))
    }
}

impl FromSql for Str {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value.as_str().map(Str::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Strings round-trip through SQLite.
    #[test]
    fn round_trip() {
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        let text: Str = connection
            .query_row("SELECT ?1", [Str::from("hello")], |row| row.get(0))
            .unwrap();
        assert_eq!(text, "hello");
    }
}
