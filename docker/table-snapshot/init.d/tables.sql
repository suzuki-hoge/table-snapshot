create table project
(
    project_id char(36),
    rdbms      varchar(16),
    user       varchar(32),
    password   varchar(32),
    host       varchar(32),
    port       varchar(8),
    `schema`   varchar(64),
    primary key (project_id)
);

create table snapshot_summary
(
    snapshot_id   char(36),
    project_id    char(36),
    snapshot_name varchar(256),
    create_at     char(32),
    primary key (snapshot_id),
    foreign key (project_id) references project (project_id) on delete cascade
);

create table table_snapshot
(
    snapshot_id char(36),
    table_name  varchar(256),
    data        json,
    primary key (snapshot_id, table_name),
    foreign key (snapshot_id) references snapshot_summary (snapshot_id) on delete cascade
);

create table snapshot_diff
(
    diff_id      char(36),
    snapshot_id1 char(36),
    snapshot_id2 char(36),
    data         json,
    primary key (diff_id),
    unique (snapshot_id1, snapshot_id2),
    foreign key (snapshot_id1) references snapshot_summary (snapshot_id) on delete cascade,
    foreign key (snapshot_id2) references snapshot_summary (snapshot_id) on delete cascade
);
