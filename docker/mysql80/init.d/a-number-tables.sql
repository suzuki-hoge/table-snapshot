create table 01_number_signed
(
    col_tinyint   tinyint,
    col_smallint  smallint,
    col_mediumint mediumint,
    col_int       int,
    col_bigint    bigint
);
insert into 01_number_signed
values (127, 32767, 8388607, 2147483647, 9223372036854775807),
       (-128, -32768, -8388608, -2147483648, -9223372036854775808);

create table 02_number_unsigned
(
    col_tinyint   tinyint unsigned,
    col_smallint  smallint unsigned,
    col_mediumint mediumint unsigned,
    col_int       int unsigned,
    col_bigint    bigint unsigned
);
insert into 02_number_unsigned
values (255, 65535, 16777215, 4294967295, 18446744073709551615),
       (0, 0, 0, 0, 0);

create table 03_number_fixed
(
    col_decimal decimal(5, 2),
    col_numeric numeric(5, 2)
);
insert into 03_number_fixed
values (-999.99, -999.99),
       (999.99, 999.99);

create table 04_number_float
(
    col_float float(5, 2
) ,
    col_double double(5, 2)
);
insert into 04_number_float
values (-999.99, -999.99),
       (999.99, 999.99);

create table 05_number_bit
(
    col_bit bit(10)
);
insert into 05_number_bit
values (b'1000000000'),
       (b'0'),
       (512),
       (0);

select table_name, column_name, data_type, column_type
from information_schema.columns
where table_name in ('01_number_signed', '02_number_unsigned', '03_number_fixed', '04_number_float', '05_number_bit')
order by table_name, ordinal_position;