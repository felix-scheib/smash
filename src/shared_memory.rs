use memory::{Bytes, Memory, Text};
use std::net::SocketAddr;
use tracing::error;

mod receiver;
mod sender;

pub mod memory {
    include!(concat!(env!("OUT_DIR"), "/protobuf.memory.rs"));
}
struct SharedMemory {
    hosts: Vec<SocketAddr>,
    memory: memory::Memory,
}

impl SharedMemory {
    pub fn new(hosts: &Vec<String>) -> Self {
        SharedMemory {
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
            memory: Self::init_memory(),
        }
    }

    fn init_memory() -> Memory {
        Memory {
            bytes: Some(Bytes {
                payload: Vec::new(),
            }),
            text: Some(Text {
                data: String::new(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use memory::Memory;
    use prost::Message;

    use super::*;

    #[test]
    fn test_new_host_parsing() {
        let hosts = vec![
            "127.0.0.1:4242".to_owned(),
            "127.0.0.1:2323".to_owned(),
            "unknown".to_owned(),
        ];
        let result = SharedMemory::new(&hosts);

        let expected: Vec<SocketAddr> = vec![
            "127.0.0.1:4242".parse().unwrap(),
            "127.0.0.1:2323".parse().unwrap(),
        ];

        assert_eq!(result.hosts, expected);
    }

    #[test]
    fn test_new_proto_memory() {
        let hosts = vec![];
        let mut result = SharedMemory::new(&hosts);

        result.memory.bytes = Some(Bytes {
            payload: vec![4, 2, 2, 3],
        });
        result.memory.text = Some(Text {
            data: "hello from Rust!".to_owned(),
        });

        let mut serialized = Vec::new();
        serialized.reserve(result.memory.bytes.as_ref().unwrap().encoded_len());

        let _ = result.memory.bytes.unwrap().encode(&mut serialized);

        let mut serialized = Vec::new();
        serialized.reserve(result.memory.text.as_ref().unwrap().encoded_len());

        let _ = result.memory.text.unwrap().encode(&mut serialized);
    }
}
