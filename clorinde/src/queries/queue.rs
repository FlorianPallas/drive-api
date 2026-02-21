// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct EnqueueParams<T1: crate::JsonSql> {
    pub payload: T1,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Clone, Copy, Debug)]
pub struct UpdateStatusParams {
    pub status: crate::types::JobStatus,
    pub updated_at: chrono::NaiveDateTime,
    pub id: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct JobEntity {
    pub id: i64,
    pub status: crate::types::JobStatus,
    pub payload: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
pub struct JobEntityBorrowed<'a> {
    pub id: i64,
    pub status: crate::types::JobStatus,
    pub payload: postgres_types::Json<&'a serde_json::value::RawValue>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
impl<'a> From<JobEntityBorrowed<'a>> for JobEntity {
    fn from(
        JobEntityBorrowed {
            id,
            status,
            payload,
            created_at,
            updated_at,
        }: JobEntityBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status,
            payload: serde_json::from_str(payload.0.get()).unwrap(),
            created_at,
            updated_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct JobEntityQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<JobEntityBorrowed, tokio_postgres::Error>,
    mapper: fn(JobEntityBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> JobEntityQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(JobEntityBorrowed) -> R) -> JobEntityQuery<'c, 'a, 's, C, R, N> {
        JobEntityQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct EnqueueStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn enqueue() -> EnqueueStmt {
    EnqueueStmt(
        "INSERT INTO jobs (payload, created_at, updated_at) VALUES ($1, $2, $3)",
        None,
    )
}
impl EnqueueStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::JsonSql>(
        &'s self,
        client: &'c C,
        payload: &'a T1,
        created_at: &'a chrono::NaiveDateTime,
        updated_at: &'a chrono::NaiveDateTime,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[payload, created_at, updated_at])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::JsonSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        EnqueueParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for EnqueueStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a EnqueueParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.payload,
            &params.created_at,
            &params.updated_at,
        ))
    }
}
pub struct DequeueStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn dequeue() -> DequeueStmt {
    DequeueStmt(
        "UPDATE jobs SET status = 'running', updated_at = $1 WHERE id = ( SELECT id FROM jobs WHERE status = 'pending' ORDER BY created_at FOR UPDATE SKIP LOCKED LIMIT 1 ) RETURNING *",
        None,
    )
}
impl DequeueStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        updated_at: &'a chrono::NaiveDateTime,
    ) -> JobEntityQuery<'c, 'a, 's, C, JobEntity, 1> {
        JobEntityQuery {
            client,
            params: [updated_at],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<JobEntityBorrowed, tokio_postgres::Error> {
                    Ok(JobEntityBorrowed {
                        id: row.try_get(0)?,
                        status: row.try_get(1)?,
                        payload: row.try_get(2)?,
                        created_at: row.try_get(3)?,
                        updated_at: row.try_get(4)?,
                    })
                },
            mapper: |it| JobEntity::from(it),
        }
    }
}
pub struct DeleteStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete() -> DeleteStmt {
    DeleteStmt("DELETE FROM jobs WHERE id = $1", None)
}
impl DeleteStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a i64,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[id]).await
    }
}
pub struct UpdateStatusStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_status() -> UpdateStatusStmt {
    UpdateStatusStmt(
        "UPDATE jobs SET status = $1, updated_at = $2 WHERE id = $3",
        None,
    )
}
impl UpdateStatusStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        status: &'a crate::types::JobStatus,
        updated_at: &'a chrono::NaiveDateTime,
        id: &'a i64,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[status, updated_at, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateStatusParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateStatusStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateStatusParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.status, &params.updated_at, &params.id))
    }
}
