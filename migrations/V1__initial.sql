CREATE TABLE files (
    id BIGSERIAL PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    size BIGINT,
    mime_type TEXT,
    trashed_at TIMESTAMP
);

CREATE TYPE job_status AS ENUM ('pending', 'running', 'failed');

CREATE TABLE jobs (
    id BIGSERIAL PRIMARY KEY,
    status job_status NOT NULL DEFAULT 'pending',
    payload JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
