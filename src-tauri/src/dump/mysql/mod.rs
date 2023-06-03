use uuid::Uuid;

use crate::core::connector::Connector;
use crate::core::types::{SnapshotId, TableSummary};
use crate::dump::mysql::schema_query::{create_connection, get_col_schemata, get_rows, get_table_schemata};
use crate::dump::snapshot::{insert_rows, insert_table_summary};

mod column_parser;
mod schema_query;

pub fn dump(snapshot_connector: &Connector, target_connector: &Connector) -> anyhow::Result<SnapshotId> {
    let mut snapshot_connection = create_connection(snapshot_connector)?;
    let mut target_connection = create_connection(target_connector)?;

    let snapshot_id = Uuid::new_v4().to_string();

    let table_schemata = get_table_schemata(&mut target_connection, &target_connector.schema)?;

    for table_schema in table_schemata {
        let col_schemata = get_col_schemata(&mut target_connection, &target_connector.schema, &table_schema)?;

        let rows = get_rows(&mut target_connection, &table_schema, &col_schemata)?;

        let (primary_col_name, col_names) = col_schemata.get_col_names();
        let table_summary =
            TableSummary::new(snapshot_id.clone(), table_schema.table_name.clone(), primary_col_name, col_names, &rows);

        insert_table_summary(&mut snapshot_connection, &table_summary)?;
        insert_rows(&mut snapshot_connection, &table_summary, rows)?;
    }

    Ok(snapshot_id)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use mysql::Conn;
    use crate::core::connector::Connector;
    use crate::core::types::ColValue::*;
    use crate::dump::mysql::dump;
    use crate::dump::mysql::schema_query::{create_connection, get_table_schemata};
    use crate::dump::snapshot::{_find_rows, _find_table_summaries};

    fn s(s: &str) -> String {
        s.to_string()
    }
    
    fn drop_all(conn: &mut Conn, schema:&str)->anyhow::Result<()>{
        for table_schema in get_table_schemata(conn, schema)? {
            conn.prep_exec(format!("drop table {}", table_schema.table_name), ())?;
        }
        Ok(())
    }

    #[test]
    fn mysql80_data() -> anyhow::Result<()> {
        let snapshot_connector = Connector::mysql("user", "password", "127.0.0.1", "19000", "table-snapshot");
        let target_connector = Connector::mysql("user", "password", "127.0.0.1", "19001", "testdata");

        let mut s_conn = create_connection(&snapshot_connector)?;
        let mut t_conn = create_connection(&target_connector)?;

        drop_all(&mut t_conn, &target_connector.schema)?;

        t_conn.prep_exec("create table 01_number_signed ( id int auto_increment, col_tinyint tinyint, col_smallint smallint, col_mediumint mediumint, col_int int, col_bigint bigint, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 01_number_signed values (1, 127, 32767, 8388607, 2147483647, 9223372036854775807), (2, -128, -32768, -8388608, -2147483648, -9223372036854775808)", ())?;
        
        t_conn.prep_exec("create table 02_number_unsigned ( id int auto_increment, col_tinyint tinyint unsigned, col_smallint smallint unsigned, col_mediumint mediumint unsigned, col_int int unsigned, col_bigint bigint unsigned, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 02_number_unsigned values (1, 255, 65535, 16777215, 4294967295, 18446744073709551615), (2, 0, 0, 0, 0, 0)", ())?;
        
        t_conn.prep_exec("create table 03_number_fixed ( id int auto_increment, col_decimal decimal(5, 2), col_numeric numeric(5, 2), primary key (id) )", ())?;
        t_conn.prep_exec("insert into 03_number_fixed values (1, -999.99, -999.99), (2, 999.99, 999.99)", ())?;
        
        t_conn.prep_exec("create table 04_number_float ( id int auto_increment, col_float float(5, 2), col_double double(5, 2), primary key (id) )", ())?;
        t_conn.prep_exec("insert into 04_number_float values (1, -999.99, -999.99), (2, 999.99, 999.99)", ())?;
        
        t_conn.prep_exec("create table 05_number_bit ( id int auto_increment, col_bit bit(10), primary key (id) )", ())?;
        t_conn.prep_exec("insert into 05_number_bit values (1, b'1000000000'), (2, b'0'), (3, 512), (4, 0)", ())?;
        
        t_conn.prep_exec("create table 06_date_date ( id int auto_increment, col_date date, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 06_date_date values (1, '2020-01-01')", ())?;
        
        t_conn.prep_exec("create table 07_date_time ( id int auto_increment, col_time time, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 07_date_time values (1, '00:00:00')", ())?;
        
        t_conn.prep_exec("create table 08_date_datetime ( id int auto_increment, col_datetime datetime, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 08_date_datetime values (1, '2020-01-01 00:00:00')", ())?;
        
        t_conn.prep_exec("create table 09_date_timestamp ( id int auto_increment, col_timestamp timestamp, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 09_date_timestamp values (1, '2020-01-01 00:00:00')", ())?;
        
        t_conn.prep_exec("create table 10_date_year ( id int auto_increment, col_year year, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 10_date_year values (1, 2020)", ())?;
        
        t_conn.prep_exec("create table 11_string_char ( id int auto_increment, col_char char(3), col_varchar varchar(3), primary key (id) )", ())?;
        t_conn.prep_exec("insert into 11_string_char values (1, 'abc', 'abc'), (2, '', '')", ())?;
        
        t_conn.prep_exec("create table 12_string_binary ( id int auto_increment, col_binary binary(3), col_varbinary varbinary(3), primary key (id) )", ())?;
        t_conn.prep_exec("insert into 12_string_binary values (1, 'abc', 'abc')", ())?;
        
        t_conn.prep_exec("create table 13_string_blob ( id int auto_increment, col_tinyblob tinyblob, col_blob blob, col_mediumblob mediumblob, col_longblob longblob, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 13_string_blob values (1, 'abc', 'abc', 'abc', 'abc')", ())?;
        
        t_conn.prep_exec("create table 14_string_text ( id int auto_increment, col_tinytext tinytext, col_text text, col_mediumtext mediumtext, col_longtext longtext, primary key (id) )", ())?;
        t_conn.prep_exec("insert into 14_string_text values (1, 'abc', 'abc', 'abc', 'abc')", ())?;
        
        t_conn.prep_exec("create table 15_string_enum ( id int auto_increment, col_enum enum ('active', 'inactive'), primary key (id) )", ())?;
        t_conn.prep_exec("insert into 15_string_enum values (1, 'active'), (2, 'inactive')", ())?;
        
        t_conn.prep_exec("create table 16_string_set ( id int auto_increment, col_set set ('pc', 'phone'), primary key (id) )", ())?;
        t_conn.prep_exec("insert into 16_string_set values (1, 'pc'), (2, 'phone'), (3, 'phone,pc'), (4, 'pc,phone')", ())?;

        t_conn.prep_exec("create table 17_json_json ( id int auto_increment, col_json json, primary key (id) )", ())?;
        t_conn.prep_exec(r#"insert into 17_json_json values (1, '{"id": 1, "name": "John"}')"#, ())?;
        t_conn.prep_exec(r#"insert into 17_json_json values (2, '[1, 2, "foo"]')"#, ())?;
        t_conn.prep_exec(r#"insert into 17_json_json values (3, '{"items": ["pc", "phone"], "option": {"id": 1}}')"#, ())?;

        t_conn.prep_exec("create table 18_empty ( id int auto_increment, name varchar(3), primary key (id) )", ())?;
        t_conn.prep_exec("insert into 18_empty values (1, '')", ())?;
        t_conn.prep_exec("insert into 18_empty values (2, null)", ())?;
        
        let snapshot_id = dump(&snapshot_connector, &target_connector)?;

        let act_table_summaries = _find_table_summaries(&mut s_conn, &snapshot_id)?;
        assert_eq!("01_number_signed",   act_table_summaries[0].table_name);
        assert_eq!("02_number_unsigned", act_table_summaries[1].table_name);
        assert_eq!("03_number_fixed",    act_table_summaries[2].table_name);
        assert_eq!("04_number_float",    act_table_summaries[3].table_name);
        assert_eq!("05_number_bit",      act_table_summaries[4].table_name);
        assert_eq!("06_date_date",       act_table_summaries[5].table_name);
        assert_eq!("07_date_time",       act_table_summaries[6].table_name);
        assert_eq!("08_date_datetime",   act_table_summaries[7].table_name);
        assert_eq!("09_date_timestamp",  act_table_summaries[8].table_name);
        assert_eq!("10_date_year",       act_table_summaries[9].table_name);
        assert_eq!("11_string_char",     act_table_summaries[10].table_name);
        assert_eq!("12_string_binary",   act_table_summaries[11].table_name);
        assert_eq!("13_string_blob",     act_table_summaries[12].table_name);
        assert_eq!("14_string_text",     act_table_summaries[13].table_name);
        assert_eq!("15_string_enum",     act_table_summaries[14].table_name);
        assert_eq!("16_string_set",      act_table_summaries[15].table_name);
        assert_eq!("17_json_json",       act_table_summaries[16].table_name);
        assert_eq!("18_empty",           act_table_summaries[17].table_name);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("01_number_signed"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleNumber(s("127")), SimpleNumber(s("32767")), SimpleNumber(s("8388607")), SimpleNumber(s("2147483647")), SimpleNumber(s("9223372036854775807"))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), SimpleNumber(s("-128")), SimpleNumber(s("-32768")), SimpleNumber(s("-8388608")), SimpleNumber(s("-2147483648")), SimpleNumber(s("-9223372036854775808"))], act_rows[1].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("02_number_unsigned"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleNumber(s("255")), SimpleNumber(s("65535")), SimpleNumber(s("16777215")), SimpleNumber(s("4294967295")), SimpleNumber(s("18446744073709551615"))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), SimpleNumber(s("0")), SimpleNumber(s("0")), SimpleNumber(s("0")), SimpleNumber(s("0")), SimpleNumber(s("0"))], act_rows[1].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("03_number_fixed"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleNumber(s("-999")), SimpleNumber(s("-999"))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), SimpleNumber(s("999")), SimpleNumber(s("999"))], act_rows[1].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("04_number_float"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleNumber(s("-999")), SimpleNumber(s("-999"))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), SimpleNumber(s("999")), SimpleNumber(s("999"))], act_rows[1].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("05_number_bit"))?;
        assert_eq!(vec![SimpleNumber(s("1")), BitNumber(s("1000000000"))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), BitNumber(s("0"))], act_rows[1].col_values);
        assert_eq!(vec![SimpleNumber(s("3")), BitNumber(s("1000000000"))], act_rows[2].col_values);
        assert_eq!(vec![SimpleNumber(s("4")), BitNumber(s("0"))], act_rows[3].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("06_date_date"))?;
        assert_eq!(vec![SimpleNumber(s("1")), DateString(s("2020-01-01"))], act_rows[0].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("07_date_time"))?;
        assert_eq!(vec![SimpleNumber(s("1")), DateString(s("00:00:00"))], act_rows[0].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("08_date_datetime"))?;
        assert_eq!(vec![SimpleNumber(s("1")), DateString(s("2020-01-01 00:00:00"))], act_rows[0].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("09_date_timestamp"))?;
        assert_eq!(vec![SimpleNumber(s("1")), DateString(s("2020-01-01 00:00:00"))], act_rows[0].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("10_date_year"))?;
        assert_eq!(vec![SimpleNumber(s("1")), DateString(s("2020"))], act_rows[0].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("11_string_char"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleString(s("abc")), SimpleString(s("abc"))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), SimpleString(s("")), SimpleString(s(""))], act_rows[1].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("12_string_binary"))?;
        assert_eq!(vec![SimpleNumber(s("1")), BinaryString(s("abc")), BinaryString(s("abc"))], act_rows[0].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("13_string_blob"))?;
        assert_eq!(vec![SimpleNumber(s("1")), BinaryString(s("abc")), BinaryString(s("abc")), BinaryString(s("abc")), BinaryString(s("abc"))], act_rows[0].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("14_string_text"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleString(s("abc")), SimpleString(s("abc")), SimpleString(s("abc")), SimpleString(s("abc"))], act_rows[0].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("15_string_enum"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleString(s("active"))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), SimpleString(s("inactive"))], act_rows[1].col_values);
        
        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("16_string_set"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleString(s("pc"))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), SimpleString(s("phone"))], act_rows[1].col_values);
        assert_eq!(vec![SimpleNumber(s("3")), SimpleString(s("pc,phone"))], act_rows[2].col_values);
        assert_eq!(vec![SimpleNumber(s("4")), SimpleString(s("pc,phone"))], act_rows[3].col_values);
        
        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("17_json_json"))?;
        assert_eq!(vec![SimpleNumber(s("1")), JsonString(s(r#"{"id": 1, "name": "John"}"#))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), JsonString(s(r#"[1, 2, "foo"]"#))], act_rows[1].col_values);
        assert_eq!(vec![SimpleNumber(s("3")), JsonString(s(r#"{"items": ["pc", "phone"], "option": {"id": 1}}"#))], act_rows[2].col_values);

        let act_rows = _find_rows(&mut s_conn, &snapshot_id, &s("18_empty"))?;
        assert_eq!(vec![SimpleNumber(s("1")), SimpleString(s(""))], act_rows[0].col_values);
        assert_eq!(vec![SimpleNumber(s("2")), Null], act_rows[1].col_values);

        Ok(())
    }

    #[test]
    fn mysql80_col() -> anyhow::Result<()> {
        let snapshot_connector = Connector::mysql("user", "password", "127.0.0.1", "19000", "table-snapshot");
        let target_connector = Connector::mysql("user", "password", "127.0.0.1", "19001", "testdata");

        let mut s_conn = create_connection(&snapshot_connector)?;
        let mut t_conn = create_connection(&target_connector)?;

        drop_all(&mut t_conn, &target_connector.schema)?;

        t_conn.prep_exec("create table 01_primary ( id int auto_increment, primary key (id) )", ())?;

        let snapshot_id = dump(&snapshot_connector, &target_connector)?;

        let act_table_summaries = _find_table_summaries(&mut s_conn, &snapshot_id)?;
        assert_eq!("01_primary", act_table_summaries[0].table_name);

        Ok(())
    }
}
