create table tokens (
    id bigserial primary key,
    token varchar(240) not null
);

insert into tokens (token) values ('sanford_and_son');
insert into tokens (token) values ('madder_hatter');