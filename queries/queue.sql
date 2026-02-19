--: EventEntity(id, status, payload)

--! enqueue
INSERT INTO jobs (payload, type) VALUES (:payload, :type);

--! dequeue : EventEntity
UPDATE jobs SET status = 'Running' WHERE id = (SELECT id FROM jobs WHERE status = 'Pending' AND type = ANY(ARRAY[:types::VARCHAR[]]) LIMIT 1) RETURNING *;

--! delete
DELETE FROM jobs WHERE id = :id;

--! update_status
UPDATE jobs SET status = :status WHERE id = :id;
