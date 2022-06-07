use super::state::AppState;
use actix_web::{web,HttpResponse};
use super::db_access::*;

pub async fn health_check_hanlder(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response =
        format!("{} {} times",health_check_response,visit_count);
    *visit_count+=1;
    HttpResponse::Ok().json(&response)
}

use super::models::Course;
// use chrono::Utc;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course!");
    /* let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len(); */
    /* let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count as usize + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    }; */
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    // app_state.courses.lock().unwrap().push(new_course); 
    HttpResponse::Ok().json(format!("Course added: {:?}",course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>
) -> HttpResponse {
    let  teacher_id = i32::try_from(params.0).unwrap();

    /* let filtered_courses =  app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id)
        .collect::<Vec<Course>>(); */
    
    let courses = get_courses_for_teacher_db(&app_state.db, teacher_id).await;

    
    if courses.len() > 0 {
        HttpResponse::Ok().json(courses)
    } else {
        HttpResponse::Ok().json("No courses found for teacher".to_string())
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,usize)>
) -> HttpResponse {
    let  teacher_id = i32::try_from(params.0).unwrap();
    let  course_id = i32::try_from(params.1).unwrap();

    /* let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|course| course.teacher_id == teacher_id && course.id == Some(course_id))
        .ok_or("Course not found"); */

    let course = get_course_detail_db(&app_state.db, teacher_id, course_id).await;

    HttpResponse::Ok().json(course)
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use std::env;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not exist!");
        let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let course = web::Json(Course {
            teacher_id:1,
            name: "Test Course".into(),
            id:None,
            time:None,
        });
        let app_state = web::Data::new(AppState {
            health_check_response: "".into(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool,
        });
        let resp = new_course(course, app_state).await;
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
        let teacher_id:web::Path<(usize,)> = web::Path::from((1,));
        let resp = get_courses_for_teacher(app_state, teacher_id).await;
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
        let params: web::Path<(usize,usize)> = web::Path::from((1,1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(),StatusCode::OK);
    }
}