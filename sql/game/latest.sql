select g.*
from games g
    join tokens t on g.token_id = t.id
where t.token = $1
limit 1