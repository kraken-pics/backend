use std::path::Path;

use actix_multipart_extract::Multipart;
use actix_web::{post, web, Error, Responder, Scope};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;

use crate::typings::{response::ApiResponse, upload::UploadForm};
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
    let file = &data.file;

    // TO-DO:
    // add file extension check
    // let extension = Path::new(&file.name)
    //     .extension()
    //     .and_then(OsStr::to_str)
    //     .unwrap();

    let digest = &format!("{:x}", Sha256::digest(&data.file.bytes));

    let format_path = format!(
        "{}{}/",
        upload_dir.to_string(),
        get_file_path(digest.to_string()).to_string()
    );
    let path = std::path::Path::new(&format_path);

    if !Path::new(&path).exists() {
        std::fs::create_dir_all(path).unwrap();
    }

    let mut cfile = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}{}", format_path, digest))
        .await?;
    cfile.write_all(&file.bytes).await?;

    Ok(actix_web::web::Json(ApiResponse {
        success: true,
        message: "Successfully uploaded file".to_string(),
    }))
}
