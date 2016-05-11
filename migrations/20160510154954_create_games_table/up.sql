create table games (
    id bigserial primary key,
    token_id bigserial references tokens (id) not null,
    game_state text not null,
    outcome varchar(10) not null
);

create index games_by_token_id on games (token_id);
create index games_by_outcome on games (outcome);