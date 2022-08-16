use actix_cors::Cors;
use actix_web::{web,
    App,
    HttpServer,
    http,middleware,
    // cookie::SameSite,
};
// use actix_identity::{IdentityService,CookieIdentityPolicy};
use actix_session::{
    SessionMiddleware,
    storage::RedisSessionStore,
    // SessionLength,
    config::PersistentSession,
};
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
    // let domain = env::var("DOMAIN").unwrap_or_else(|_| "zhenghan.icu".into());

    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not exist!");
    let redis_store = RedisSessionStore::new(redis_url).await.unwrap();

    let shared_data= web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        auth_key: "user".to_string(),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    // INFO level enabled
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET","POST","DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION,http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
            
        let session = SessionMiddleware::builder(
            redis_store.clone(), 
            SECRET_KEY.clone(),
        )
        .cookie_name("auth".to_string())
        .cookie_http_only(false)
        // .cookie_domain(Some(domain.clone()))
        .cookie_secure(false)
        // .cookie_same_site(SameSite::Strict)
        .session_lifecycle(
            PersistentSession::default()
                .session_ttl(time::Duration::hours(2))
        )
        .build();

        App::new()
            .app_data(shared_data.clone())
            .wrap(middleware::Logger::default().log_target("HAN"))
            .wrap(session)
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

    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await?;
    println!("async web server down ...");

    Ok(())
}