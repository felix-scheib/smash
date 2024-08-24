use memory_layout::MemoryLayout;
use receiver::Receiver;
use sender::Sender;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};
use tracing::{error, trace};
use tracing_unwrap::ResultExt;

use crate::networking::packet::Packet;

mod memory_layout;
mod receiver;
mod sender;
mod slot;

trait OutgoingObserver {
    fn notify_write(&self, handle: usize, payload: Vec<u8>);
}

pub trait IncommingObserver {
    fn notify(&self, payload: Vec<u8>);
}

fn as_trait<T: IncommingObserver + 'static>(arc: Arc<T>) -> Arc<dyn IncommingObserver> {
    arc
}

pub struct SharedMemory {
    hosts: Vec<SocketAddr>,
    sender: Sender,
    receiver: Receiver,
    pub memory_layout: MemoryLayout,
}

impl SharedMemory {
    pub fn new(hosts: &Vec<String>, send: UdpSocket, recv: UdpSocket) -> Arc<Self> {
        Arc::new_cyclic(|weak| Self {
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
            receiver: Receiver::new(recv, weak),
            memory_layout: MemoryLayout::init(weak),
        })
    }

    pub fn receive(&self) -> JoinHandle<()> {
        self.receiver.receive()
    }

    pub fn test_sending(&self) {
        loop {
            for addr in &self.hosts {
                self.sender
                    .send("Hello from sender!".as_bytes(), addr)
                    .expect_or_log("Failed to send message!");
                trace!("UDP packet sent!");
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn notify_change(&self, packet: Packet) {
        trace!("Change notification received!");

        let handle = packet.header.handle;

        if let Some(v) = self.memory_layout.get_slot(handle) {
            v.notify(packet.payload);
        } else {
            trace!("Handle {:#x} not found!", packet.header.handle);
        }
    }
}

impl OutgoingObserver for SharedMemory {
    fn notify_write(&self, handle: usize, payload: Vec<u8>) {
        trace!("Received incomming message from {:#x}!", handle);

        let packet = Packet::new(handle, payload);

        for host in &self.hosts {
            let _ = self.sender.send(&packet.to_vec(), host);
        }
    }
}
