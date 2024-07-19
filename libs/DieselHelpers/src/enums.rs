#[macro_export]
macro_rules! bi_dir_from{
    ($name:ident, $db_name:ident, $($variant:ident <=> $variant_name:expr),*) => {
        mod bi_dir_from_inner {
            use {
                diesel::{
                    deserialize::{self, FromSql},
                    pg::{Pg, PgValue},
                    serialize::{self, IsNull, Output, ToSql},
                    sql_types::Text,
                },
                std::io::Write,
                super::{$name, $db_name},
            };

            impl ToSql<$db_name, Pg> for $name {
                fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
                    match *self {
                        $(Self::$variant => out.write_all($variant_name.as_bytes())?),*
                    }
                    Ok(IsNull::No)
                }
            }

            impl FromSql<$db_name, Pg> for $name {
                fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
                    match <String as FromSql<Text, Pg>>::from_sql(bytes)?.as_str() {
                        $($variant_name => Ok(Self::$variant),)*
                        _ => Err("Unrecognized enum variant".into()),
                    }
                }
            }
        }
}}
