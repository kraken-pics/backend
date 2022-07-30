use crate::entity::sea_orm_active_enums::Urltype;
use rand::seq::SliceRandom;

const ENGLISH_CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const HIRAGANA_CHARSET: &str = "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔ";

pub fn gen_file_mask(url_type: Urltype) -> String {
    match url_type {
        Urltype::Default => return gen_english_string(),
        Urltype::Invisible => return gen_invis_string(),
        Urltype::Emoji => return gen_english_string(),
        Urltype::Hiragana => return gen_hiragana_sting(),
        Urltype::Fake => return gen_english_string(),
    }
}

pub fn gen_english_string() -> String {
    random_string::generate(12, ENGLISH_CHARSET)
}

pub fn gen_hiragana_sting() -> String {
    random_string::generate(12, HIRAGANA_CHARSET)
}

// this is really awkward
pub fn gen_invis_string() -> String {
    let invisible_charset: Vec<&str> = vec!["\u{200B}", "\u{2060}", "\u{200C}", "\u{200D}"];
    let mut url = String::new();

    for _ in 0..12 {
        url.push_str(invisible_charset.choose(&mut rand::thread_rng()).unwrap());
    }

    url
}
