use std::{error::Error, sync::Arc};

use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};
use uuid::Uuid;

use crate::context::global::GlobalContext;

#[derive(Clone, Debug)]
pub struct Player {
    pub id: i32,
    uuid: Uuid,
    stream: Arc<Mutex<TcpStream>>,
    context: GlobalContext
}

impl Player {
    pub fn new(stream: TcpStream, context: GlobalContext) -> Self {
        Self {
            id: 0,
            uuid: Uuid::new_v4(),
            stream: Arc::new(Mutex::new(stream)),
            context: context
        }
    }

    pub async fn disconnected(&self) -> Result<(), Box<dyn Error>> {
        println!("Player {} got disconnected!", self.stream.lock().await.peer_addr()?);
        Ok(())
    }

    pub async fn handle_data(&self, buf: Vec<u8>) -> Result<(), Box<dyn Error>> {
        println!("Got {} bytes from {}:\n{}", buf.len(), self.stream.lock().await.peer_addr()?, String::from_utf8(buf).expect("failed to convert buffer into utf8 string"));
        Ok(())
    }

    pub async fn poll_data(&mut self) {
        let mut buf = vec![0; 1024];

        loop {
            let bytes_read = self.stream.lock().await
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");

            if bytes_read == 0 {
                let mut players = self.context.players.lock().await;

                let index = players.iter().position(|player| (*player).uuid == (*self).uuid).unwrap();
                players.remove(index);
                drop(players);
                break;
            }

            self.handle_data(buf.clone()).await.unwrap();
        }

        self.disconnected().await.unwrap();
    }
}