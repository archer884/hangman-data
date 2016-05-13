create table tokens (
    id bigserial primary key,
    token varchar(240) not null
);

create index tokens_by_content on tokens (token);

insert into tokens (token) values ('sanford_and_son');
insert into tokens (token) values ('madder_hatter');