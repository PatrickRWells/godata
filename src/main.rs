mod project;
mod fsystem;
mod storage;
mod locations;
mod server;
mod routes;
mod handlers;

use clap::Parser;
// Allow the server to return its version with a --version flag
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    version: bool,
    #[clap(short, long)]
    debug: bool,
    #[clap(short, long)]
    port: Option<u16>
}

#[cfg(target_family = "windows")]
#[tokio::main]
async fn main () {
    let opts: Opts = Opts::parse();
    if opts.version {
        println!("{}", VERSION);
        return;
    }
    if opts.port.is_none() {
        println!("A port is required to start the server");
        return;
    }
    let srv = server::get_server(opts.port);
    srv.start().await;
}

#[cfg(target_family = "unix")]
#[tokio::main]
async fn main () {
    let opts: Opts = Opts::parse();
    if opts.version {
        println!("{}", VERSION);
        return;
    }
    let srv = server::get_server(opts.port);
    srv.start().await;
}




