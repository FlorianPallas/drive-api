--: FileEntity(id, path, size?, mime_type?, trashed_at?)

--! insert_file
INSERT INTO files (path) VALUES (:path) RETURNING id;

--! get_file : FileEntity
SELECT * FROM files WHERE id = :id;

--! list_files : FileEntity
SELECT * FROM files;

--! set_trashed_at (trashed_at?)
UPDATE files SET trashed_at = :trashed_at WHERE id = :id;

--! delete_file
DELETE FROM files WHERE id = :id;

--! update_metadata
UPDATE files SET mime_type = :mime_type, size = :size WHERE id = :id;
