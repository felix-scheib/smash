use std::sync::{Arc, Weak};

use impl_macro::Implement;

use super::{as_trait, slot::Slot, IncommingObserver, SharedMemory};

/// Define Memory here!
#[derive(Implement)]
pub struct MemoryLayout {
    first: Arc<Slot<i32>>,
    second: Arc<Slot<Vec<u8>>>,
    third: Arc<Slot<String>>,
}

impl MemoryLayout {
    // TODO: create functions using macros
    pub fn init(shared_memory: &Weak<SharedMemory>) -> Self {
        Self {
            first: Arc::new(Slot::new(
                Default::default(),
                0x01,
                Weak::clone(&shared_memory),
            )),
            second: Arc::new(Slot::new(Vec::new(), 0x02, Weak::clone(&shared_memory))),
            third: Arc::new(Slot::new(String::new(), 0x03, Weak::clone(&shared_memory))),
        }
    }

    pub fn get_slot(&self, handle: usize) -> Option<Arc<dyn IncommingObserver>> {
        match handle {
            0x01 => Some(as_trait(Arc::clone(&self.first))),
            0x02 => Some(as_trait(Arc::clone(&self.second))),
            0x03 => Some(as_trait(Arc::clone(&self.third))),
            _ => None,
        }
    }

    /*
        pub fn first(&self) -> Arc<Slot<i32>> {
        Arc::clone(&self.first)
    }

    pub fn second(&self) -> Arc<Slot<Vec<u8>>> {
        Arc::clone(&self.second)
    }

    pub fn third(&self) -> Arc<Slot<String>> {
        Arc::clone(&self.third)
    }
    */
}
