use actix_web::{web,HttpResponse,post,put,get,delete};
use actix_session::Session;
use crate::state::AppState;
use crate::errors::MyError;
use crate::models::auth::{CreateUser, UpdateUser, Login};
use crate::dbaccess::auth::*;
use crate::utils::judge_auth;

#[post("/register")]
pub async fn post_new_user(
    app_state: web::Data<AppState>,
    create_user: web::Json<CreateUser>,
) -> Result<HttpResponse,MyError> {

    post_new_user_db(&app_state.db, create_user.into())
        .await
        .map(|user| {
            HttpResponse::Ok().json(user)
        })
}



#[put("/resetpwd")]
pub async fn update_user_pwd(
    app_state: web::Data<AppState>,
    session: Session,
    update_user: web::Json<UpdateUser>,
) -> Result<HttpResponse,MyError> {

    let user = judge_auth(&session, &app_state.auth_key).await?;

    if user.user_name != update_user.user_name {
        return Err(MyError::Unauthored("user error".into()))
    }

    let pwd = match &update_user.pwd {
        Some(pwd) => pwd,
        None => &user.pwd,
    };

    let phone = match &update_user.phone {
        Some(phone) => phone,
        None => &user.phone,
    };

    update_user_pwd_db(&app_state.db, &user.user_name,pwd,phone)
        .await
        .map(|user| {
            session.insert(&user.user_name, &user)
                .expect("session insert error");
            session.renew();
            HttpResponse::Ok().json(user)
        })

}


#[post("/login")]
pub async fn user_login(
    app_state: web::Data<AppState>,
    session: Session,
    login: web::Json<Login>
) -> Result<HttpResponse,MyError> {

    if let Ok(_s) = judge_auth(&session, &app_state.auth_key).await {
        return Err(MyError::Unauthored("dont login again".into()));
    }

    let user_name = match &login.user_name {
        Some(user_name) => user_name,
        None => return Err(MyError::InvalidInput("user_name is empty".into())),
    };

    let pwd = match &login.pwd {
        Some(pwd) => pwd,
        None => return Err(MyError::InvalidInput("pwd is empty".into())),
    };

    user_login_db(&app_state.db, user_name, pwd)
        .await
        .map(|user| {
            session.insert(&app_state.auth_key, &user)
                .expect("session insert error");
            HttpResponse::Ok().json(user)
        })
}


#[get("/logout")]
pub async fn user_logout(
    app_state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse,MyError> {
    let user = judge_auth(&session, &app_state.auth_key).await?;

    session.remove(&app_state.auth_key);
    Ok(HttpResponse::Ok().json(user))
}


#[delete("/user/{user_name}")]
pub async fn user_delete(
    app_state: web::Data<AppState>,
    session: Session,
    params: web::Path<(String,)>,
) -> Result<HttpResponse,MyError> {
    let user = judge_auth(&session, &app_state.auth_key).await?;

    let (user_name,) = params.into_inner();

    if user.user_name != user_name || user.user_name != "admin" {
        return Err(MyError::Unauthored("just admin can delete user".into()));
    }

    user_delete_db(&app_state.db, user.id)
        .await?;
    
    Ok(HttpResponse::Ok().json(user))
}