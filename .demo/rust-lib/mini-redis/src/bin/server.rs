//! mini-redis server.
//!
//! This file is the entry point for the server implemented in the library. It
//! performs command line parsing and passes the arguments on to
//! `mini_redis::server`.
//!
//! The `clap` crate is used for parsing arguments.

use mini_redis::{server, DEFAULT_PORT};

use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::signal;
use std::io;
use tracing::{debug, instrument};

#[tokio::main]
pub async fn main() -> mini_redis::Result<()> {
    // enable logging
    // see https://docs.rs/tracing for more info
    tracing_subscriber::fmt::try_init()?;

    let cli = Cli::from_args();
    let port = cli.port.as_deref().unwrap_or(DEFAULT_PORT);

    // Bind a TCP listener
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;

    server::run(listener, my_exit()).await
}

#[instrument]
async fn my_exit() -> io::Result<()> {
    use std::time::{Duration, Instant};

    const TIMEOUT: Duration = Duration::from_secs(3);
    let mut is_press: Option<bool> = None;
    loop {
        let now = Instant::now();
        signal::ctrl_c().await?;
        match is_press {
            Some(_) if now.elapsed() > TIMEOUT  => {
                debug!("press once more to exit");
            }
            None => {
                debug!("press once more to exit");
                is_press = Some(true);
            }
            _ => {
                debug!("now exit");
                break;
            }
        }
    }

    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "mini-redis-server", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = "A Redis server")]
struct Cli {
    #[structopt(name = "port", long = "--port")]
    port: Option<String>,
}
