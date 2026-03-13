use async_trait::async_trait;
use crate::di::unit_of_work::UnitOfWork;

#[async_trait]
pub trait SQLProvider: Clone + Send + Sync {
        type UnitOfWork<'a>: UnitOfWork<'a> where Self: 'a;

        async fn init(&self) -> Result<(), anyhow::Error>;
        async fn begin(&self) -> Result<Self::UnitOfWork<'_>, anyhow::Error>;
}