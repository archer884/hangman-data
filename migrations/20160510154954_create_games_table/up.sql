create table games (
    id bigserial primary key,
    token_id bigserial references tokens (id) not null,
    game_state text not null,
    outcome varchar(10) not null
)