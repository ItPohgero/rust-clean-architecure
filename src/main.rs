#[macro_use] extern crate rocket;

mod domain;
mod shared;
mod infrastructure;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            shared::controllers::health_controller::health_check,
            shared::controllers::user_controller::list_users,
            shared::controllers::user_controller::show_user,
        ])
}