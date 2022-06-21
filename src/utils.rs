use rand::Rng;


lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = rand_string();
}


fn rand_string() -> String {
    const CHARSET:&[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890";

    const SECRET_KEY_LEN:usize =32;
    let mut rng = rand::thread_rng();

    (0..SECRET_KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
    
}