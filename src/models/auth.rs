use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Debug,Clone,sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub pwd: String,
    pub register_time: NaiveDateTime,
}



#[derive(Deserialize,Debug,Clone)]
pub struct CreateUser {
    pub user_name: String,
    pub pwd: String,
}


impl From<web::Json<CreateUser>> for CreateUser {
    fn from(user: web::Json<CreateUser>) -> Self {
        CreateUser {
            user_name: user.user_name.clone(),
            pwd: user.pwd.clone(),
        }
    }
}

#[derive(Deserialize,Debug,Clone)]
pub struct UpdateUser {
    pub pwd:Option<String>,
}

impl From<web::Json<UpdateUser>> for UpdateUser {
    fn from(user: web::Json<UpdateUser>) -> Self {
        UpdateUser {
            pwd: user.pwd.clone(),
        }
    }
}

#[derive(Deserialize,Debug,Clone)]
pub struct Login {
    pub user_name: Option<String>,
    pub pwd: Option<String>,
}


impl From<web::Json<Login>> for Login {
    fn from(login: web::Json<Login>) -> Self {
        Login {
            user_name: login.user_name.clone(),
            pwd: login.pwd.clone(),
        }
    }
}



