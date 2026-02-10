use axum::{Router, routing::post};
use tokio::sync::mpsc;

use show::mensagem::mensageiro::Mensageiro;
use show::mensagem::mensagem_manager::{MensagemManager, ManagerRequest};
use show::mensagem::sistema_handle::SistemaHandle;
use show::api::send::send_handler;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    let mut manager = MensagemManager::new(rx);

    tokio::spawn(async move {
        manager.start().await.unwrap();
    });

    register_default_mensageiro(tx.clone()).await;

    let app = Router::new()
        .route("/api/send", post(send_handler))
        .with_state(SistemaHandle { manager_tx: tx });

    println!("Server running on 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn register_default_mensageiro(tx: tokio::sync::mpsc::Sender<ManagerRequest>) {
    use tokio::sync::mpsc;

    let (resp_tx, mut resp_rx) = mpsc::channel(1);

    tx.send(ManagerRequest::AdicionarMensageiro {
        id: 1,
        mensageiro: Box::new(Mensageiro {}),
        resposta: resp_tx,
    }).await.unwrap();

    let _ = resp_rx.recv().await;
}