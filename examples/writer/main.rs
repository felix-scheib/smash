#[cfg(target_os = "hermit")]
use hermit as _;

use std::{env, net::UdpSocket, time::Instant};

use smash::{config::Config, shared_memory::SharedMemory};
use tracing::info;
use tracing_unwrap::ResultExt;

const SEND_IP: &str = "127.0.0.1:2301";
const RECV_IP: &str = "127.0.0.1:4201";

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let send = UdpSocket::bind(SEND_IP).unwrap_or_log();
    info!("Bind sender to: {:?}", send);
    let recv = UdpSocket::bind(RECV_IP).unwrap_or_log();
    info!("Bind receiver to: {:?}", recv);

    let sm = SharedMemory::new(&config.get_hosts(), send, recv);
    info!("Shared Memory initialized! Writer started");

    loop {
        let start = Instant::now();
        let first = sm.memory_layout.first();

        {
            let mut value = first.write();
            *value = 42;
        }

        first.update();
        let duration = start.elapsed();

        info!("Write operation took: {}us", duration.as_micros());

        let start = Instant::now();
        let first = sm.memory_layout.first();

        {
            let _value = first.read();
        }

        let duration = start.elapsed();

        info!("Read operation took: {}us", duration.as_micros());

        //thread::sleep(Duration::from_secs(1));
    }
}
