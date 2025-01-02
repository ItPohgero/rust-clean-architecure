use crate::domain::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_all(&self) -> Vec<User>;
    async fn find_by_id(&self, id: i32) -> Option<User>;
}