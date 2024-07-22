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

mod memory_layout;
mod receiver;
mod sender;
mod slot;

trait OutgoingObserver {
    fn notify(&self, handle: usize, payload: Vec<u8>);
}

trait IncommingObserver {
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
        let sm = Arc::new(Self {
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
            memory_layout: MemoryLayout::init(),
        });

        sm.memory_layout.register(&sm);

        sm
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

impl OutgoingObserver for SharedMemory {
    fn notify(&self, handle: usize, payload: Vec<u8>) {
        println!("Received incomming messagefrom {}!", handle);
    }
}
