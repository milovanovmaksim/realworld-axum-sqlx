-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table
    if not exists users (
        id UUID primary key default uuid_generate_v4(),
        username varchar not null,
        email varchar not null,
        password varchar not null,
        bio varchar,
        image varchar,
        created_at timestamptz not null default current_timestamp,
        updated_at timestamptz not null default current_timestamp
    );

create index if not exists users_email_idx on users (email);
