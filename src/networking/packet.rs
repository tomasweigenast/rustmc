#[derive(Debug)]
pub struct Packet {
    length: i32,
    id: i32,
    data: Vec<u8>,
}
