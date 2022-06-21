use actix_web::{web,HttpResponse,get,post,put,delete};
use actix_identity::Identity;
use crate::dbaccess::teacher::*;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;
use crate::errors::MyError;


#[post("/")]
pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>,
    id:Identity,
) -> Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    post_new_teacher_db(&app_state.db, new_teacher.into())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

#[get("/")]
pub async fn get_all_teachers(
    app_state: web::Data<AppState>,
    id:Identity,
) -> Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

#[get("/{teacher_id}")]
pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    id:Identity,
) -> Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    let teacher_id = params.into_inner();

    get_teacher_details_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}


#[delete("/{teacher_id}")]
pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    id: Identity,
) -> Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    let teacher_id = params.into_inner();

    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|result| HttpResponse::Ok().json(result))
}


#[put("/{teacher_id}")]
pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_teacher: web::Json<UpdateTeacher>,
    id: Identity,
) -> Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    let teacher_id = params.into_inner();

    update_teacher_details_db(&app_state.db, update_teacher.into(), teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}



/* #[cfg(test)]

mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use std::env;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use actix_web::ResponseError;


    #[ignore = "add teacher"]
    #[actix_rt::test]
    async fn post_teacher_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let new_teacher = web::Json(CreateTeacher {
            name: "zhenghan".into(),
            picture_url: "http:// goddd".into(),
            profile:"okkkk".into()

        });
        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let resp = post_new_teacher(new_teacher, app_state).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_teachers_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });

        let resp = get_all_teachers(app_state).await.unwrap();

        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_details_test_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        let params = web::Path::from(2);

        let resp = get_teacher_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_details_test_failure() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        let params = web::Path::from(200);

        let resp = get_teacher_details(app_state, params).await.unwrap_err();
        assert_eq!(resp.status_code(),StatusCode::NOT_FOUND);
    }

    #[ignore = "delete teacher"]
    #[actix_rt::test]
    async fn delete_course_test_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params = web::Path::from(1);

        let resp = delete_teacher(app_state, params).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_teacher_test_failure() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params = web::Path::from(1);

        let resp = delete_teacher(app_state, params).await.unwrap_err();
        assert_eq!(resp.status_code(),StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_rt::test]
    async fn update_teacher_details_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        let params = web::Path::from(2);
        let update_teacher = web::Json( UpdateTeacher {
            picture_url: Some("hahahha".into()),
            profile: Some("lalalalal".into()),
        });

        let resp = update_teacher_details(app_state, params,update_teacher).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
} */