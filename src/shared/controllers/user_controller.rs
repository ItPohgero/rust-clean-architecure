use rocket::serde::json::Json;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::repositories::user_repository_impl::UserRepositoryImpl;
use crate::shared::dto::user_dto::UserDTO;

#[get("/user/list")]
pub async fn list_users() -> Json<Vec<UserDTO>> {
    let repository = UserRepositoryImpl;
    let users = repository.find_all().await;
    Json(users.into_iter().map(UserDTO::from).collect())
}

#[get("/user/<id>/show")]
pub async fn show_user(id: i32) -> Option<Json<UserDTO>> {
    let repository = UserRepositoryImpl;
    repository.find_by_id(id).await
        .map(|user| Json(UserDTO::from(user)))
}
