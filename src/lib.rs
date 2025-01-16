mod client;
mod server;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "wsmux", about = "websocket based proxy")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Serve {
        #[arg(id = "addr", long, default_value = "0.0.0.0:8080")]
        addr: String,
    },

    Proxy {
        #[arg(id = "server", long)]
        server: String,

        #[arg(id = "local-addr", long, default_value = "localhost")]
        local_host: String,

        #[arg(id = "remote-addr", long, default_value = "localhost")]
        remote_host: String,

        #[arg(id = "local-port", long)]
        local_port: String,

        #[arg(id = "remote-port", long)]
        remote_port: String,
    },
}
