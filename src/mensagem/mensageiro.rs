use async_trait::async_trait;
use tokio::{
    io::{AsyncWriteExt, Result},
    net,
};

use crate::mensagem::Mensagem;

#[async_trait]
pub trait MandarMensagem: Send + Sync {
    async fn mandar_mensagem(&self, mensagem: Mensagem) -> Result<()>;
}

pub struct Mensageiro {}

#[async_trait]
impl MandarMensagem for Mensageiro {
    async fn mandar_mensagem(&self, mensagem: Mensagem) -> Result<()> {
        let addr = format!("{}:{}", mensagem.endereco, mensagem.porta);
        let mut stream = net::TcpStream::connect(addr).await?;
        stream.write_all(mensagem.conteudo.as_bytes()).await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_send() {
    use tokio::io::AsyncReadExt;

    let listener = net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind tcp listener");
    let porta = listener
        .local_addr()
        .expect("Failed to read local addr")
        .port();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let mut buf = vec![0u8; 64];
        let bytes = socket
            .read(&mut buf)
            .await
            .expect("Failed to read from socket");
        String::from_utf8(buf[..bytes].to_vec()).expect("invalid utf-8")
    });

    let conteudo = "Ola, vindo do teste".to_string();
    let msg = Mensagem::new(conteudo.clone(), "127.0.0.1".to_string(), porta);

    let mensageiro = Mensageiro {};

    mensageiro
        .mandar_mensagem(msg)
        .await
        .expect("Failed to send message");

    let recebido = server.await.expect("Failed to await server task");
    assert_eq!(recebido, conteudo);
}
