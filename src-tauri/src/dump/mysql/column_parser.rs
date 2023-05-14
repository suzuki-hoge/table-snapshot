use crate::core::types::Column;
use crate::core::types::Column::{
    BinaryString, BitNumber, DateString, JsonString, ParseError, SimpleNumber, SimpleString,
};
use crate::dump::mysql::schema_query::ColumnSchema;

pub fn parse(column_schema: &ColumnSchema, value: String) -> Column {
    match column_schema.data_type.as_str() {
        "tinyint" | "smallint" | "mediumint" | "int" | "bigint" => SimpleNumber(value),
        "decimal" | "float" | "double" => SimpleNumber(value),
        "bit" => BitNumber(value),
        "date" | "time" | "datetime" | "timestamp" | "year" => DateString(value),
        "char" | "varchar" => SimpleString(value),
        "binary" | "varbinary" => BinaryString(value),
        "tinyblob" | "mediumblob" | "blob" | "longblob" => BinaryString(value),
        "tinytext" | "mediumtext" | "text" | "longtext" => SimpleString(value),
        "enum" | "set" => SimpleString(value),
        "json" => JsonString(value),
        _ => ParseError(value),
    }
}

#[cfg(test)]
mod parse_tests {
    use crate::core::types::Column;
    use crate::dump::mysql::column_parser::parse;
    use crate::dump::mysql::schema_query::ColumnSchema;

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
        let exp = "42";
        assert_eq!(exp, sut("tinyint", "tinyint", "42").show());
    }

    #[test]
    fn parse_u_tinyint() {
        let exp = "42";
        assert_eq!(exp, sut("tinyint", "tinyint unsigned", "42").show());
    }

    #[test]
    fn parse_i_smallint() {
        let exp = "42";
        assert_eq!(exp, sut("smallint", "smallint", "42").show());
    }

    #[test]
    fn parse_u_smallint() {
        let exp = "42";
        assert_eq!(exp, sut("smallint", "smallint unsigned", "42").show());
    }

    #[test]
    fn parse_i_mediumint() {
        let exp = "42";
        assert_eq!(exp, sut("mediumint", "mediumint", "42").show());
    }

    #[test]
    fn parse_u_mediumint() {
        let exp = "42";
        assert_eq!(exp, sut("mediumint", "mediumint unsigned", "42").show());
    }

    #[test]
    fn parse_i_int() {
        let exp = "42";
        assert_eq!(exp, sut("int", "int", "42").show());
    }

    #[test]
    fn parse_u_int() {
        let exp = "42";
        assert_eq!(exp, sut("int", "int unsigned", "42").show());
    }

    #[test]
    fn parse_i_bigint() {
        let exp = "42";
        assert_eq!(exp, sut("bigint", "bigint", "42").show());
    }

    #[test]
    fn parse_u_bigint() {
        let exp = "42";
        assert_eq!(exp, sut("bigint", "bigint unsigned", "42").show());
    }

    #[test]
    fn parse_decimal() {
        let exp = "42.0";
        assert_eq!(exp, sut("decimal", "decimal(5,2)", "42.0").show());
    }

    #[test]
    fn parse_float() {
        let exp = "42.0";
        assert_eq!(exp, sut("float", "float(5,2)", "42.0").show());
    }

    #[test]
    fn parse_double() {
        let exp = "42.0";
        assert_eq!(exp, sut("double", "double(5,2)", "42.0").show());
    }

    #[test]
    fn parse_bit() {
        let exp = "bit(111)";
        assert_eq!(exp, sut("bit", "bit(3)", "111").show());
    }

    #[test]
    fn parse_date() {
        let exp = r#""2020-01-01""#;
        assert_eq!(exp, sut("date", "date", "2020-01-01").show());
    }

    #[test]
    fn parse_time() {
        let exp = r#""12:34:56""#;
        assert_eq!(exp, sut("time", "time", "12:34:56").show());
    }

    #[test]
    fn parse_datetime() {
        let exp = r#""2020-01-01 12:34:56""#;
        assert_eq!(exp, sut("datetime", "datetime", "2020-01-01 12:34:56").show());
    }

    #[test]
    fn parse_timestamp() {
        let exp = r#""2020-01-01 12:34:56""#;
        assert_eq!(exp, sut("timestamp", "timestamp", "2020-01-01 12:34:56").show());
    }

    #[test]
    fn parse_year() {
        let exp = r#""2020""#;
        assert_eq!(exp, sut("year", "year", "2020").show());
    }

    #[test]
    fn parse_char() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("char", "char(3)", "abc").show());
    }

    #[test]
    fn parse_varchar() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("varchar", "varchar(3)", "abc").show());
    }

    #[test]
    fn parse_binary() {
        let exp = "binary";
        assert_eq!(exp, sut("binary", "binary(3)", "abc").show());
    }

    #[test]
    fn parse_varbinary() {
        let exp = "binary";
        assert_eq!(exp, sut("varbinary", "varbinary(3)", "abc").show());
    }

    #[test]
    fn parse_tinyblob() {
        let exp = "binary";
        assert_eq!(exp, sut("tinyblob", "tinyblob", "abc").show());
    }

    #[test]
    fn parse_blob() {
        let exp = "binary";
        assert_eq!(exp, sut("blob", "blob", "abc").show());
    }

    #[test]
    fn parse_mediumblob() {
        let exp = "binary";
        assert_eq!(exp, sut("mediumblob", "mediumblob", "abc").show());
    }

    #[test]
    fn parse_longblob() {
        let exp = "binary";
        assert_eq!(exp, sut("longblob", "longblob", "abc").show());
    }

    #[test]
    fn parse_tinytext() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("tinytext", "tinytext", "abc").show());
    }

    #[test]
    fn parse_text() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("text", "text", "abc").show());
    }

    #[test]
    fn parse_mediumtext() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("mediumtext", "mediumtext", "abc").show());
    }

    #[test]
    fn parse_longtext() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("longtext", "longtext", "abc").show());
    }

    #[test]
    fn parse_enum() {
        let exp = r#""abc""#;
        assert_eq!(exp, sut("enum", "enum('abc','def')", "abc").show());
    }

    #[test]
    fn parse_set() {
        let exp = r#""abc,def""#;
        assert_eq!(exp, sut("set", "set('abc','def')", "abc,def").show());
    }

    #[test]
    fn parse_json() {
        let exp = r#"{"id": 1, "name": "John"}"#;
        assert_eq!(exp, sut("json", "json", r#"{"id": 1, "name": "John"}"#).show());
    }
}
