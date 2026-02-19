CREATE TABLE files (
    id BIGSERIAL PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    size BIGINT,
    mime_type TEXT,
    trashed_at TIMESTAMP
);

CREATE TABLE jobs (
    id BIGSERIAL PRIMARY KEY,
    status TEXT NOT NULL DEFAULT 'Pending',
    payload TEXT NOT NULL
);
