use actix_cors::Cors;
use actix_web::{web,App,HttpServer, http,middleware};
use actix_identity::{IdentityService,CookieIdentityPolicy};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

#[path ="../errors.rs"]
mod errors;
#[path ="../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path ="../models/mod.rs"]
mod models;
#[path ="../utils.rs"]
mod utils;

use routers::*;
use state::AppState;
use errors::MyError;
use utils::SECRET_KEY;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "zhenghan.icu".into());

    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data= web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });
    let app = move || {
        let cors = Cors::default()
            // .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET","POST","DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION,http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let identity = IdentityService::new(
            CookieIdentityPolicy::new(SECRET_KEY.as_bytes())
                .domain(&domain)
                .name("auth")
                .path("/")
                .max_age(time::Duration::days(1))
                .login_deadline(time::Duration::days(1))
                .secure(false), // this can only be true if you have https
        );

        App::new()
            .app_data(shared_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(identity)
            .wrap(middleware::DefaultHeaders::new().add(("X-Version","1.0")))
            .app_data(web::JsonConfig::default().limit(4096).error_handler(|_err,_req|{
                MyError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .wrap(cors)
            .service(web::scope("/apiv1")
                .configure(auth_routes)
                .configure(general_routes)
                .configure(course_routes)
                .configure(teacher_routes)
            )
            
    };

    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}