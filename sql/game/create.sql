insert into games (
    token_id,
    state,
    difficulty
) values (
    (select id from tokens t where t.token = $1 limit 1),
    $2,
    $3
) returning id