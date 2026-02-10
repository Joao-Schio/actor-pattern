pub mod send;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SendRequest {
    pub id: usize,
    pub conteudo: String,
    pub endereco: String,
    pub porta: u16,
}

#[derive(Serialize)]
pub struct SendResponse {
    pub status: String,
    pub id : Option<String>
}