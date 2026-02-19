// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct UpdateStatusParams<T1: crate::StringSql> {
    pub status: T1,
    pub id: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct EventEntity {
    pub id: i64,
    pub status: String,
    pub payload: String,
}
pub struct EventEntityBorrowed<'a> {
    pub id: i64,
    pub status: &'a str,
    pub payload: &'a str,
}
impl<'a> From<EventEntityBorrowed<'a>> for EventEntity {
    fn from(
        EventEntityBorrowed {
            id,
            status,
            payload,
        }: EventEntityBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            status: status.into(),
            payload: payload.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct EventEntityQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<EventEntityBorrowed, tokio_postgres::Error>,
    mapper: fn(EventEntityBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> EventEntityQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(EventEntityBorrowed) -> R,
    ) -> EventEntityQuery<'c, 'a, 's, C, R, N> {
        EventEntityQuery {
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
    EnqueueStmt("INSERT INTO jobs (payload) VALUES ($1)", None)
}
impl EnqueueStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        payload: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[payload]).await
    }
}
pub struct DequeueStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn dequeue() -> DequeueStmt {
    DequeueStmt(
        "UPDATE jobs SET status = 'Running' WHERE id = (SELECT id FROM jobs WHERE status = 'Pending' LIMIT 1) RETURNING *",
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
    ) -> EventEntityQuery<'c, 'a, 's, C, EventEntity, 0> {
        EventEntityQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<EventEntityBorrowed, tokio_postgres::Error> {
                    Ok(EventEntityBorrowed {
                        id: row.try_get(0)?,
                        status: row.try_get(1)?,
                        payload: row.try_get(2)?,
                    })
                },
            mapper: |it| EventEntity::from(it),
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
    UpdateStatusStmt("UPDATE jobs SET status = $1 WHERE id = $2", None)
}
impl UpdateStatusStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        status: &'a T1,
        id: &'a i64,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[status, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateStatusParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateStatusStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateStatusParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.status, &params.id))
    }
}
