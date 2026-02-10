use async_trait::async_trait;
use tokio::{io::{AsyncWriteExt, Result}, net};

use crate::mensagem::Mensagem;



#[async_trait]
pub trait MandarMensagem: Send + Sync {
    async fn mandar_mensagem(&self, mensagem : Mensagem) -> Result<()>; 
}


pub struct Mensageiro { }


#[async_trait]
impl MandarMensagem for Mensageiro {
    async fn mandar_mensagem(&self, mensagem : Mensagem) -> Result<()> {
        let addr = format!("{}:{}", mensagem.endereco, mensagem.porta);
        let mut stream = net::TcpStream::connect(addr).await?;
        stream.write_all(mensagem.conteudo.as_bytes()).await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_send() { 
    let msg = Mensagem::new(
        "Ola, vindo do teste".to_string(),
        "localhost".to_string(),
        6969
    );

    let mensageiro = Mensageiro { };
    
    mensageiro.mandar_mensagem(msg).await.expect("Failed to send message");
}