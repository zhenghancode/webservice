use sqlx::postgres::PgPool;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use crate::errors::MyError;

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>,MyError> {
    let courses = sqlx::query_as!(
        Course,
        r#"select * from course where teacher_id = $1"#,
        teacher_id,
    ).fetch_all(pool)
    .await?;

    match courses.len() {
        0 => Err(MyError::NotFound("Courses not found for teacher".into())),
        _ => Ok(courses),
    }
}

pub async fn get_course_detail_db(pool: &PgPool,teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"select * from course where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id,
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(course) => Ok(course),
        None => Err(MyError::NotFound("not found this course".into())),
    }
}

pub async fn post_new_course_db(pool: &PgPool,new_course: CreateCourse) -> Result<Course,MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"insert into course 
    (teacher_id,name,description,format,structure,duration,price,language,level)
    values($1,$2,$3,$4,$5,$6,$7,$8,$9)
    RETURNING *"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,
    )
    .fetch_one(pool)
    .await?;
     
    Ok(row)
}

pub async fn delete_course_db(pool: &PgPool,teacher_id: i32,id: i32) 
    -> Result<String,MyError>
{
    let result = sqlx::query!(
        r#"delete from course 
        where teacher_id = $1 
        and id = $2"#,
        teacher_id,
        id,
    )
    .execute(pool)
    .await?;
    match result.rows_affected() {
        0 => Err(MyError::NotFound("no such course to delete".into())),
        _ => Ok(format!("Deleted {:?} record",result)),
    }
}


pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course,MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        r#"select * from course
        where teacher_id = $1
        and id = $2"#,
        teacher_id,
        id,
    ).fetch_one(pool)
    .await
    .map_err(|_| MyError::NotFound("course id not found".into()))?;

    let name = match update_course.name {
        Some(name) => name,
        None => current_course_row.name,
    };
    let description = match update_course.description {
        Some(description) => description,
        None => current_course_row.description.unwrap_or_default(),
    };
    let format = match update_course.format {
        Some(format) => format,
        None => current_course_row.format.unwrap_or_default(),
    };
    let structure = match update_course.structure {
        Some(structure) => structure,
        None => current_course_row.structure.unwrap_or_default(),
    };
    let duration = match update_course.duration {
        Some(duration) => duration,
        None => current_course_row.duration.unwrap_or_default(),
    };
    let price = match update_course.price {
        Some(price) => price,
        None => current_course_row.price.unwrap_or_default(),
    };
    let language = match update_course.language {
        Some(language) => language,
        None => current_course_row.language.unwrap_or_default(),
    };
    let level = match update_course.level {
        Some(level) => level,
        None => current_course_row.level.unwrap_or_default(),
    };
    let course = sqlx::query_as!(
        Course,
        r#"update course set
        name = $1, description = $2,
        format = $3, structure = $4,
        duration = $5, price = $6,
        language = $7, level = $8 
        where teacher_id = $9 and id = $10 
        RETURNING *"#,
        name,description,
        format,structure,
        duration,price,
        language,level,
        teacher_id,id,
    ).fetch_one(pool)
    .await?;


    Ok(course)

}