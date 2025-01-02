use rocket::serde::json::Json;
use rocket::State;
use crate::shared::dto::user_dto::UserDTO;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::repositories::user_repository_impl::UserRepositoryImpl;

#[get("/user/list")]
pub async fn list_users(user_repo: &State<UserRepositoryImpl>) -> Json<Vec<UserDTO>> {
    let users = user_repo.find_all().await;
    Json(users.into_iter().map(UserDTO::from).collect())
}

#[get("/user/<id>/show")]
pub async fn show_user(id: i32, user_repo: &State<UserRepositoryImpl>) -> Option<Json<UserDTO>> {
    user_repo.find_by_id(id).await
        .map(|user| Json(UserDTO::from(user)))
}
