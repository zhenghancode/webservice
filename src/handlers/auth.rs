use actix_web::{web,HttpResponse,post,put,get};
use actix_identity::Identity;
use crate::state::AppState;
use crate::errors::MyError;
use crate::models::auth::{CreateUser, UpdateUser, Login, User,};
use crate::dbaccess::auth::*;


#[post("/register")]
pub async fn post_new_user(
    app_state: web::Data<AppState>,
    id: Identity,
    create_user: web::Json<CreateUser>,
) -> Result<HttpResponse,MyError> {

    post_new_user_db(&app_state.db, create_user.into())
        .await
        .map(|user| {
            id.remember(
                serde_json::to_string(&user).expect("json serialize error")
            );
            HttpResponse::Ok().json(user)
        })
}



#[put("/resetpwd")]
pub async fn update_user_pwd(
    app_state: web::Data<AppState>,
    id: Identity,
    update_user: web::Json<UpdateUser>,
) -> Result<HttpResponse,MyError> {

    let user:User = match id.identity() {
        Some(s) => serde_json::from_str(&s)
            .map_err(|err| MyError::InvalidInput(err.to_string()))?,
        None => return Err(MyError::Unauthored("not user login".into())),
    };

    let pwd = match &update_user.pwd {
        Some(pwd) => pwd,
        None => &user.pwd,
    };

    update_user_pwd_db(&app_state.db, &user.user_name,pwd)
        .await
        .map(|user| {
            id.forget();
            id.remember(serde_json::to_string(&user).expect("json serialize error"));
            HttpResponse::Ok().json(user)
        })

}


#[post("/login")]
pub async fn user_login(
    app_state: web::Data<AppState>,
    id: Identity,
    login: web::Json<Login>
) -> Result<HttpResponse,MyError> {

    if let Some(_s) = id.identity()  {
        return Err(MyError::InvalidInput("please dont login again".into()));
    };

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
            id.remember(serde_json::to_string(&user).expect("json serialize error"));
            HttpResponse::Ok().json(user)
        })
}


#[get("/logout")]
pub async fn user_logout(
    app_state: web::Data<AppState>,
    id: Identity,
) -> Result<HttpResponse,MyError> {
    let user:User = match id.identity() {
        Some(s) => serde_json::from_str(&s).map_err(|err| MyError::InvalidInput(err.to_string()))?,
        None => return Err(MyError::Unauthored("not logining".into())),
    };

    user_logout_db(&app_state.db, user.id)
        .await
        .map(|msg| {
            id.forget();
            HttpResponse::Ok().json(msg)
        })
}