update users set 
    username=$1::text,
    email=$2::text,
    password=$3::text, 
    bio=$4::text, 
    image=$5::text, 
    updated_at = current_timestamp
where id=$6
returning *
