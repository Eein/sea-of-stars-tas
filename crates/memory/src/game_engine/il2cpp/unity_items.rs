use crate::process::{Error, Process};

#[derive(Default, Debug)]
pub struct UnityItems<T: UnityItem> {
    pub count: u32,
    pub items: Vec<T>,
}

impl<T: UnityItem> UnityItems<T> {
    pub fn read(process: &Process, addr: u64) -> Result<Self, Error> {
        const OFFSET: u64 = 0x8;
        const ITEMS_0_INDEX_BASE: u64 = 0x20;

        let mut items = vec![];
        let mut fields_base = ITEMS_0_INDEX_BASE;

        let items_ptr = process.read_pointer::<u64>(addr + 0x10)?;
        let count = process.read_pointer::<u32>(items_ptr + 0x18)?;

        if count == 0 {
            return Err(Error);
        }

        for _index in 0..count {
            let relic_button = T::read(process, fields_base, items_ptr);
            if relic_button.is_ok() {
                items.push(relic_button?);
            }

            fields_base += OFFSET;
        }

        Ok(Self { count, items })
    }
}

/// Trait provided
pub trait UnityItem {
    fn read(process: &Process, fields_base: u64, items_ptr: u64) -> Result<Self, Error>
    where
        Self: Sized;
}
