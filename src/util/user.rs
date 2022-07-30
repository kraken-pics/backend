pub fn check_username(username: String) -> Result<(), &'static str> {
    match username.len() {
        0..=2 => Err("Username too short"),
        50.. => Err("Username too long"),

        _ => Ok({}),
    }
}

pub fn check_password(password: String) -> Result<(), &'static str> {
    match password.len() {
        0..=4 => Err("Password too short"),
        50.. => Err("Password too long"),

        _ => Ok({}),
    }
}
