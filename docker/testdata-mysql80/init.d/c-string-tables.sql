create table 11_string_char
(
    col_char    char(3),
    col_varchar varchar(3)
);
insert into 11_string_char
values ('abc', 'abc'),
       ('', '');

create table 12_string_binary
(
    col_binary    binary(3),
    col_varbinary varbinary(3)
);
insert into 12_string_binary
values ('abc', 'abc');

create table 13_string_blob
(
    col_tinyblob   tinyblob,
    col_blob       blob,
    col_mediumblob mediumblob,
    col_longblob   longblob
);
insert into 13_string_blob
values ('abc', 'abc', 'abc', 'abc');

create table 14_string_text
(
    col_tinytext   tinytext,
    col_text       text,
    col_mediumtext mediumtext,
    col_longtext   longtext
);
insert into 14_string_text
values ('abc', 'abc', 'abc', 'abc');

create table 15_string_enum
(
    col_enum enum ('active', 'inactive')
);
insert into 15_string_enum
values ('active'),
       ('inactive');

create table 16_string_set
(
    col_set set ('pc', 'phone')
);
insert into 16_string_set
values ('pc'),
       ('phone'),
       ('phone,pc'),
       ('pc,phone');

select table_name, column_name, data_type, column_type
from information_schema.columns
where table_name in ('11_string_char', '12_string_binary', '13_string_blob', '14_string_text', '15_string_enum', '16_string_set')
order by table_name, ordinal_position;
