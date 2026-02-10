use axum::http::StatusCode;
use axum::{Json, extract::State};
use tokio::sync::mpsc;

use crate::mensagem::mensagem_manager::ManagerRequest;
use crate::mensagem::{mensagem_manager::ManagerResponse, sistema_handle::SistemaHandle};

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

        ManagerResponse::Erro { mensagem } => (
            StatusCode::BAD_REQUEST,
            Json(SendResponse {
                status: mensagem,
                id: None,
            }),
        ),

        _ => (
            StatusCode::BAD_REQUEST,
            Json(SendResponse {
                status: "requisicao invalida".to_string(),
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

    if handle
        .manager_tx
        .send(ManagerRequest::MandarMensagem {
            id: payload.id,
            conteudo: payload.conteudo,
            endereco: payload.endereco,
            porta: payload.porta,
            resposta: resp_tx,
        })
        .await
        .is_err()
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SendResponse {
                status: "falha ao comunicar com o manager".to_string(),
                id: None,
            }),
        );
    }

    let resp = resp_rx.recv().await;

    match resp {
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SendResponse {
                status: "Mensagem nao pode ser enviada".to_string(),
                id: None,
            }),
        ),
        Some(resp) => create_response(resp),
    }
}
