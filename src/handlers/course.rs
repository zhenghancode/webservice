use crate::state::AppState;
use actix_web::{web,HttpResponse,get,post,delete,put};
use actix_identity::Identity;
use crate::dbaccess::course::*;
use crate::errors::MyError;
use crate::models::course::{ UpdateCourse, CreateCourse};

#[post("/")]
pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
    id: Identity,
) -> Result<HttpResponse, MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course|HttpResponse::Ok().json(course))
    
}

#[get("/{teacher_id}")]
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    id: Identity,
) -> Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    // let  teacher_id = i32::try_from(params.0).unwrap();
    let teacher_id = params.into_inner();
    
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(| courses| HttpResponse::Ok().json(courses))
}

#[get("/{teacher_id}/{course_id}")]
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,i32)>,
    id: Identity,
) -> Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    // let  teacher_id = i32::try_from(params.0).unwrap();
    // let  course_id = i32::try_from(params.1).unwrap();
    let (teacher_id,course_id) = params.into_inner();

    get_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[delete("/{teacher_id}/{course_id}")]
pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,i32)>,
    id:Identity,
) ->Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    let (teacher_id,course_id) = params.into_inner();

    delete_course_db(&app_state.db, teacher_id, course_id)
    .await
    .map(|resp| HttpResponse::Ok().json(resp))
}


#[put("/{teacher_id}/{course_id}")]
pub async fn update_course_details(
    update_course: web::Json<UpdateCourse>,
    app_state: web::Data<AppState>,
    params: web::Path<(i32,i32)>,
    id: Identity,
) -> Result<HttpResponse,MyError> {

    if let None = id.identity() {
        return Err(MyError::Unauthored("not user login".into()));
    };

    let (teacher_id,course_id) = params.into_inner();

    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
    .await
    .map(|course| HttpResponse::Ok().json(course))
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

    #[ignore = "new course"]
    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let course = web::Json(CreateCourse {
            teacher_id:1,
            name: "Test Course".into(),

            description: None,
            format:None,
            structure:None,
            duration:None,
            price:None,
            language:Some("English".into()),
            level:Some("high".into()),
        });
        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        let resp = post_new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();


        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses:Mutex::new(vec![]),
            db: db_pool,
        });
        let teacher_id:web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();


        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses:Mutex::new(vec![]),
            db: db_pool,
        });
        let params: web::Path<(i32,i32)> = web::Path::from((1,17));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_failure() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();


        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses:Mutex::new(vec![]),
            db: db_pool,
        });
        let params: web::Path<(i32,i32)> = web::Path::from((1,100));
        let resp = get_course_detail(app_state, params).await;

        match resp {
            Ok(_) => println!("Something wrong...."),
            Err(err) => assert_eq!(err.status_code(),StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();


        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses:Mutex::new(vec![]),
            db: db_pool,
        });
        let update_course = UpdateCourse {
            name: Some("Course name changed".into()),

            description: Some("this is another test course".into()),
            format:None,
            structure:None,
            duration:None,
            price:Some(11),
            language:Some("Chinese".into()),
            level:Some("Intermediate".into()),
        };
        let params: web::Path<(i32,i32)> = web::Path::from((1,17));

        let resp = update_course_details(web::Json(update_course), app_state, params)
            .await
            .unwrap();

        assert_eq!(resp.status(),StatusCode::OK);

    }

    #[ignore = "delete course"]
    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();


        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses:Mutex::new(vec![]),
            db: db_pool,
        });
        let params: web::Path<(i32,i32)> = web::Path::from((1,19));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();


        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses:Mutex::new(vec![]),
            db: db_pool,
        });
        let params: web::Path<(i32,i32)> = web::Path::from((1,302));
        let resp = delete_course(app_state, params).await;

        match resp {
            Ok(_) => println!("Something wrong....."),
            Err(err) => assert_eq!(err.status_code(),StatusCode::NOT_FOUND),
        };
        
    }



} */