// This file was generated with `clorinde`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct SetTrashedAtParams {
    pub trashed_at: Option<chrono::NaiveDateTime>,
    pub id: i64,
}
#[derive(Debug)]
pub struct UpdateMetadataParams<T1: crate::StringSql> {
    pub mime_type: T1,
    pub size: i64,
    pub id: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FileEntity {
    pub id: i64,
    pub path: String,
    pub size: Option<i64>,
    pub mime_type: Option<String>,
    pub trashed_at: Option<chrono::NaiveDateTime>,
}
pub struct FileEntityBorrowed<'a> {
    pub id: i64,
    pub path: &'a str,
    pub size: Option<i64>,
    pub mime_type: Option<&'a str>,
    pub trashed_at: Option<chrono::NaiveDateTime>,
}
impl<'a> From<FileEntityBorrowed<'a>> for FileEntity {
    fn from(
        FileEntityBorrowed {
            id,
            path,
            size,
            mime_type,
            trashed_at,
        }: FileEntityBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            path: path.into(),
            size,
            mime_type: mime_type.map(|v| v.into()),
            trashed_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct I64Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i64, tokio_postgres::Error>,
    mapper: fn(i64) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I64Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'c, 'a, 's, C, R, N> {
        I64Query {
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
pub struct FileEntityQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<FileEntityBorrowed, tokio_postgres::Error>,
    mapper: fn(FileEntityBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> FileEntityQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(FileEntityBorrowed) -> R,
    ) -> FileEntityQuery<'c, 'a, 's, C, R, N> {
        FileEntityQuery {
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
pub struct InsertFileStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn insert_file() -> InsertFileStmt {
    InsertFileStmt("INSERT INTO files (path) VALUES ($1) RETURNING id", None)
}
impl InsertFileStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        path: &'a T1,
    ) -> I64Query<'c, 'a, 's, C, i64, 1> {
        I64Query {
            client,
            params: [path],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct GetFileStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_file() -> GetFileStmt {
    GetFileStmt("SELECT * FROM files WHERE id = $1", None)
}
impl GetFileStmt {
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
        id: &'a i64,
    ) -> FileEntityQuery<'c, 'a, 's, C, FileEntity, 1> {
        FileEntityQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<FileEntityBorrowed, tokio_postgres::Error> {
                    Ok(FileEntityBorrowed {
                        id: row.try_get(0)?,
                        path: row.try_get(1)?,
                        size: row.try_get(2)?,
                        mime_type: row.try_get(3)?,
                        trashed_at: row.try_get(4)?,
                    })
                },
            mapper: |it| FileEntity::from(it),
        }
    }
}
pub struct ListFilesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn list_files() -> ListFilesStmt {
    ListFilesStmt("SELECT * FROM files", None)
}
impl ListFilesStmt {
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
    ) -> FileEntityQuery<'c, 'a, 's, C, FileEntity, 0> {
        FileEntityQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<FileEntityBorrowed, tokio_postgres::Error> {
                    Ok(FileEntityBorrowed {
                        id: row.try_get(0)?,
                        path: row.try_get(1)?,
                        size: row.try_get(2)?,
                        mime_type: row.try_get(3)?,
                        trashed_at: row.try_get(4)?,
                    })
                },
            mapper: |it| FileEntity::from(it),
        }
    }
}
pub struct SetTrashedAtStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn set_trashed_at() -> SetTrashedAtStmt {
    SetTrashedAtStmt("UPDATE files SET trashed_at = $1 WHERE id = $2", None)
}
impl SetTrashedAtStmt {
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
        trashed_at: &'a Option<chrono::NaiveDateTime>,
        id: &'a i64,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[trashed_at, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        SetTrashedAtParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for SetTrashedAtStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a SetTrashedAtParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.trashed_at, &params.id))
    }
}
pub struct DeleteFileStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_file() -> DeleteFileStmt {
    DeleteFileStmt("DELETE FROM files WHERE id = $1", None)
}
impl DeleteFileStmt {
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
pub struct UpdateMetadataStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_metadata() -> UpdateMetadataStmt {
    UpdateMetadataStmt(
        "UPDATE files SET mime_type = $1, size = $2 WHERE id = $3",
        None,
    )
}
impl UpdateMetadataStmt {
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
        mime_type: &'a T1,
        size: &'a i64,
        id: &'a i64,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[mime_type, size, id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateMetadataParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateMetadataStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateMetadataParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.mime_type, &params.size, &params.id))
    }
}
