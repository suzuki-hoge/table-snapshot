create table table_summary
(
    snapshot_id      char(36),
    table_name       varchar(256),
    hash             char(32),
    primary_col_name varchar(64),
    col_names        json,
    primary key (snapshot_id, table_name)
);

create table row_data
(
    snapshot_id      char(36),
    table_name       varchar(256),
    hash             char(32),
    primary_value    varchar(256),
    col_values       json,
    foreign key (snapshot_id, table_name) references table_summary (snapshot_id, table_name)
);
