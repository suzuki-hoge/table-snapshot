create table 13_string_blob
(
    id             int auto_increment,
    col_tinyblob   tinyblob,
    col_blob       blob,
    col_mediumblob mediumblob,
    col_longblob   longblob,
    primary key (id)
);
