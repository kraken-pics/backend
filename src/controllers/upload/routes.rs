use std::{
    ffi::OsStr,
    fmt::format,
    fs,
    os::unix::prelude::FileExt,
    path::{Path, PathBuf},
};

use actix_multipart_extract::{File, Multipart, MultipartForm};
use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Responder, Scope};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

use crate::typings::response::ApiResponse;
pub fn get() -> Scope {
    web::scope("/upload").service(upload_file)
}

fn get_file_path(digest: String) -> String {
    let digest_vec: Vec<char> = digest.chars().collect();
    format!("/{}/{}", digest_vec[0], digest_vec[1])
}

#[derive(Deserialize, MultipartForm)]
pub struct ExampleForm {
    #[multipart(max_size = 25MB)]
    file: File,
}

#[post("")]
async fn upload_file(data: Multipart<ExampleForm>) -> Result<impl Responder, Error> {
    let file = &data.file;

    let extension = Path::new(&file.name)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    let digest = &format!("{:x}", Sha256::digest(&data.file.bytes));

    let upload_dir = "/home/ian/Documents/code/kraken-pics/backend/uploads";
    let upload_path = format!(
        "{}{}/",
        upload_dir.to_string(),
        get_file_path(digest.to_string()).to_string()
    );
    let path = std::path::Path::new(&upload_path);

    if !Path::new(&path).exists() {
        std::fs::create_dir_all(path).unwrap();
    }

    let mut cfile = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}{}", upload_path, digest))
        .await?;
    cfile.write_all(&file.bytes).await?;

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "cum".to_string(),
    }))
}
