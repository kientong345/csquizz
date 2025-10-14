pub fn secret_key() -> Vec<u8> {
    std::env::var("SECRET_KEY").expect("SECRET_KEY is not set");
    todo!()
}
