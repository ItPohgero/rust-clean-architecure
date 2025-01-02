#[macro_use] extern crate rocket;

mod domain;
mod shared;
mod infrastructure;

use infrastructure::database::db::init_db_pool;
use crate::domain::repositories::user_repository_impl::UserRepositoryImpl;

#[launch]
async fn rocket() -> _ {
    let db_pool = init_db_pool().await;
    let user_repo = UserRepositoryImpl { pool: db_pool.clone() };
    rocket::build()
        .manage(db_pool) // Register db_pool to Rocket state
        .manage(user_repo) // Register user_repo to Rocket state
        .mount("/", routes![
            shared::controllers::health_controller::health_check,
            shared::controllers::user_controller::list_users,
            shared::controllers::user_controller::show_user,
        ])
}
