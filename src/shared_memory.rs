use receiver::Receiver;
use sender::Sender;
use std::{
    net::{SocketAddr, UdpSocket},
    thread::{self, JoinHandle},
    time::Duration,
};
use tracing::{error, trace};
use tracing_unwrap::ResultExt;

mod memory;
mod receiver;
mod sender;
mod slot;

pub struct SharedMemory {
    hosts: Vec<SocketAddr>,
    sender: Sender,
    receiver: Receiver,
}

impl SharedMemory {
    pub fn new(hosts: &Vec<String>, send: UdpSocket, recv: UdpSocket) -> Self {
        Self {
            hosts: hosts
                .iter()
                .map(|h| match h.parse() {
                    Ok(h) => Some(h),
                    Err(e) => {
                        error!("Failed to parse IP address: {}", e);
                        None
                    }
                })
                .filter_map(|h| h)
                .collect(),
            sender: Sender::new(send),
            receiver: Receiver::new(recv),
        }
    }

    pub fn receive(&self) -> JoinHandle<()> {
        self.receiver.receive()
    }

    pub fn test_sending(&self) {
        loop {
            for addr in &self.hosts {
                self.sender
                    .send("Hello from sender!".as_bytes(), addr.clone())
                    .expect_or_log("Failed to send message!");
                trace!("UDP-package sent!");
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}
