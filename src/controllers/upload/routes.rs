use std::path::Path;

use actix_multipart_extract::Multipart;
use actix_web::{post, web, Error, Responder, Scope};
use sha2::{Digest, Sha256};
use std::fmt::Write;
use tokio::io::AsyncWriteExt;

use crate::{
    entity::user,
    state::AppState,
    typings::{response::ApiResponse, upload::UploadForm},
};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

// global AppState
type AppData = web::Data<AppState>;

pub fn get() -> Scope {
    web::scope("/upload").service(upload_file)
}

fn get_file_path(digest: String) -> String {
    let digest_vec: Vec<char> = digest.chars().collect();
    format!("/{}/{}", digest_vec[0], digest_vec[1])
}

#[post("")]
async fn upload_file(data: Multipart<UploadForm>) -> Result<impl Responder, Error> {
    let upload_dir = dotenv::var("UPLOAD_DIR").expect("UPLOAD_DIR envar");

    // get file digest, as
    // this can be used to prevent spam uploads wasting storage
    // though, each time; it does create a db value aka a mask just pointing toward the same file
    let digest_create = Sha256::digest(&data.file.bytes);
    let mut digest = String::with_capacity(digest_create.len());
    if let Err(_) = write!(digest, "{:?}", digest_create) {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Internal error occurred, try again later".to_string(),
        }));
    };
    let digest_path = get_file_path(digest.to_string()).to_string();

    // get the first two characters of the file digest
    // to split files into directories & files, again:
    // to prevent spam uploads
    let mut hash_dir = String::with_capacity(1 + upload_dir.len() + digest_path.len());
    if let Err(_) = write!(hash_dir, "{}{}/", upload_dir, digest_path) {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Internal error occurred, try again later".to_string(),
        }));
    };

    // create a std path, used to create the path
    let file_dir = std::path::Path::new(&hash_dir);

    // if the uploaded files path doesn't exist, create it
    // tokios's fs is fast as fuck
    if !Path::new(&file_dir).exists() {
        if let Err(_) = tokio::fs::create_dir_all(file_dir).await {
            return Ok(actix_web::web::Json(ApiResponse {
                success: false,
                message: "Internal error occurred, try again later".to_string(),
            }));
        };
    }

    let mut file_path = String::with_capacity(1 + hash_dir.len() + digest.len());
    if let Err(_) = write!(file_path, "{}{}", hash_dir, digest) {
        return Ok(actix_web::web::Json(ApiResponse {
            success: false,
            message: "Internal error occurred, try again later".to_string(),
        }));
    };

    // if the file exists, theres no point in re-writing it
    if std::path::Path::new(&file_path).exists() {
        return Ok(actix_web::web::Json(ApiResponse {
            success: true,
            message: "Successfully uploaded file".to_string(),
        }));
    }
    // open a write stream to the dir & file to create them
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .truncate(true)
        .open(file_path)
        .await?;

    // write the file from bytes
    file.write_all(&data.file.bytes).await?;

    // success!
    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully uploaded file".to_string(),
    }))
}
