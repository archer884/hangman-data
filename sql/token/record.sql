select
    t.id as token_id,
    t.token,
    sum(g.wins) as wins,
    sum(g.losses) as losses

from tokens t
    join (
        select
            g.token_id,
            (case when g.outcome = 'Win' then 1 else 0 end) as wins,
            (case when g.outcome = 'Loss' then 1 else 0 end) as losses

        from games g
    ) as g on g.token_id = t.id

where t.token = $1

group by t.id, t.token