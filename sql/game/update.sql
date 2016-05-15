update games
set
    state = $2,
    outcome = $3
where id = $1