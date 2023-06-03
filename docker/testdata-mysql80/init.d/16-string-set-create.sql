create table 16_string_set
(
    id      int auto_increment,
    col_set set ('pc', 'phone'),
    primary key (id)
);
