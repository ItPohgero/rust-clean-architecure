use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use async_trait::async_trait;

pub struct UserRepositoryImpl;

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_all(&self) -> Vec<User> {
        // Simulasi data
        vec![
            User { id: 1, name: "John Doe".to_string(), email: "john@example.com".to_string() },
            User { id: 2, name: "Jane Doe".to_string(), email: "jane@example.com".to_string() },
        ]
    }

    async fn find_by_id(&self, id: i32) -> Option<User> {
        // Simulasi pencarian user
        if id == 1 {
            Some(User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            })
        } else {
            None
        }
    }
}