#[derive(Debug)]
pub struct Slot {
    present: bool,
    item_id: Option<i32>,
    item_count: Option<u8>,
    nbt: u8, // todo: implement
}
