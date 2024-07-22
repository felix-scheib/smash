use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use super::{as_trait, slot::Slot, IncommingObserver, OutgoingObserver, SharedMemory};

/// Define Memory here!
pub struct MemoryLayout {
    first: Arc<Slot<i32>>,
    second: Arc<Slot<Vec<u8>>>,
    third: Arc<Slot<String>>,
}

impl MemoryLayout {
    // TODO: create functions using macros
    pub fn init() -> Self {
        Self {
            first: Arc::new(Slot::new(Default::default(), 0x01)),
            second: Arc::new(Slot::new(Vec::new(), 0x02)),
            third: Arc::new(Slot::new(String::new(), 0x03)),
        }
    }

    pub fn register(&self, shared_memory: &Arc<SharedMemory>) {
        self.first.register(Arc::downgrade(shared_memory));
        self.second.register(Arc::downgrade(shared_memory));
        self.third.register(Arc::downgrade(shared_memory));
    }

    pub fn as_map(&self) -> HashMap<usize, Arc<dyn IncommingObserver>> {
        HashMap::from([
            (self.first.handle(), as_trait(Arc::clone(&self.first))),
            (self.second.handle(), as_trait(Arc::clone(&self.second))),
            (self.third.handle(), as_trait(Arc::clone(&self.third))),
        ])
    }

    pub fn first(&self) -> Arc<Slot<i32>> {
        Arc::clone(&self.first)
    }

    pub fn second(&self) -> Arc<Slot<Vec<u8>>> {
        Arc::clone(&self.second)
    }

    pub fn third(&self) -> Arc<Slot<String>> {
        Arc::clone(&self.third)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback_map() {
        let instace = Memory {
            first: Arc::new(Slot::new(42, 0x42)),
        };

        for (k, v) in instace.callback_map() {
            assert_eq!(v(), k);
        }
    }
}
*/
