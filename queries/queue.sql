--: EventEntity(id, status, payload)

--! enqueue
INSERT INTO jobs (payload) VALUES (:payload);

--! dequeue : EventEntity
UPDATE jobs SET status = 'Running' WHERE id = (SELECT id FROM jobs WHERE status = 'Pending' LIMIT 1) RETURNING *;

--! delete
DELETE FROM jobs WHERE id = :id;

--! update_status
UPDATE jobs SET status = :status WHERE id = :id;
