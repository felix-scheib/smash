use std::{
    ops::{Deref, DerefMut},
    sync::RwLockWriteGuard,
};

use serde::{Deserialize, Serialize};

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
