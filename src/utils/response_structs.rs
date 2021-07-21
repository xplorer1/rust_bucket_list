use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralResponse {
    pub status: bool,
    pub message: String,
}