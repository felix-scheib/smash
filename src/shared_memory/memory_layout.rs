use std::sync::Arc;

use impl_macro::Implement;

use super::slot::Slot;

/// Define Memory here!
#[derive(Implement)]
pub struct MemoryLayout {
    first: Arc<Slot<i32>>,
    second: Arc<Slot<Vec<u8>>>,
    third: Arc<Slot<String>>,
}