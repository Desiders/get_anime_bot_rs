use async_trait::async_trait;

#[async_trait]
pub trait UnitOfWork {
    async fn commit(&self);

    async fn rollback(&self);
}
