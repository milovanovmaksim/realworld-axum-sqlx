-- Add up migration script here

create table if not exists articles
(
        id UUID primary key default uuid_generate_v4(),
        title text not null,
        body text not null,
        description text not null,
        slug, text not null,
        user_id bigint not null references users (id) on delete cascade,
        created_at timestamptz not null default current_timestamp,
        updated_at timestamptz default current_timestamp
        
        
)
