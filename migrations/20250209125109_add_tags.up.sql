-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table if not exists tags
(
    id UUID primary key default uuid_generate_v4(),
    tag        varchar     not null default '',
    created_at timestamptz not null default current_timestamp
);

create index if not exists tags_tag_idx on tags (tag);
