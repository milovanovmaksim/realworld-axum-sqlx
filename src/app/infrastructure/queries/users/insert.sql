insert into users (username, email, password, bio, image)
values ($1, $2, $3, '', '') returning *;
