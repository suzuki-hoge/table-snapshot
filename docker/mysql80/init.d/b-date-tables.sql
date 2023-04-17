create table 06_date_date
(
    col_date date
);
insert into 06_date_date
values ('2020-01-01');

create table 07_date_time
(
    col_time time
);
insert into 07_date_time
values ('00:00:00');

create table 08_date_datetime
(
    col_datetime datetime
);
insert into 08_date_datetime
values ('2020-01-01 00:00:00');

create table 09_date_timestamp
(
    col_timestamp timestamp
);
insert into 09_date_timestamp
values ('2020-01-01 00:00:00');

create table 10_date_year
(
    col_year year
);
insert into 10_date_year
values (2020);

select table_name, column_name, data_type, column_type
from information_schema.columns
where table_name in ('06_date_date', '07_date_time', '08_date_datetime', '09_date_timestamp', '10_date_year')
order by table_name, ordinal_position;