#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub results: Option<T>,
}
