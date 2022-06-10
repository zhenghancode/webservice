use sqlx::postgres::PgPool;
use crate::models::teacher::{Teacher,CreateTeacher,UpdateTeacher};
use crate::errors::MyError;

pub async fn get_all_teachers_db(
    pool: &PgPool,
) -> Result<Vec<Teacher>,MyError> {
    let rows = sqlx::query!(
        r#"select * from teacher"#
    ).fetch_all(pool)
    .await?;

    match rows.len() {
        0 => Err(MyError::NotFound("teacher list not found".into())),
        _ => {
            let mut teachers = vec![];
            for row in rows {
                teachers.push(Teacher {
                    id: row.id,
                    name: row.name,
                    picture_url: row.picture_url,
                    profile: row.profile,
                });
            };
            Ok(teachers)
        },
    }
}

pub async fn get_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Teacher,MyError> {
    let row = sqlx::query!(
        r#"select * from teacher where id = $1"#,
        teacher_id,
    )
    .fetch_optional(pool)
    .await?;
    match row {
        Some(teacher) => {
            Ok(Teacher {
                id: teacher.id,
                name: teacher.name,
                picture_url: teacher.picture_url,
                profile: teacher.profile,
            })
        },
        None => Err(MyError::NotFound("teacher id not found".into()))
    }
    
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    create_teacher: CreateTeacher,
) -> Result<Teacher,MyError> {
    let row = sqlx::query!(
        r#"insert into teacher
        (name,picture_url,profile)
        values($1,$2,$3)
        RETURNING *"#,
        create_teacher.name,
        create_teacher.picture_url,
        create_teacher.profile,
    )
    .fetch_one(pool)
    .await?;

    Ok(Teacher {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile,
    })
}


pub async fn delete_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<String,MyError> {
    let result = sqlx::query!(
        r#"delete from teacher where id = $1"#,
        teacher_id,
    )
    .execute(pool)
    .await?;

    match result.rows_affected() {
        0 => Err(MyError::DBError("can not delete unexist teacher".into())),
        _ => Ok(format!("deleted! {:?}",result)),
    }
}

pub async fn update_teacher_details_db(
    pool: &PgPool,
    update_teacher: UpdateTeacher,
    teacher_id: i32,
) -> Result<Teacher,MyError> {
    let current_teacher_row = sqlx::query!(
        r#"select * from teacher
        where id = $1"#,
        teacher_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_| MyError::NotFound("teacher not found".into()))?;

    let update_teacher = UpdateTeacher {
        picture_url: match update_teacher.picture_url {
            Some(_) => update_teacher.picture_url,
            None => Some(current_teacher_row.picture_url),
        },
        profile: match update_teacher.profile {
            Some(_) => update_teacher.profile,
            None => Some(current_teacher_row.profile),
        },
    };
    let row = sqlx::query!(
        r#"update teacher set
        picture_url = $1,
        profile = $2 
        where id = $3
        RETURNING *"#,
        update_teacher.picture_url,
        update_teacher.profile,
        teacher_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(Teacher {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile,
    })
}




