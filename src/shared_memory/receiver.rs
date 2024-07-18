use std::{
    net::UdpSocket,
    thread::{self, JoinHandle},
};

use crc::{Crc, CRC_32_CKSUM};
use tracing::trace;
use tracing_unwrap::ResultExt;

const BUFFER_SIZE: usize = 1_024;

pub struct Receiver {
    sock: UdpSocket,
}

impl Receiver {
    pub fn new(sock: UdpSocket) -> Self {
        Self { sock }
    }

    pub fn receive(&self) -> JoinHandle<()> {
        let sock = self
            .sock
            .try_clone()
            .expect_or_log("Failed to clone Socket!");
        thread::spawn(move || {
            let mut buf = [0x00; BUFFER_SIZE];
            let crc = Crc::<u32>::new(&CRC_32_CKSUM);

            loop {
                let mut digest = crc.digest();
                let (amount, source) = sock
                    .recv_from(&mut buf)
                    .expect("Failed to read from socket!");

                digest.update(&buf[0..amount]);

                trace!("Received UDP-package from: {:?} with checksum: {}", source, digest.finalize());

                buf.iter_mut().for_each(|x| *x = 0x00);
            }
        })
    }
}
