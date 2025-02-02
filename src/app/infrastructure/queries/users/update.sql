update users set 
    username=COALESCE($1, username),
    email=COALESCE($2, email),
    password=COALESCE($3, password), 
    bio=COALESCE($4, bio), 
    image=COALESCE($5, image), 
    updated_at = current_timestamp
where id=$6
returning *
