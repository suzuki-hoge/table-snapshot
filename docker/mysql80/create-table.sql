create table foo
(
    id   int(10),
    name varchar(16),
    type enum('a', 'b'),
    created date,
    last timestamp
);

insert into foo ( id, name, type, created, last ) values ( 1, 'John', 'a', '2020-01-01', '2022-12-31 23:59:59' );
