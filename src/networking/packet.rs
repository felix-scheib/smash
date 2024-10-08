use std::mem::size_of;

use crc::{Crc, CRC_32_CKSUM};
use serde::{Deserialize, Serialize};
use tracing::trace;

const PREAMBLE: &str = "SMasH";
const TYPE_SIZE: usize = 12;
const HEADER_SIZE: usize = 25 + TYPE_SIZE;
//const MTU: usize = 1_500;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Type {
    SINGLE(usize),
}

#[derive(Debug, PartialEq)]
pub struct Header {
    preamble: &'static str,
    packet_type: Type,
    pub handle: usize,
    checksum: u32,
    size: usize,
}

impl Header {
    pub fn new(handle: usize, checksum: u32, size: usize) -> Self {
        Self {
            preamble: PREAMBLE,
            packet_type: Type::SINGLE(0x00),
            handle,
            checksum,
            size,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let preamble = self.preamble.as_bytes();
        let packet_type = bincode::serialize(&self.packet_type).unwrap();
        let handle = self.handle.to_be_bytes();
        let checksum = self.checksum.to_be_bytes();
        let size = self.size.to_be_bytes();

        let mut vec = Vec::with_capacity(
            preamble.len() + TYPE_SIZE + handle.len() + checksum.len() + size.len(),
        );
        vec.extend_from_slice(preamble);
        vec.extend_from_slice(&packet_type);
        vec.extend_from_slice(&handle);
        vec.extend_from_slice(&checksum);
        vec.extend_from_slice(&size);

        vec
    }

    pub fn from_slice(data: &[u8]) -> Option<Self> {
        if data.len() < HEADER_SIZE {
            trace!("Invalid header lenght: {}", data.len());
            return None;
        }

        let preamble = &*String::from_utf8_lossy(&data[0..PREAMBLE.len()]);

        if preamble != PREAMBLE {
            trace!("Wrong format!");
            return None;
        }

        let start = PREAMBLE.len();
        let end = start + TYPE_SIZE;
        let packet_type = bincode::deserialize(&data[start..end]).unwrap();

        let start = end;
        let end = start + size_of::<usize>();
        let handle = usize::from_be_bytes(data[start..end].try_into().unwrap());

        let start = end;
        let end = start + size_of::<u32>();
        let checksum = u32::from_be_bytes(data[start..end].try_into().unwrap());

        let start = end;
        let end = start + size_of::<usize>();
        let size = usize::from_be_bytes(data[start..end].try_into().unwrap());

        let header = Self {
            preamble: PREAMBLE,
            packet_type,
            handle,
            checksum,
            size,
        };

        trace!("Header accepted: {:?}", header);

        Some(header)
    }
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    pub header: Header,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(handle: usize, payload: Vec<u8>) -> Self {
        let size = payload.len();
        let checksum = checksum(&payload[0..size]);

        Self {
            header: Header::new(handle, checksum, size),
            payload: payload,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let header = self.header.to_vec();

        let mut vec = Vec::with_capacity(header.len() + self.payload.len());
        vec.extend(&header);
        vec.extend(&self.payload);

        vec
    }

    pub fn from_slice(data: &[u8]) -> Option<Self> {
        let header = Header::from_slice(data);
        if header == None {
            return None;
        }
        let header = header.unwrap();

        let start = HEADER_SIZE;
        let end = start + header.size;

        let content = &data[start..end];

        if content.len() != end - start {
            trace!("Invalid content lenght: {}", content.len());
            return None;
        }

        let checksum = checksum(content);

        if header.checksum != checksum {
            trace!(
                "Invalid checksum! expected: {:#x}, actual: {:#x}",
                header.checksum,
                checksum
            );
            return None;
        }

        let payload = content.to_vec();

        trace!("Packet for Slot {:#x} parsed!", header.handle);
        Some(Self { header, payload })
    }
}

fn checksum(data: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&CRC_32_CKSUM);
    let mut digest = crc.digest();

    digest.update(data);
    digest.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_to_vec() {
        let packet_type = Type::SINGLE(0x00);
        let handle = 0x42;
        let cksum = 0x23;
        let size = 0x00;

        let header = Header {
            preamble: PREAMBLE,
            packet_type: packet_type.clone(),
            handle,
            checksum: cksum,
            size,
        };

        let vec = header.to_vec();

        let start = 0;
        let stop = start + PREAMBLE.len();
        assert_eq!(&vec[start..stop], PREAMBLE.as_bytes());

        let packet_type = bincode::serialize(&packet_type).unwrap();
        let start = stop;
        let stop = start + TYPE_SIZE;
        assert_eq!(&vec[start..stop], packet_type);

        let handle = handle.to_be_bytes();
        let start = stop;
        let stop = start + handle.len();
        assert_eq!(&vec[start..stop], handle);

        let cksum = cksum.to_be_bytes();
        let start = stop;
        let stop = start + cksum.len();
        assert_eq!(&vec[start..stop], cksum);

        let size = size.to_be_bytes();
        let start = stop;
        let stop = start + size.len();
        assert_eq!(&vec[start..stop], size);
    }

    #[test]
    fn test_header_from_slice() {
        // Header length
        let slice = "".as_bytes();

        assert_eq!(Header::from_slice(slice), None);

        let header = Header {
            preamble: PREAMBLE,
            packet_type: Type::SINGLE(0x00),
            handle: Default::default(),
            checksum: Default::default(),
            size: Default::default(),
        };
        let vec = header.to_vec();

        assert_eq!(Header::from_slice(vec.as_slice()), Some(header));

        // Preamble
        let header = Header {
            preamble: "",
            packet_type: Type::SINGLE(0x00),
            handle: Default::default(),
            checksum: Default::default(),
            size: Default::default(),
        };
        let vec = header.to_vec();

        assert_eq!(Header::from_slice(vec.as_slice()), None);

        let header = Header {
            preamble: PREAMBLE,
            packet_type: Type::SINGLE(0x00),
            handle: Default::default(),
            checksum: Default::default(),
            size: Default::default(),
        };
        let vec = header.to_vec();

        assert_eq!(Header::from_slice(vec.as_slice()), Some(header));

        // Fields
        let header = Header {
            preamble: PREAMBLE,
            packet_type: Type::SINGLE(0x00),
            handle: 0x42,
            checksum: 0x23,
            size: 0x00,
        };
        let vec = header.to_vec();

        assert_eq!(Header::from_slice(vec.as_slice()), Some(header));
    }

    #[test]
    fn test_packets_from_slice() {
        // Header length
        let data = "".as_bytes();

        assert_eq!(Packet::from_slice(data), None);

        let packet = Packet::new(0x00, data.to_vec());

        let vec = packet.to_vec();

        assert_eq!(Packet::from_slice(vec.as_slice()), Some(packet));
    }
}
