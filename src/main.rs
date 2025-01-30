mod db;
mod models;
mod routes;
mod auth;

use rocket::{
    http::Method::{Connect, Delete, Get, Patch, Post, Put},
    launch, routes,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket_db_pools::Database;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Get, Post, Put, Patch, Delete, Connect]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true);

    rocket::build()
        .attach(db::AuthRsDatabase::init())
        .attach(cors.to_cors().unwrap())
        .mount(
            "/api",
            routes![
                // Audit Log routes
                routes::audit_logs::get_by_type::get_audit_logs_by_type,
                routes::audit_logs::get_by_id::get_audit_log_by_id,
                routes::audit_logs::get_by_entity_id::get_audit_log_by_entity_id,
                routes::audit_logs::get_by_user_id::get_audit_logs_by_user_id,

                // User Routes
                routes::users::create::create_user,
                routes::users::get_all::get_all_users,
                routes::users::get_by_id::get_user_by_id,
                routes::users::update::update_user,
                routes::users::delete::delete_user,
            ],
        )
}
