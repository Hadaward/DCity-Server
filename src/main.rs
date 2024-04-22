use std::error::Error;

use context::global::GlobalContext;
use player::Player;
use tokio::net::TcpListener;

pub mod player;
pub mod context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Dead City server listening on: {}", listener.local_addr()?);

    let global_ctx = GlobalContext::new();

    tokio::spawn(accept_connections(listener, global_ctx.clone()));

    loop {
        update(global_ctx.clone()).await;
    }
}

async fn accept_connections(listener: TcpListener, global_ctx: GlobalContext) {
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let global_ctx = global_ctx.clone();
        
        tokio::spawn(async move {
            let global_ctx = global_ctx.clone();
            let mut player = Player::new(stream, global_ctx.clone());
            let mut players = global_ctx.players.lock().await;

            players.push(player.clone());
            drop(players);

            player.poll_data().await;
        });
    }
}

static mut LAST_PLAYER_COUNT: usize = 0;

async fn update(global_ctx: GlobalContext) {
    let players = global_ctx.players.lock().await;

    if players.len() != unsafe { LAST_PLAYER_COUNT } {
        println!("Há {} jogadores conectados!", players.len());
        unsafe { LAST_PLAYER_COUNT = players.len() };
    }

    drop(players);
}