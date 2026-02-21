--: JobEntity(id, status, payload, created_at, updated_at)

--! enqueue
INSERT INTO jobs (payload, created_at, updated_at) VALUES (:payload, :created_at, :updated_at);

--! dequeue : JobEntity
UPDATE jobs
SET status = 'running', updated_at = :updated_at
WHERE id = (
    SELECT id
    FROM jobs
    WHERE status = 'pending'
    ORDER BY created_at
    FOR UPDATE SKIP LOCKED
    LIMIT 1
) RETURNING *;

--! delete
DELETE FROM jobs WHERE id = :id;

--! update_status
UPDATE jobs SET status = :status, updated_at = :updated_at WHERE id = :id;
