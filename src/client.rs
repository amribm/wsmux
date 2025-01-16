use fastwebsockets::{WebSocket};
use reqwest;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub struct Client {
    local_addr: String,
    remote_addr: String,
    via_addr: String,
}

impl Client {
    pub fn new(local_addr: String, remote_addr: String, via_addr: String) -> Client {
        Client {
            local_addr,
            remote_addr,
            via_addr,
        }
    }

    /// proxies the request from local addr to websocket server
    pub async fn listen_and_serve(&self, err_rx: mpsc::Receiver<WsmuxError>) {
        let mut server_stream = TcpStream::connect(self.via_addr).await?;
    }
}
