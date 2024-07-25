use std::mem::size_of;

use crc::{Crc, CRC_32_CKSUM};

// TODO
const PREAMBLE: &str = "SMasH";
const HEADER_SIZE: usize = 25;
//const MTU: usize = 1_500;

/*
#[derive(Debug)]
pub enum Kind {
    SINGLE,
    SEGMENT,
    NACK,
    REPEAT,
}
*/

#[derive(Debug, PartialEq)]
struct Header {
    preamble: &'static str,
    handle: usize,
    checksum: u32,
    size: usize,
}

impl Header {
    pub fn new(handle: usize, checksum: u32, size: usize) -> Self {
        Self {
            preamble: PREAMBLE,
            handle,
            checksum,
            size,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let preamble = self.preamble.as_bytes();
        let handle = self.handle.to_be_bytes();
        let checksum = self.checksum.to_be_bytes();
        let size = self.size.to_be_bytes();

        let mut vec =
            Vec::with_capacity(preamble.len() + handle.len() + checksum.len() + size.len());
        vec.extend_from_slice(preamble);
        vec.extend_from_slice(&handle);
        vec.extend_from_slice(&checksum);
        vec.extend_from_slice(&size);

        vec
    }

    pub fn from_slice(data: &[u8]) -> Option<Self> {
        if data.len() < HEADER_SIZE {
            return None;
        }

        let preamble = &*String::from_utf8_lossy(&data[0..PREAMBLE.len()]);

        if preamble != PREAMBLE {
            return None;
        }

        let start = PREAMBLE.len();
        let end = start + size_of::<usize>();
        let handle = usize::from_be_bytes(data[start..end].try_into().unwrap());

        let start = end;
        let end = start + size_of::<u32>();
        let checksum = u32::from_be_bytes(data[start..end].try_into().unwrap());

        let start = end;
        let end = start + size_of::<usize>();
        let size = usize::from_be_bytes(data[start..end].try_into().unwrap());

        Some(Self {
            preamble: PREAMBLE,
            handle,
            checksum,
            size,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Package {
    header: Header,
    payload: Vec<u8>,
}

impl Package {
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
            return None;
        }

        let payload = content.to_vec();

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
        let handle = 0x42;
        let cksum = 0x23;
        let size = 0x00;

        let header = Header {
            preamble: PREAMBLE,
            handle,
            checksum: cksum,
            size,
        };

        let vec = header.to_vec();

        let start = 0;
        let stop = start + PREAMBLE.len();
        assert_eq!(&vec[start..stop], PREAMBLE.as_bytes());

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
            handle: Default::default(),
            checksum: Default::default(),
            size: Default::default(),
        };
        let vec = header.to_vec();

        assert_eq!(Header::from_slice(vec.as_slice()), Some(header));

        // Preamble
        let header = Header {
            preamble: "",
            handle: Default::default(),
            checksum: Default::default(),
            size: Default::default(),
        };
        let vec = header.to_vec();

        assert_eq!(Header::from_slice(vec.as_slice()), None);

        let header = Header {
            preamble: PREAMBLE,
            handle: Default::default(),
            checksum: Default::default(),
            size: Default::default(),
        };
        let vec = header.to_vec();

        assert_eq!(Header::from_slice(vec.as_slice()), Some(header));

        // Fields
        let header = Header {
            preamble: PREAMBLE,
            handle: 0x42,
            checksum: 0x23,
            size: 0x00,
        };
        let vec = header.to_vec();

        assert_eq!(Header::from_slice(vec.as_slice()), Some(header));
    }

    #[test]
    fn test_package_from_slice() {
        // Header length
        let data = "".as_bytes();

        assert_eq!(Package::from_slice(data), None);

        let package = Package {
            header: Header {
                preamble: PREAMBLE,
                handle: Default::default(),
                checksum: Default::default(),
                size: 0x00,
            },
            payload: data.to_vec(),
        };

        let vec = package.to_vec();

        assert_eq!(Package::from_slice(vec.as_slice()), Some(package));
    }
}
