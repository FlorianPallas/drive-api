use anyhow::Result;
use sqlx::PgConnection;

#[derive(Debug, Clone)]
pub struct FileEntity {
    pub id: i32,
    pub path: String,
    pub size: i64,
    pub mime_type: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct FileRepository {}

impl FileRepository {
    pub async fn get_file_by_id(
        &self,
        conn: &mut PgConnection,
        id: i32,
    ) -> Result<Option<FileEntity>> {
        let entity = sqlx::query_as!(FileEntity, "SELECT * FROM files WHERE id = $1", id)
            .fetch_optional(conn)
            .await?;
        Ok(entity)
    }

    pub async fn insert_file(
        &self,
        conn: &mut PgConnection,
        path: &str,
        size: i64,
        mime_type: &str,
    ) -> Result<FileEntity> {
        let entity = sqlx::query_as!(
            FileEntity,
            "INSERT INTO files (path, size, mime_type) VALUES ($1, $2, $3) RETURNING *",
            path,
            size,
            mime_type
        )
        .fetch_one(conn)
        .await?;
        Ok(entity)
    }

    pub async fn list_files(
        &self,
        conn: &mut PgConnection,
        prefix: &str,
    ) -> Result<Vec<FileEntity>> {
        let files = sqlx::query_as!(
            FileEntity,
            "SELECT * FROM files WHERE path LIKE $1",
            format!("{}%", prefix)
        )
        .fetch_all(conn)
        .await?;
        Ok(files)
    }

    pub async fn delete_file_by_id(&self, conn: &mut PgConnection, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM files WHERE id = $1", id)
            .execute(conn)
            .await?;
        Ok(())
    }
}
