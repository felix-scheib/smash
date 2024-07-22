use std::{
    ops::{Deref, DerefMut},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
};

use serde::{Deserialize, Serialize};

/*
struct OnDropWriteGuard<'a, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    guard: RwLockWriteGuard<'a, T>,
}

impl<'a, T> Drop for OnDropWriteGuard<'_, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn drop(&mut self) {
        println!("Dropped!");
    }
}

impl<'a, T> Deref for OnDropWriteGuard<'_, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> DerefMut for OnDropWriteGuard<'_, T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}
*/


type Callback = Box<dyn Fn(usize, Vec<u8>)>;
pub struct Slot<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    data: RwLock<T>,
    handle: usize,
    update_callback: Callback,
}

impl<T> Slot<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(data: T, handle: usize, update_callback: Callback) -> Self {
        Self {
            data: RwLock::new(data),
            handle,
            update_callback,
        }
    }

    pub fn handle(&self) -> usize {
        self.handle
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.data.read().unwrap()
    }

    pub fn write(&self) ->  RwLockWriteGuard<'_, T> {
        self.data.write().unwrap()
    }

    fn serialize(&self) -> Vec<u8> {
        let data = { self.data.try_read().unwrap() };
        bincode::serialize(&*data).unwrap()
    }

    pub fn update(&self) {
        // TODO: improve handling without the need of an explicit call
        println!("Update triggered!");

        (self.update_callback)(self.handle, self.serialize());
    }

    pub fn notify_chages(&self) -> usize {
        //TODO: implement update-logic for receiver
        self.handle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let update_callback = Box::new(|_, _| {});
        let instance = Slot::new(42i32, 0x42, update_callback);

        assert_eq!(instance.data.read().unwrap().to_owned(), 42i32);
        assert_eq!(instance.handle, 0x42);
    }

    #[test]
    fn test_read() {
        let update_callback = Box::new(|_, _| {});
        let instance = Slot::new(42i32, 0x42, update_callback);

        let result = { *instance.read() };

        assert_eq!(result, 42i32);
    }

    #[test]
    fn test_write() {
        let update_callback = Box::new(|_, _| {});
        let instance = Slot::new(42i32, 0x42, update_callback);

        {
            let mut data = instance.write();
            *data = 23;
        }

        let result = { *instance.read() };

        assert_eq!(result, 23i32);
    }

    #[test]
    fn test_update_callback() {
        let update_callback = Box::new(|h, v| {
            assert_eq!(h, 0x42);
            assert_eq!(v, bincode::serialize(&23i32).unwrap())
        });
        let instance = Slot::new(42i32, 0x42, update_callback);

        {
            let mut data = instance.write();
            *data = 23;
        }
        // TODO: this is workaround!!
        instance.update();
    }
}
