use crate::process::{Error, Process};

#[derive(Default, Debug)]
pub struct UnityItems<T: UnityItem> {
    pub count: u32,
    pub items: Vec<T>,
}

const ITEMS_OFFSET: u64 = 0x10;
const OFFSET: u64 = 0x8;
const ITEMS_0_INDEX_BASE: u64 = 0x20;
const COUNT_OFFSET: u64 = 0x18;

///  UnityItems are laid out in the following format:
///  ```
///  field -> ptr
///    0x10 items -> ptr
///      0x08 count -> u32
///      0x20 item[0] -> ptr
///      0x28 item[1] -> ptr
///      0x30 item[x] -> ptr
///  ```

impl<T: UnityItem> UnityItems<T> {
    pub fn read(process: &Process, addr: u64) -> Result<Self, Error> {
        let mut items = vec![];
        let mut fields_base = ITEMS_0_INDEX_BASE;

        // The root items pointer
        let items_ptr = process.read_pointer::<u64>(addr + ITEMS_OFFSET)?;

        // The count in the items pointer
        let count = process.read_pointer::<u32>(items_ptr + COUNT_OFFSET)?;

        // ensure neither of these is 0, or else just dummy the struct
        // this is for performance and memory safety
        if items_ptr == 0 {
            return Ok(Self { count: 0, items });
        }

        if count == 0 {
            return Ok(Self { count: 0, items });
        }

        for _index in 0..count {
            // the item in the items pointer by offset
            let item_ptr = process.read_pointer::<u64>(items_ptr + fields_base)?;

            if item_ptr != 0 {
                let item = T::read(process, item_ptr);
                if item.is_ok() {
                    items.push(item?);
                }
            }

            fields_base += OFFSET;
        }

        Ok(Self { count, items })
    }
}

/// Trait provided
pub trait UnityItem {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, Error>
    where
        Self: Sized;
}
