use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionPayload {
    pub patient_id: Option<String>,
    pub game_id: String,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub message: String,
}