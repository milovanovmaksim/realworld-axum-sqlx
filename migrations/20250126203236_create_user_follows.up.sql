-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table if not exists user_follows
(
    id UUID primary key default uuid_generate_v4(),
    follower_id UUID not null references users (id) on delete cascade,
    followee_id UUID not null references users (id) on delete cascade,  
    created_at timestamptz not null default current_timestamp
);
