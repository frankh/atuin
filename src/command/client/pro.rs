
use std::future::Future;
use std::net::SocketAddr;



use clap::Parser;

use eyre::{Result};
use tokio::sync::mpsc;

use atuin_client::{
    settings::{Settings},
};

#[derive(Parser)]
#[clap(infer_subcommands = true)]
pub struct Cmd {
}

use warp::Filter;

async fn server() -> Result<(SocketAddr, impl Future<Output = ()> + 'static)> {
    let (tx, mut rx) = mpsc::channel(100);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("success")
        .map(move || {
            let resp = tx.try_send(());
            if resp.is_err() {
                panic!("Failed to send signal to channel listening for pro payment");
            }
            "Subscription successful! You can close this tab and start using Atiun Pro!".to_string()
        });

    let (addr, server) = warp::serve(hello)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 0), async move {
             rx.recv().await;

        });

    Ok((addr, server))
}

impl Cmd {
    pub async fn run(&self, settings: &Settings) -> Result<()> {
        let session_path = atuin_common::utils::data_dir().join("session");

        if !session_path.exists() {
            println!(
                "You must be logged in to upgrade to Atuin Pro! Please run 'atuin login' first"
            );

            return Ok(());
        }

        let (addr, server) = server().await.unwrap();

        let upgrade_resp = atuin_client::pro::upgrade(settings, addr.port()).await?;
        println!("Thanks for upgrading! A payment page should open in your browser");
        print!("Waiting for payment to complete...");
        open::that(upgrade_resp.checkout_url).unwrap();
        server.await;
        println!("Done!");
        println!();
        println!("Atuin Pro Activated! 🐢");
        Ok(())
    }
}
