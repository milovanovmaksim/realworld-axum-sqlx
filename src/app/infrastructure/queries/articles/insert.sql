insert into articles (title, body, description, slug, user_id)
    values ($1, $2, $3, $4, $5) returning *;
