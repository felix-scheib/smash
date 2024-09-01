use std::net::{SocketAddr, UdpSocket};

use tracing::trace;

pub struct Sender {
    sock: UdpSocket,
}

impl Sender {
    pub fn new(sock: UdpSocket) -> Self {
        Self { sock }
    }

    pub fn send(&self, msg: &[u8], dest: &SocketAddr) -> Result<usize, std::io::Error> {
        trace!("Sending UDP packet to: {:?}", dest);

        let res = self.sock.send_to(msg, dest);

        res
    }
}
