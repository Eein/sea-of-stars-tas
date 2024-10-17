use crate::process::{MemoryError, Process};
use indexmap::IndexMap;
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct UnitySerializableDictionary<K: UnitySerializableDictKey, V: UnitySerializableDictValue> {
    pub count: u32,
    pub items: IndexMap<K, V>,
}

const ENTRIES_OFFSET: u64 = 0x18;
const OFFSET: u64 = 0x18;
const ITEMS_0_INDEX_BASE: u64 = 0x20; // Skipping first metadata
const KEY_OFFSET: u64 = 0x8;
const VALUE_OFFSET: u64 = 0x10;
const DICTIONARY_OFFSET: u64 = 0x20;

///  UnitySerializableDictionary are laid out in the following format:
///  ```
///  field -> ptr
///    0x10 keys (unused)
///    0x18 values (unused)
///    0x20 dictionary -> ptr
///      0x10 _buckets (unused)
///      0x18 _entries
///        ... header/metadata
///        0x20 metadata -> (unused)
///        0x28 key -> ptr
///          0x14 first byte of key if is struct
///        0x30 value -> ptr
///  ```
///

impl<K: UnitySerializableDictKey + Eq + Hash, V: UnitySerializableDictValue>
    UnitySerializableDictionary<K, V>
{
    pub fn read(process: &Process, addr: u64) -> Result<Self, MemoryError> {
        let mut items = IndexMap::<K, V>::new();
        let mut fields_base = ITEMS_0_INDEX_BASE;

        // The root items pointer
        let items_ptr =
            process.read_pointer_path::<u64>(addr, &[DICTIONARY_OFFSET, ENTRIES_OFFSET])?;

        // ensure neither of these is 0, or else just dummy the struct
        // this is for performance and memory safety
        if items_ptr == 0 {
            return Ok(Self { count: 0, items });
        }

        loop {
            let item_base = items_ptr + fields_base;
            if let Ok(metadata_pointer) = process.read_pointer::<u64>(item_base) {
                // If the loop encounters a null pointer just break.
                // This is UB but the best we have for now since we dont have a real count

                if metadata_pointer == 0x0 {
                    break;
                }

                if let Ok(key_ptr) = process.read_pointer::<u64>(item_base + KEY_OFFSET) {
                    let key = K::read(process, key_ptr)?;
                    if let Ok(value_ptr) = process.read_pointer::<u64>(item_base + VALUE_OFFSET) {
                        let value = V::read(process, value_ptr)?;
                        items.insert(key, value);
                    }
                }
            }

            fields_base += OFFSET;
        }

        Ok(Self {
            count: items.len() as u32,
            items,
        })
    }
}

/// Trait provided
pub trait UnitySerializableDictKey {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError>
    where
        Self: Sized;
}

pub trait UnitySerializableDictValue {
    fn read(process: &Process, item_ptr: u64) -> Result<Self, MemoryError>
    where
        Self: Sized;
}
