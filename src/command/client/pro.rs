use std::future::Future;
use std::net::SocketAddr;

use clap::Parser;

use eyre::{Result};
use tokio::sync::mpsc;

use atuin_client::{
    settings::{Settings},
};

use atuin_common::utils::uuid_v4;


#[derive(Parser)]
#[clap(infer_subcommands = true)]
pub enum Cmd {
    Upgrade,
}

use warp::Filter;

async fn server() -> Result<(SocketAddr, impl Future<Output = ()> + 'static)> {
    let (tx, mut rx) = mpsc::channel(100);
    let tx2 = tx.clone();

    let success = warp::path!("success")
        .map(move || {
            let resp = tx.try_send(());
            if resp.is_err() {
                panic!("Failed to send signal to channel listening for pro payment");
            }
            let success_html_bytes = include_bytes!("static/success.html");
            warp::reply::html(format!("{}", String::from_utf8_lossy(success_html_bytes)))
        });

    let cancel = warp::path!("cancel")
        .map(move || {
            let resp = tx2.try_send(());
            if resp.is_err() {
                panic!("Failed to send signal to channel listening for pro payment");
            }
            let cancel_html_bytes = include_bytes!("static/cancel.html");
            warp::reply::html(format!("{}", String::from_utf8_lossy(cancel_html_bytes)))
        });

    let (addr, server) = warp::serve(warp::get().and(success.or(cancel)))
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 0), async move {
             rx.recv().await;
        });

    Ok((addr, server))
}

impl Cmd {
    pub async fn upgrade(settings: &Settings) -> Result<()> {
        let session_path = atuin_common::utils::data_dir().join("session");

        if !session_path.exists() {
            println!(
                "You must be logged in to upgrade to Atuin Pro! Please run 'atuin login' first"
            );

            return Ok(());
        }

        let (addr, server) = server().await.unwrap();

        let uuid = format!("{}", uuid_v4());
        let upgrade_resp = atuin_client::pro::upgrade(settings, uuid, addr.port()).await?;
        println!("Thanks for upgrading! A payment page should open in your browser");
        print!("Waiting for payment to complete...");
        open::that(upgrade_resp.checkout_url).unwrap();
        server.await;
        println!("Done!");
        println!();
        println!("Atuin Pro Activated! 🐢");
        Ok(())
    }

    pub async fn run(&self, settings: &Settings) -> Result<()> {
        match self {
            Self::Upgrade => Self::upgrade(settings).await
        }
    }
}
