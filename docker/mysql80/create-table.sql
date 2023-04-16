create table number_table
(
    col_u_tinyint   tinyint unsigned,
    col_i_tinyint   tinyint,
    col_u_smallint  smallint unsigned,
    col_i_smallint  smallint,
    col_u_mediumint mediumint unsigned,
    col_i_mediumint mediumint,
    col_u_int       int unsigned,
    col_i_int       int,
    col_u_bigint    bigint unsigned,
    col_i_bigint    bigint
);

insert into number_table
values (255, 127,
        65535, 32767,
        16777215, 8388607,
        4294967295, 2147483647,
        18446744073709551615, 9223372036854775807),
       (0, -128,
        0, -32768,
        0, -8388608,
        0, -2147483648,
        0, -9223372036854775808);
