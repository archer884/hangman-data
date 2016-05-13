update game g
set
    g.state = $2,
    g.outcome = $3
where g.id = $1