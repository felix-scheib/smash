use std::{
    net::UdpSocket,
    sync::Weak,
    thread::{self, JoinHandle},
};

use tracing::trace;
use tracing_unwrap::ResultExt;

use crate::networking::package::Package;

use super::SharedMemory;

const BUFFER_SIZE: usize = 1_024;

pub struct Receiver {
    sock: UdpSocket,
    shared_memory: Weak<SharedMemory>,
}

impl Receiver {
    pub fn new(sock: UdpSocket, shared_memory: &Weak<SharedMemory>) -> Self {
        Self {
            sock,
            shared_memory: Weak::clone(shared_memory),
        }
    }

    pub fn receive(&self) -> JoinHandle<()> {
        let sock = self
            .sock
            .try_clone()
            .expect_or_log("Failed to clone Socket!");

        let shared_memory = Weak::clone(&self.shared_memory);

        thread::spawn(move || {
            let mut buf = [0x00; BUFFER_SIZE];

            loop {
                let (amount, source) = sock
                    .recv_from(&mut buf)
                    .expect("Failed to read from socket!");

                trace!("Received UDP-package from: {:?}", source);

                if let Some(package) = Package::from_slice(&buf[0..amount]) {
                    if let Some(shared_memory) = shared_memory.upgrade() {
                        shared_memory.notify_change(package);
                    }
                }

                buf.iter_mut().for_each(|x| *x = 0x00);
            }
        })
    }
}
