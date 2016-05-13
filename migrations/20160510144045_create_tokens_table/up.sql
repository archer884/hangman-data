create table tokens (
    id bigserial primary key,
    token varchar(240) not null
);

--Not sure whether this will work or not, but whatever... We'll find out
create unique index tokens_by_value on tokens (token);

insert into tokens (token) values ('sanford_and_son');
insert into tokens (token) values ('madder_hatter');