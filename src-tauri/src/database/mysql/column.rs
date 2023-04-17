use crate::database::base::Column;
use crate::database::base::Column::{
    BinaryString, BitNumber, DateString, JsonString, ParseError, SimpleNumber, SimpleString,
};
use crate::database::mysql::base::ColumnSchema;

pub fn parse(column_schema: &ColumnSchema, value: String) -> Column {
    match column_schema.data_type.as_str() {
        "tinyint" | "smallint" | "mediumint" | "int" | "bigint" => SimpleNumber(value),
        "decimal" | "float" | "double" => SimpleNumber(value),
        "bit" => BitNumber(value),
        "date" | "time" | "datetime" | "timestamp" | "year" => DateString(value),
        "char" | "varchar" => SimpleString(value),
        "binary" | "varbinary" => BinaryString("binary".to_string()),
        "tinyblob" | "mediumblob" | "blob" | "longblob" => BinaryString("binary".to_string()),
        "tinytext" | "mediumtext" | "text" | "longtext" => SimpleString(value),
        "enum" | "set" => SimpleString(value),
        "json" => JsonString(value),
        _ => ParseError(value),
    }
}

#[cfg(test)]
mod parse_tests {
    use crate::database::base::Column;
    use crate::database::base::Column::{BinaryString, BitNumber, DateString, JsonString, SimpleNumber, SimpleString};
    use crate::database::mysql::base::ColumnSchema;
    use crate::database::mysql::column::parse;

    fn sut(data_type: &str, column_type: &str, value: &str) -> Column {
        parse(
            &ColumnSchema {
                column_name: "col_test".to_string(),
                data_type: data_type.to_string(),
                column_type: column_type.to_string(),
            },
            value.to_string(),
        )
    }

    #[test]
    fn parse_i_tinyint() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("tinyint", "tinyint", "42"));
    }

    #[test]
    fn parse_u_tinyint() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("tinyint", "tinyint unsigned", "42"));
    }

    #[test]
    fn parse_i_smallint() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("smallint", "smallint", "42"));
    }

    #[test]
    fn parse_u_smallint() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("smallint", "smallint unsigned", "42"));
    }

    #[test]
    fn parse_i_mediumint() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("mediumint", "mediumint", "42"));
    }

    #[test]
    fn parse_u_mediumint() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("mediumint", "mediumint unsigned", "42"));
    }

    #[test]
    fn parse_i_int() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("int", "int", "42"));
    }

    #[test]
    fn parse_u_int() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("int", "int unsigned", "42"));
    }

    #[test]
    fn parse_i_bigint() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("bigint", "bigint", "42"));
    }

    #[test]
    fn parse_u_bigint() {
        let s = String::from("42");
        assert_eq!(SimpleNumber(s), sut("bigint", "bigint unsigned", "42"));
    }

    #[test]
    fn parse_decimal() {
        let s = String::from("42.0");
        assert_eq!(SimpleNumber(s), sut("decimal", "decimal(5,2)", "42.0"));
    }

    #[test]
    fn parse_float() {
        let s = String::from("42.0");
        assert_eq!(SimpleNumber(s), sut("float", "float(5,2)", "42.0"));
    }

    #[test]
    fn parse_double() {
        let s = String::from("42.0");
        assert_eq!(SimpleNumber(s), sut("double", "double(5,2)", "42.0"));
    }

    #[test]
    fn parse_bit() {
        let s = String::from("111");
        assert_eq!(BitNumber(s), sut("bit", "bit(3)", "111"));
    }

    #[test]
    fn parse_date() {
        let s = String::from("2020-01-01");
        assert_eq!(DateString(s), sut("date", "date", "2020-01-01"));
    }

    #[test]
    fn parse_time() {
        let s = String::from("12:34:56");
        assert_eq!(DateString(s), sut("time", "time", "12:34:56"));
    }

    #[test]
    fn parse_datetime() {
        let s = String::from("2020-01-01 12:34:56");
        assert_eq!(DateString(s), sut("datetime", "datetime", "2020-01-01 12:34:56"));
    }

    #[test]
    fn parse_timestamp() {
        let s = String::from("2020-01-01 12:34:56");
        assert_eq!(DateString(s), sut("timestamp", "timestamp", "2020-01-01 12:34:56"));
    }

    #[test]
    fn parse_year() {
        let s = String::from("2020");
        assert_eq!(DateString(s), sut("year", "year", "2020"));
    }

    #[test]
    fn parse_char() {
        let s = String::from("abc");
        assert_eq!(SimpleString(s), sut("char", "char(3)", "abc"));
    }

    #[test]
    fn parse_varchar() {
        let s = String::from("abc");
        assert_eq!(SimpleString(s), sut("varchar", "varchar(3)", "abc"));
    }

    #[test]
    fn parse_binary() {
        let s = String::from("binary");
        assert_eq!(BinaryString(s), sut("binary", "binary(3)", "abc"));
    }

    #[test]
    fn parse_varbinary() {
        let s = String::from("binary");
        assert_eq!(BinaryString(s), sut("varbinary", "varbinary(3)", "abc"));
    }

    #[test]
    fn parse_tinyblob() {
        let s = String::from("binary");
        assert_eq!(BinaryString(s), sut("tinyblob", "tinyblob", "abc"));
    }

    #[test]
    fn parse_blob() {
        let s = String::from("binary");
        assert_eq!(BinaryString(s), sut("blob", "blob", "abc"));
    }

    #[test]
    fn parse_mediumblob() {
        let s = String::from("binary");
        assert_eq!(BinaryString(s), sut("mediumblob", "mediumblob", "abc"));
    }

    #[test]
    fn parse_longblob() {
        let s = String::from("binary");
        assert_eq!(BinaryString(s), sut("longblob", "longblob", "abc"));
    }

    #[test]
    fn parse_tinytext() {
        let s = String::from("abc");
        assert_eq!(SimpleString(s), sut("tinytext", "tinytext", "abc"));
    }

    #[test]
    fn parse_text() {
        let s = String::from("abc");
        assert_eq!(SimpleString(s), sut("text", "text", "abc"));
    }

    #[test]
    fn parse_mediumtext() {
        let s = String::from("abc");
        assert_eq!(SimpleString(s), sut("mediumtext", "mediumtext", "abc"));
    }

    #[test]
    fn parse_longtext() {
        let s = String::from("abc");
        assert_eq!(SimpleString(s), sut("longtext", "longtext", "abc"));
    }

    #[test]
    fn parse_enum() {
        let s = String::from("abc");
        assert_eq!(SimpleString(s), sut("enum", "enum('abc','def')", "abc"));
    }

    #[test]
    fn parse_set() {
        let s = String::from("abc,def");
        assert_eq!(SimpleString(s), sut("set", "set('abc','def')", "abc,def"));
    }

    #[test]
    fn parse_json() {
        let s = String::from(r#"{"id": 1, "name": "John"}"#);
        assert_eq!(JsonString(s), sut("json", "json", r#"{"id": 1, "name": "John"}"#));
    }
}
