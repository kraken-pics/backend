use serde::Deserialize;

#[derive(Deserialize)]
pub struct ILogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct IRegister {
    pub username: String,
    pub email: String,
    pub password: String,
    // pub recaptcha: String,
}
