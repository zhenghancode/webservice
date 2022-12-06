use crate::handlers::{course::*,general::*,teacher::*,auth::*};
use actix_web::{web, services};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    // cfg.route("/health",web::get().to(health_check_hanlder));
    cfg.service(services![health_check_hanlder]);
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/courses")
        .service(services![
            post_new_course,
            get_courses_for_teacher,
            get_course_detail,
            delete_course,
            update_course_details,
        ]),
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/teachers")
        .service(services![
            post_new_teacher,
            get_all_teachers,
            get_teacher_details,
            delete_teacher,
            update_teacher_details,
        ]),
    );
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/auth")
        .service(services![
            post_new_user,
            update_user_pwd,
            user_login,
            user_logout,
        ]),
    );  
}