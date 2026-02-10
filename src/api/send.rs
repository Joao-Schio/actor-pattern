use axum::http::StatusCode;
use axum::{extract::State, Json};
use tokio::sync::mpsc;

use crate::mensagem::{mensagem_manager::ManagerResponse, sistema_handle::SistemaHandle};
use crate::mensagem::mensagem_manager::ManagerRequest;

use super::{SendRequest, SendResponse};


#[inline(always)]
pub fn create_response(resp: ManagerResponse) -> (StatusCode, Json<SendResponse>) {
    match resp {
        ManagerResponse::MensagemEnviada { id } => (
            StatusCode::OK,
            Json(SendResponse {
                status: "mensagem enviada".to_string(),
                id: Some(id),
            }),
        ),

        _ => (
            StatusCode::UNAUTHORIZED,
            Json(SendResponse {
                status: "Mensagem nao pode ser enviada".to_string(),
                id: None,
            }),
        ),
    }
}


pub async fn send_handler(
    State(handle): State<SistemaHandle>,
    Json(payload): Json<SendRequest>,
) -> (StatusCode, Json<SendResponse>) {

    let (resp_tx, mut resp_rx) = mpsc::channel(1);

    handle.manager_tx.send(
        ManagerRequest::MandarMensagem {
            id: payload.id,
            conteudo: payload.conteudo,
            endereco: payload.endereco,
            porta: payload.porta,
            resposta: resp_tx,
        }
    ).await.unwrap();

    let resp = resp_rx.recv().await;

    let out = match resp {
        None => {
            (
                StatusCode::UNAUTHORIZED,
                Json(
                    SendResponse {
                        status : "Mensagem nao pode ser enviada".to_string(),
                        id : None
                    }
                )
            )
        },
        Some(resp) => {
            create_response(resp)
        }
    };

    out
}