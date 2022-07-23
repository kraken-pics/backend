use actix_multipart_extract::{File, MultipartForm};
use serde::Deserialize;

#[derive(Deserialize, MultipartForm)]
pub struct UploadForm {
    #[multipart(max_size = 25MB)]
    pub file: File,
}
