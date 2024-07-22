use crc::{Crc, CRC_32_CKSUM};

// TODO
const PREAMBLE: &str = "SMasH";
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

#[derive(Debug)]
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
        // TODO:
        None
    }
}

#[derive(Debug)]
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

    pub fn from_slice() -> Option<Self> {
        // TODO:
        None
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
}
