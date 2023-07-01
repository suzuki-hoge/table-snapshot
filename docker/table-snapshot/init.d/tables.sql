create table table_snapshot
(
    snapshot_id      char(36),
    table_name       varchar(256),
    primary_col_name varchar(64),
    col_names        json,
    hash             char(32),
    create_at        datetime default current_timestamp,
    primary key (snapshot_id, table_name)
);

create table row_snapshot
(
    snapshot_id   char(36),
    table_name    varchar(256),
    primary_value varchar(256),
    col_values    json,
    hash          char(32),
    primary key (snapshot_id, table_name, primary_value),
    foreign key (snapshot_id, table_name) references table_snapshot (snapshot_id, table_name)
);
