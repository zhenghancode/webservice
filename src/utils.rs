use actix_session::Session;
use actix_web::cookie::Key;
// use rand::Rng;

use crate::models::auth::User;
use crate::errors::MyError;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: Key = Key::generate();
}


/* fn rand_string() -> String {
    const CHARSET:&[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890";

    const SECRET_KEY_LEN:usize =64;
    let mut rng = rand::thread_rng();

    (0..SECRET_KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
    
} */


pub async fn judge_auth(session: &Session,auth_key: &str) -> Result<User,MyError> {
    match session.get::<User>(auth_key)
        .map_err(|error| MyError::InvalidInput(error.to_string()))?
    {
        Some(user) => Ok(user),
        None => Err(MyError::Unauthored("not user login".into())),
    }
}