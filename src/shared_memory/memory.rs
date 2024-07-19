use std::{collections::HashMap, sync::Arc};

use super::slot::Slot;

/// Define Memory here!
struct Memory {
    first: Arc<Slot<i32>>,
}

impl Memory {
    /*
    pub fn init() -> Self {
    }
    */

    // TODO: create HashMap with macros and reflection
    pub fn callback_map(&self) -> HashMap<usize, impl Fn() -> usize> {
        HashMap::from([(self.first.handle(), {
            let clone = Arc::clone(&self.first);
            move || clone.notify_chages()
        })])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callback_map() {
        let update_callback = Box::new(|_, _|{});
        let instace = Memory {
            first: Arc::new(Slot::new(42, 0x42, update_callback)),
        };

        for (k, v) in instace.callback_map() {
            assert_eq!(v(), k);
        }
    }
}
