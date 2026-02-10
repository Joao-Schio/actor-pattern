use std::collections::HashMap;

use crate::mensagem::mensageiro::MandarMensagem;



pub enum ManagerRequest {
    MandarMensagem {id : usize, conteudo : String, endereco : String, porta : u16, resposta : tokio::sync::mpsc::Sender<ManagerResponse>},
    AdicionarMensageiro {id : usize, mensageiro : Box<dyn MandarMensagem>, resposta : tokio::sync::mpsc::Sender<ManagerResponse>},
}

pub enum ManagerResponse {
    MensageiroAdicionado {id : usize},
    MensagemEnviada { id : String}
}

pub struct MensagemManager {
    mensageiros : HashMap<usize, Box<dyn MandarMensagem>>,
    requisicoes : tokio::sync::mpsc::Receiver<ManagerRequest>
}

impl MensagemManager {
    pub fn new(requicioes : tokio::sync::mpsc::Receiver<ManagerRequest>) -> Self {
        Self {
            mensageiros : HashMap::new(),
            requisicoes : requicioes
        }
    }


    async fn adicionar_mensageiro(&mut self, id : usize, mensageiro : Box<dyn MandarMensagem>, resposta : tokio::sync::mpsc::Sender<ManagerResponse>) -> tokio::io::Result<()> {
        let mes = self.mensageiros.get(&id);
        let id_real = match mes {
            None => {
                self.mensageiros.insert(id, mensageiro);
                id
            },
            Some(_) => {
                let id_diferente = self.mensageiros.keys().max().unwrap().clone();
                self.mensageiros.insert(id_diferente, mensageiro);
                id_diferente
            }
        };

        let _ = resposta.send(
            ManagerResponse::MensageiroAdicionado { id : id_real }
        ).await;

        Ok(())
    }

    async fn mandar_mensagem(&self, id : usize, conteudo : String, endereco : String, porta : u16, resposta : tokio::sync::mpsc::Sender<ManagerResponse>) {
         let mes = self.mensageiros.get(&id);
         let id = guid_create::GUID::rand();

         match mes {
            None => {
                
            }
            Some(mensageiro) => {
                let _ = mensageiro.mandar_mensagem(
                    super::Mensagem { conteudo, endereco, porta }
                ).await;
            }
         };
         let _ = resposta.send(
            ManagerResponse::MensagemEnviada {id : id.to_string()}
         ).await;
    }

    async fn match_req(&mut self, req : ManagerRequest) -> tokio::io::Result<()> {
        match req {
            ManagerRequest::MandarMensagem { id, conteudo, endereco, porta, resposta } => {
                self.mandar_mensagem(id, conteudo, endereco, porta, resposta).await;
            },
            ManagerRequest::AdicionarMensageiro { id, mensageiro , resposta } => {
                let _ = self.adicionar_mensageiro(id, mensageiro, resposta).await;
            }
        }
        Ok(())
    }

    pub async fn start(&mut self) -> tokio::io::Result<()> {
        while let Some(req) = self.requisicoes.recv().await {
            self.match_req(req).await?;
        }
        Ok(())
    }
}
