// TODO
const IDENTIFIER: &str = "SMasH";
const MTU: usize = 1_500;

#[derive(Debug)]
pub enum Kind {
    SINGLE,
    SEGMENT,
    NACK,
    REPEAT,
}

#[derive(Debug)]
pub struct Package {
    kind: Kind,
    checksum: u32,
    payload: Vec<u8>,
}

/*
impl Package {
    pub fn to_package() -> Vec<Self> {


    }

}
*/
