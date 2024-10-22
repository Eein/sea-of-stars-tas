use crate::process::{MemoryError, Process};
use indexmap::IndexMap;
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct UnitySerializableDictionary<K: UnitySerializableDictKey, V: UnitySerializableDictValue> {
    pub count: u32,
    pub items: IndexMap<K, V>,
}

const ENTRIES_OFFSET: u64 = 0x18;
const ITEMS_0_INDEX_BASE: u64 = 0x20;
const DICTIONARY_OFFSET: u64 = 0x20;

///  UnitySerializableDictionary are laid out in the following format:
///
///  Example of [packed struct header, key, value]
///  In this case: slot[]<key, value>
///  ITEM_OFFSET: u64 = 0x18;
///  KEY_OFFSET: u64 = 0x8;
///  VALUE_OFFSET: u64 = 0x10;
///  ```
///  field -> ptr
///    0x10 keys (unused)
///    0x18 values (unused)
///    0x20 dictionary -> ptr
///      0x10 _buckets (unused)
///      0x18 _entries
///        0x20 metadata -> (unused)
///        0x28 key -> ptr
///        0x30 value -> ptr
///  ```
///
///  Example of packed struct [4byte Enum, 4byte float]
///  ITEM_OFFSET: u64 = 0x10;
///  KEY_OFFSET: u64 = 0x8;
///  VALUE_OFFSET: u64 = 0x10;
///  ```
///  field -> ptr
///    0x10 keys (unused)
///    0x18 values (unused)
///    0x20 dictionary -> ptr
///      0x10 _buckets (unused)
///      0x18 _entries
///        0x20 metadata -> (unused)
///        0x28 key -> ptr
///        0x29 value -> ptr
///  ```
///
///  Note:
///  Implementors will receive the pointer value, and must follow the pointer if desired.

impl<K: UnitySerializableDictKey + Eq + Hash, V: UnitySerializableDictValue>
    UnitySerializableDictionary<K, V>
{
    pub fn read(
        process: &Process,
        addr: u64,
        item_offset: u64,
        key_offset: u64,
        value_offset: u64,
    ) -> Result<Self, MemoryError> {
        let mut items = IndexMap::<K, V>::new();
        let mut fields_base = ITEMS_0_INDEX_BASE;

        // If the root address is 0x0, then we dont have any items.
        if addr == 0x0 {
            return Ok(Self { count: 0, items });
        }
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

                if let Ok(key_ptr) =
                    process.read_pointer_path_without_read(item_base + key_offset, &[0x0])
                {
                    let key = K::read(process, key_ptr)?;
                    if let Ok(value_ptr) =
                        process.read_pointer_path_without_read(item_base + value_offset, &[0x0])
                    {
                        let value = V::read(process, value_ptr)?;
                        items.insert(key, value);
                    }
                }
            }

            fields_base += item_offset;
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
