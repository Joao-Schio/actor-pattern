pub mod mensageiro;
pub mod mensagem_manager;
pub mod sistema_handle;

pub struct Mensagem {
    conteudo : String,
    endereco : String,
    porta : u16
}

impl Mensagem {
    pub fn new(conteudo : String, endereco : String, porta : u16) -> Self {
        Self { conteudo, endereco, porta }
    }
}