use tokio::sync::mpsc;
use crate::mensagem::mensagem_manager::ManagerRequest;

#[derive(Clone)]
pub struct SistemaHandle {
    pub manager_tx: mpsc::Sender<ManagerRequest>,
}