use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use rocket::response::status;
use crate::domain::repositories::user_repository::UserRepository;
use crate::shared::dto::user_dto::UserDTO;
use crate::domain::repositories::user_repository_impl::UserRepositoryImpl;
use crate::common::response::ApiResponse;


#[get("/user")]
pub async fn list_users(user_repo: &State<UserRepositoryImpl>) -> Json<ApiResponse<Vec<UserDTO>>> {
    let users = user_repo.find_all().await;
    Json(ApiResponse {
        code: 200,
        message: "Successfully retrieved users".to_string(),
        results: Some(users.into_iter().map(UserDTO::from).collect()),
    })
}

#[get("/user/<id>")]
pub async fn show_user(
    id: i32,
    user_repo: &State<UserRepositoryImpl>,
) -> Result<Json<ApiResponse<UserDTO>>, status::Custom<Json<ApiResponse<()>>>> {
    match user_repo.find_by_id(id).await {
        Some(user_data) => Ok(Json(ApiResponse {
            code: 200,
            message: "Successfully retrieved user".to_string(),
            results: Some(UserDTO::from(user_data)),
        })),
        None => Err(status::Custom(
            Status::BadRequest,
            Json(ApiResponse {
                code: 400,
                message: format!("User with id {} not found", id),
                results: None,
            }),
        )),
    }
}
