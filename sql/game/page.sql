select *
    
from games g
    join tokens t on g.token_id = t.id
    
where t.token = $1
order by g.id
offset $2
limit $3