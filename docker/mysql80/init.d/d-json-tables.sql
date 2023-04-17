create table 17_json_json
(
    col_json    json
);
insert into 17_json_json
values ('{"id": 1, "name": "John"}');

select table_name, column_name, data_type, column_type
from information_schema.columns
where table_name in ('17_json_json')
order by table_name, ordinal_position;
