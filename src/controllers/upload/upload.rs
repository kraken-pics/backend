use std::path::Path;

use actix_multipart_extract::Multipart;
use actix_web::{post, web, Responder};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

use crate::{
    entity::{config as ConfigTable, upload as UploadTable, user as UserTable},
    state::AppState,
    typings::{
        response::{ApiResponse, ErrorResponse},
        upload::UploadForm,
    },
    util::string::gen_file_mask,
};

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

// global AppState
type AppData = web::Data<AppState>;

// create file path from digest
fn create_digest_path(digest: String) -> String {
    let digest_vec: Vec<char> = digest.chars().collect();

    let first_char = digest_vec[0].to_string();
    let second_char = digest_vec[1].to_string();

    concat_string!(first_char, "/", second_char, "/")
}

#[post("")]
async fn upload(
    data: Multipart<UploadForm>,
    state: AppData,
) -> Result<impl Responder, ErrorResponse> {
    let upload_dir = dotenv::var("UPLOAD_DIR").expect("UPLOAD_DIR envar");

    // find user by their upload token, hopefully, specified in the multipart
    let found_user = match UserTable::Entity::find()
        .filter(UserTable::Column::Uploadtoken.eq(data.upload_key.clone()))
        .one(&state.db)
        .await
        .expect("User not found")
    {
        Some(val) => val,
        None => {
            return Err(ErrorResponse {
                message: "Not authorized".to_string(),
            });
        }
    };

    let found_config = match ConfigTable::Entity::find()
        .filter(ConfigTable::Column::Userid.eq(found_user.id))
        .one(&state.db)
        .await
        .expect("Config not found")
    {
        Some(val) => val,
        None => {
            return Err(ErrorResponse {
                message: "Not authorized".to_string(),
            });
        }
    };

    if data.file.bytes.len() == 0 {
        return Err(ErrorResponse {
            message: "File cannot be empty".to_string(),
        });
    }

    // get file digest, as this can be used to prevent spam uploads wasting storage
    // though, each time; it does create a db value aka a mask just pointing toward the same file
    let digest = format!("{:x}", Sha256::digest(&data.file.bytes));

    // get the first two characters of the digest and make a directory using it
    let digest_dir = create_digest_path(digest.clone());

    // adds upload_dir to the start of digest_dir
    let file_dir = concat_string!(upload_dir, digest_dir);

    // if the uploaded files path doesn't exist, create it
    if !Path::new(&file_dir).exists() {
        if let Err(_) = tokio::fs::create_dir_all(&file_dir).await {
            return Err(ErrorResponse {
                message: "Internal error occurred, try again later".to_string(),
            });
        };
    }

    // add the digest to the end of file dir
    let file_dir = concat_string!(&file_dir, digest.clone());

    // open a write stream to the dir & file to create them
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .truncate(true)
        .open(&file_dir)
        .await
        .unwrap();

    if let Err(_) = file.write_all(&data.file.bytes).await {
        return Err(ErrorResponse {
            message: "Internal error occurred, try again later write".to_string(),
        });
    };

    let file_mask = gen_file_mask(found_config.urltype);

    // attempt to add upload to db
    if let Err(err) = (UploadTable::ActiveModel {
        userid: Set(found_user.id),
        filemask: Set(file_mask.escape_unicode().to_string()),
        mimetype: Set(data.file.content_type.to_string()),
        hash: Set(digest),
        size: Set(data.file.bytes.len() as i32),
        ..Default::default()
    }
    .insert(&state.db)
    .await)
    {
        return Err(ErrorResponse {
            message: err.to_string(),
        });
    }

    // success!
    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: file_mask.to_string(),
    }))
}
