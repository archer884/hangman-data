select top 1 g.*
from games g
    join tokens t on g.token_id = t.id
where t.token = $1