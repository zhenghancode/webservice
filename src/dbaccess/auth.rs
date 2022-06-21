use sqlx::postgres::PgPool;
use crate::errors::MyError;
use crate::models::auth::{CreateUser,User};


pub async fn post_new_user_db(
    pool: &PgPool,
    new_user: CreateUser,
) -> Result<User,MyError> {
    let row = sqlx::query_as!(
        User,
        r#"insert into "user"
    (user_name,pwd)
        values($1,$2)
        RETURNING *"#,
        new_user.user_name,
        new_user.pwd,
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_user_pwd_db(
    pool: &PgPool,
    user_name: &str,
    pwd: &str,
) -> Result<User,MyError> {

    let user = sqlx::query_as!(
        User,
        r#"update "user" set
        pwd=$1 
        where user_name=$2
        RETURNING *"#,
        pwd,
        user_name,
    ).fetch_one(pool)
    .await?;

    Ok(user)
}



pub async fn user_login_db(
    pool: &PgPool,
    user_name: &str,
    pwd: &str,
) -> Result<User,MyError> {

    let row = sqlx::query_as!(
        User,
        r#"select * from "user" 
        where user_name=$1 
        and 
        pwd=$2"#,
        user_name,
        pwd,
    ).fetch_optional(pool)
    .await?;

    match row {
        Some(user) => Ok(user),
        None => Err(MyError::NotFound("no such user found".into())),
    }
}


pub async fn user_logout_db(
    pool: &PgPool,
    user_id: i32,
) -> Result<String,MyError> {

    let row = sqlx::query_as!(
        User,
        r#"delete from "user" 
        where id =$1"#,
        user_id,
    )
    .execute(pool)
    .await?;

    match row.rows_affected() {
        0 => Err(MyError::DBError("DELETE NOTHING".into())),
        _ => Ok("logout success!".to_string()), 
    }
}


