use bytemuck::{CheckedBitPattern, Pod};
use memory::game_engine::il2cpp::{Class, Module};
use memory::memory_manager::il2cpp::UnityMemoryManager;
use memory::process::{MemoryError, Process};
use memory::string::ArrayWString;

// TODO(eein): is it possible to make this more generic so it can be
// moved into memory crate?
use crate::state::StateContext;

pub struct MemoryContext<'a> {
    pub class: &'a Class,
    pub process: &'a Process,
    pub module: &'a Module,
    pub singleton: &'a Class,
}

impl<'a> MemoryContext<'a> {
    pub fn create(
        ctx: &'a StateContext,
        manager: &'a mut UnityMemoryManager,
    ) -> Result<MemoryContext<'a>, MemoryError> {
        if let Some(class) = &manager.class
            && let Some(process) = &ctx.process
            && let Some(module) = &ctx.module
            && let Some(singleton) = &manager.singleton
        {
            return Ok(Self {
                class,
                process,
                module,
                singleton,
            });
        }

        Err(MemoryError::Unset)
    }

    pub fn follow_fields<T: Pod>(&self, fields: &[&str]) -> Result<T, MemoryError> {
        self.class
            .follow_fields::<T>(*self.singleton, self.process, self.module, fields)
    }

    pub fn read_pointer_path<T: CheckedBitPattern + Pod>(
        &self,
        path: &[u64],
    ) -> Result<T, MemoryError> {
        self.process
            .read_pointer_path::<T>(self.singleton.class, path)
    }

    pub fn read_pointer<T: Pod>(&self, addr: u64) -> Result<T, MemoryError> {
        self.process.read_pointer::<T>(addr)
    }

    pub fn get_field_offset(&self, field: &str) -> Option<u32> {
        self.class
            .get_field_offset(self.process, self.module, field)
    }

    /// Resolve a named field's offset on an *arbitrary* il2cpp object by reading
    /// its class from the object header. Unlike hardcoded offsets, this survives
    /// game patches that shift struct layouts.
    ///
    /// Falls back to the C# auto-property backing-field name
    /// (`<field>k__BackingField`) when the plain name doesn't resolve, since many
    /// of these fields are exposed as properties.
    pub fn field_offset_of(&self, obj_ptr: u64, field: &str) -> Option<u32> {
        if obj_ptr == 0 {
            return None;
        }
        let class_ptr = self.process.read_pointer::<u64>(obj_ptr).ok()?;
        let class = Class { class: class_ptr };
        class
            .get_field_offset(self.process, self.module, field)
            .or_else(|| {
                let backing = format!("<{field}>k__BackingField");
                class.get_field_offset(self.process, self.module, &backing)
            })
    }

    /// Read a pointer-typed field by name from an arbitrary il2cpp object,
    /// returning `None` on a failed read or a null pointer.
    pub fn read_named_ptr(&self, obj_ptr: u64, field: &str) -> Option<u64> {
        let offset = self.field_offset_of(obj_ptr, field)?;
        self.process
            .read_pointer::<u64>(obj_ptr + offset as u64)
            .ok()
            .filter(|p| *p != 0)
    }

    /// Read a `Pod` value field (e.g. an `i32`/`f32`) by name from an arbitrary
    /// il2cpp object.
    pub fn read_named<T: Pod>(&self, obj_ptr: u64, field: &str) -> Option<T> {
        let offset = self.field_offset_of(obj_ptr, field)?;
        self.process.read_pointer::<T>(obj_ptr + offset as u64).ok()
    }

    /// Read the element pointers of a `HashSet<T>` of reference-typed elements
    /// (the `System.Collections.Generic.HashSet` layout: `_slots` at `+0x18`,
    /// `_lastIndex` at `+0x24`; each `Slot` is `{ i32 hashCode; i32 next; T value }`
    /// = 16 bytes, live when `hashCode >= 0`, with the element pointer at
    /// `slot+0x8`). The `List<T>` analogue is [`list_item_ptrs`](Self::list_item_ptrs).
    pub fn hashset_item_ptrs(&self, set_ptr: u64) -> Vec<u64> {
        const SLOTS_OFFSET: u64 = 0x18;
        const LAST_INDEX_OFFSET: u64 = 0x24;
        const SLOTS_ELEMS_BASE: u64 = 0x20;
        const SLOT_STRIDE: u64 = 0x10;
        const SLOT_VALUE_OFFSET: u64 = 0x8;
        const MAX_ITEMS: i32 = 128;

        let mut out = Vec::new();
        if set_ptr == 0 {
            return out;
        }
        let Ok(slots) = self.process.read_pointer::<u64>(set_ptr + SLOTS_OFFSET) else {
            return out;
        };
        let Ok(last_index) = self
            .process
            .read_pointer::<i32>(set_ptr + LAST_INDEX_OFFSET)
        else {
            return out;
        };
        if slots == 0 {
            return out;
        }
        for i in 0..last_index.clamp(0, MAX_ITEMS) {
            let slot = slots + SLOTS_ELEMS_BASE + i as u64 * SLOT_STRIDE;
            // Free slots have hashCode < 0; skip them.
            if self.process.read_pointer::<i32>(slot).unwrap_or(-1) < 0 {
                continue;
            }
            if let Ok(value) = self.process.read_pointer::<u64>(slot + SLOT_VALUE_OFFSET)
                && value != 0
            {
                out.push(value);
            }
        }
        out
    }

    /// Read a C# `System.String` object's contents (UTF-16 chars at `+0x14`).
    /// `None` on a failed read or an empty string.
    pub fn read_csharp_string(&self, str_obj: u64) -> Option<String> {
        let chars = self
            .process
            .read_pointer::<ArrayWString<64>>(str_obj + 0x14)
            .ok()?;
        let out = String::from_utf16(chars.as_slice()).ok()?;
        (!out.is_empty()).then_some(out)
    }

    /// Resolve a `System.Type` / `RuntimeType` object to the class name it
    /// represents (`typeof(Foo)` → `"Foo"`): `RuntimeType +0x10` is the
    /// `Il2CppType`, whose `+0x20` is the `Il2CppClass`, whose name we read.
    pub fn type_class_name<const N: usize>(&self, type_object: u64) -> Option<String> {
        let il2cpp_type = self.process.read_pointer::<u64>(type_object + 0x10).ok()?;
        let klass = self.process.read_pointer::<u64>(il2cpp_type + 0x20).ok()?;
        Class { class: klass }
            .class_name::<N>(self.process, self.module)
            .ok()?
            .validate_utf8()
            .ok()
            .map(str::to_string)
    }

    /// Resolve a live il2cpp object's class name (from its object header).
    pub fn object_class_name<const N: usize>(&self, obj: u64) -> Option<String> {
        Class::from_object(self.process, obj)?
            .class_name::<N>(self.process, self.module)
            .ok()?
            .validate_utf8()
            .ok()
            .map(str::to_string)
    }

    /// Read the element pointers of a Unity `List<T>` object (its `_items`
    /// backing array). Capped to a sane length as a safety bound.
    pub fn list_item_ptrs(&self, list_ptr: u64) -> Vec<u64> {
        const ITEMS_OFFSET: u64 = 0x10;
        const COUNT_OFFSET: u64 = 0x18;
        const ITEMS_0_INDEX_BASE: u64 = 0x20;
        const STRIDE: u64 = 0x8;
        const MAX_ITEMS: u32 = 64;

        let mut out = Vec::new();
        if list_ptr == 0 {
            return out;
        }
        let Ok(items) = self.process.read_pointer::<u64>(list_ptr + ITEMS_OFFSET) else {
            return out;
        };
        let Ok(count) = self.process.read_pointer::<u32>(items + COUNT_OFFSET) else {
            return out;
        };
        for i in 0..count.min(MAX_ITEMS) {
            if let Ok(item) = self
                .process
                .read_pointer::<u64>(items + ITEMS_0_INDEX_BASE + i as u64 * STRIDE)
                && item != 0
            {
                out.push(item);
            }
        }
        out
    }
}
