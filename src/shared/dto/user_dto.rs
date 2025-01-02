use serde::Serialize;

#[derive(Serialize)]
pub struct UserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<crate::domain::entities::user_entity::User> for UserDTO {
    fn from(user: crate::domain::entities::user_entity::User) -> Self {
        UserDTO {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}