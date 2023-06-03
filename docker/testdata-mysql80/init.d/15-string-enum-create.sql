create table 15_string_enum
(
    id       int auto_increment,
    col_enum enum ('active', 'inactive'),
    primary key (id)
);
