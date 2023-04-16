use crate::database::base::Column;
use crate::database::base::Column::{INumber, UNumber};
use crate::database::mysql::base::ColumnSchema;

pub fn parse(column_schema: &ColumnSchema, value: String) -> Column {
    match column_schema.data_type.as_str() {
        "tinyint" | "smallint" | "mediumint" | "int" | "bigint" => {
            if column_schema.column_type.contains("unsigned") {
                UNumber(value.parse().unwrap())
            } else {
                INumber(value.parse().unwrap())
            }
        }
        _ => unreachable!("parse error"),
    }
}
