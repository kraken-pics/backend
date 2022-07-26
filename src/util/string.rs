const NORMAL_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

pub fn gen_image_mask() -> std::string::String {
    return random_string::generate(12, NORMAL_CHARSET);
}
