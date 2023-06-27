use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorMessage {
    pub(crate) message: String,
}
