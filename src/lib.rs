mod client;
mod server;
use clap::{Parser,Subcommand};

#[derive(Parser, Debug)]
#[command(
    name ="wsmux",
    about = "websocket based proxy",
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand,Debug)]
enum Command {
    Serve {

    }

    Proxy {

    }
}
