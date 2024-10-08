use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

use serde::{Deserialize, Serialize};
use tracing::{debug, span, trace, Level};

use super::{IncommingObserver, OutgoingObserver, SharedMemory};

mod on_drop_write_guard;

pub struct Slot<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    data: RwLock<T>,
    handle: usize,
    shared_memory: Weak<SharedMemory>,
}

impl<T> Slot<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(data: T, handle: usize, shared_memory: Weak<SharedMemory>) -> Self {
        Self {
            data: RwLock::new(data),
            handle,
            shared_memory,
        }
    }

    pub fn handle(&self) -> usize {
        self.handle
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.data.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write().unwrap()
    }

    pub fn update(&self) {
        // TODO:improve handling without the need of an explicit call
        let _span = span!(Level::DEBUG, "udapte").entered();
        debug!("Write event on Slot {:#x} occured!", self.handle);

        if let Some(shared_memory) = self.shared_memory.upgrade() {
            let payload = self.serialize();

            shared_memory.notify_write(self.handle, payload.clone());
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let data = { self.data.read().unwrap() };
        bincode::serialize(&*data).unwrap()
    }
}

impl<T> IncommingObserver for Slot<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn notify(&self, payload: Vec<u8>) {
        let _span = span!(Level::DEBUG, "notify").entered();
        debug!("Slot {:#x} received incomming message!", self.handle);

        let deserialized: Result<T, _> = bincode::deserialize(payload.as_slice());

        match deserialized {
            Ok(v) => {
                *self.data.write().unwrap() = v;
                trace!("Slot {:#x} value updated!", self.handle);
            }
            Err(e) => debug!("Failed to deserialize data: {:#?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let instance = Slot::new(42i32, 0x42, Weak::new());

        assert_eq!(instance.data.read().unwrap().to_owned(), 42i32);
        assert_eq!(instance.handle, 0x42);
    }

    #[test]
    fn test_read() {
        let instance = Slot::new(42i32, 0x42, Weak::new());

        let result = { *instance.read() };

        assert_eq!(result, 42i32);
    }

    #[test]
    fn test_write() {
        let instance = Slot::new(42i32, 0x42, Weak::new());

        {
            let mut data = instance.write();
            *data = 23;
        }

        let result = { *instance.read() };

        assert_eq!(result, 23i32);
    }
}
