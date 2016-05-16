create table games (
    id bigserial primary key,
    token_id bigserial references tokens (id) not null,
    state text not null,
    difficulty varchar(6) not null,
    outcome varchar(10) not null default 'Incomplete'
);

create index games_by_token_id on games (token_id);
create index games_by_outcome on games (outcome);

insert into games (token_id, state, difficulty) values (
    1,
    'unplayable',
    'Easy'
);