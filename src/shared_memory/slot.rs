use std::{
    borrow::Borrow,
    sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak},
};

use serde::{Deserialize, Serialize};

use super::{IncommingObserver, OutgoingObserver};

mod on_drop_write_guard;

type Callback = Box<dyn Fn(usize, Vec<u8>)>;
pub struct Slot<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    data: RwLock<T>,
    handle: usize,
    observers: Mutex<Vec<Weak<dyn OutgoingObserver>>>,
}

impl<T> Slot<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(data: T, handle: usize) -> Self {
        Self {
            data: RwLock::new(data),
            handle,
            observers: Mutex::new(Vec::new()),
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

    pub fn register(&self, observer: Weak<dyn OutgoingObserver>) {
        self.observers.lock().unwrap().push(observer)
    }

    pub fn update(&self) {
        // TODO: improve handling without the need of an explicit call
        println!("Update in Slot triggered!");
        let payload = self.serialize();

        for observer in self.observers.lock().unwrap().iter() {
            if let Some(observer) = observer.upgrade() {
                observer.notify(self.handle, payload.clone());
            }
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let data = { self.data.try_read().unwrap() };
        bincode::serialize(&*data).unwrap()
    }
}

impl<T> IncommingObserver for Slot<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn notify(&self, payload: Vec<u8>) {
        println!("{} received incomming message!", self.handle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let instance = Slot::new(42i32, 0x42);

        assert_eq!(instance.data.read().unwrap().to_owned(), 42i32);
        assert_eq!(instance.handle, 0x42);
    }

    #[test]
    fn test_read() {
        let instance = Slot::new(42i32, 0x42);

        let result = { *instance.read() };

        assert_eq!(result, 42i32);
    }

    #[test]
    fn test_write() {
        let instance = Slot::new(42i32, 0x42);

        {
            let mut data = instance.write();
            *data = 23;
        }

        let result = { *instance.read() };

        assert_eq!(result, 23i32);
    }
}
