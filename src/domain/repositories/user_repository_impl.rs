use crate::domain::entities::user_entity::User;
use crate::domain::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::postgres::PgPool;

pub struct UserRepositoryImpl {
    pub pool: PgPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_all(&self) -> Vec<User> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email
            FROM users
            "#
        )
            .fetch_all(&self.pool)
            .await
            .unwrap_or_else(|_| vec![]); // Handle error dengan mengembalikan list kosong

        users
    }

    async fn find_by_id(&self, id: i32) -> Option<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email
            FROM users
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten(); // Handle error dengan mengembalikan None jika gagal

        user
    }
}
