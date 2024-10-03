//! Support for attaching to Unity games that are using the IL2CPP backend.
pub mod unity_list;

use crate::pe;
use crate::process::MemoryError;
use crate::process::Process;
use crate::signature::Signature;
use crate::string::ArrayCString;
use bytemuck::Pod;
use core::iter;

const CSTR: usize = 128;

/// Represents access to a Unity game that is using the IL2CPP backend.
#[derive(Debug)]
pub struct Module {
    offsets: &'static Offsets,
    assemblies: u64,
    type_info_definition_table: u64,
}

impl Module {
    /// Tries attaching to a Unity game that is using the IL2CPP backend with
    /// the [IL2CPP version](Version) provided. The version needs to be
    /// correct for this function to work. If you don't know the version in
    /// advance, use [`attach_auto_detect`](Self::attach_auto_detect) instead.
    pub fn attach(process: &mut Process) -> Option<Self> {
        // let _ = process.module_address("GameAssembly.dll");

        let mono_module = {
            let address = process.module_address("GameAssembly.dll").ok()?;
            let size = pe::read_size_of_image(process, address)? as u64;
            (address, size)
        };

        let offsets = Offsets::new()?;

        let assemblies = {
            const ASSEMBLIES_TRG_SIG: Signature<12> =
                Signature::new("48 FF C5 80 3C ?? 00 75 ?? 48 8B 1D");

            let scan = ASSEMBLIES_TRG_SIG.scan_process_range(process, mono_module)?;
            let addr = scan + 12;

            addr + 0x4 + process.read::<i32>(addr).ok()? as u64
        };

        let type_info_definition_table = {
            const TYPE_INFO_DEFINITION_TABLE_TRG_SIG: Signature<10> =
                Signature::new("48 83 3C ?? 00 75 ?? 8B C? E8");

            let addr = TYPE_INFO_DEFINITION_TABLE_TRG_SIG
                .scan_process_range(process, mono_module)?
                .checked_add_signed(-4)
                .unwrap();

            process
                .read_pointer(addr + 0x4 + process.read::<i32>(addr).ok()? as u64)
                .ok()
                .filter(|val| *val != 0)?
        };

        Some(Self {
            offsets,
            assemblies,
            type_info_definition_table,
        })
    }

    fn assemblies<'a>(
        &'a self,
        process: &'a Process,
    ) -> impl DoubleEndedIterator<Item = Assembly> + 'a {
        let (assemblies, nr_of_assemblies): (u64, u64) = {
            let [first, limit] = process
                .read::<[u64; 2]>(self.assemblies)
                .unwrap_or_default();
            let count = limit.saturating_sub(first) / self.size_of_ptr();
            (first, count)
        };

        (0..nr_of_assemblies).filter_map(move |i| {
            Some(Assembly {
                assembly: process
                    .read_pointer(assemblies + i.wrapping_mul(self.size_of_ptr()))
                    .ok()?,
            })
        })
    }

    // Looks for the specified binary [image](Image) inside the target process.
    // An [image](Image) is a .NET DLL that is loaded
    // by the game. The `Assembly-CSharp` [image](Image) is the main game
    // assembly, and contains all the game logic. The
    // [`get_default_image`](Self::get_default_image) function is a shorthand
    // for this function that accesses the `Assembly-CSharp` [image](Image).
    pub fn get_image(&self, process: &Process, assembly_name: &str) -> Option<Image> {
        self.assemblies(process)
            .find(|assembly| {
                assembly
                    .get_name::<CSTR>(process, self)
                    .is_ok_and(|name| name.matches(assembly_name))
            })?
            .get_image(process, self)
    }

    // Looks for the `Assembly-CSharp` binary [image](Image) inside the target
    // process. An [image](Image) is a .NET DLL that is loaded
    // by the game. The `Assembly-CSharp` [image](Image) is the main
    // game assembly, and contains all the game logic. This function is a
    // shorthand for [`get_image`](Self::get_image) that accesses the
    // `Assembly-CSharp` [image](Image).
    pub fn get_default_image(&self, process: &Process) -> Option<Image> {
        self.get_image(process, "Assembly-CSharp")
    }

    fn size_of_ptr(&self) -> u64 {
        8
    }
}

#[derive(Copy, Clone, Debug)]
struct Assembly {
    assembly: u64,
}

impl Assembly {
    fn get_name<const N: usize>(
        &self,
        process: &Process,
        module: &Module,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read(process.read_pointer(
            self.assembly
                + module.offsets.monoassembly_aname as u64
                + module.offsets.monoassemblyname_name as u64,
        )?)
    }

    fn get_image(&self, process: &Process, module: &Module) -> Option<Image> {
        let image = Image {
            image: process
                .read_pointer(self.assembly + module.offsets.monoassembly_image as u64)
                .ok()?,
        };
        Some(image)
    }
}

///// An image is a .NET DLL that is loaded by the game. The `Assembly-CSharp`
///// image is the main game assembly, and contains all the game logic.
#[derive(Copy, Clone, Debug)]
pub struct Image {
    image: u64,
}

impl Image {
    /// Iterates over all [.NET classes](struct@Class) in the image.
    pub fn classes<'a>(
        &self,
        process: &'a Process,
        module: &'a Module,
    ) -> impl DoubleEndedIterator<Item = Class> + 'a {
        let type_count =
            process.read_pointer::<u32>(self.image + module.offsets.monoimage_typecount as u64);
        let metadata_ptr = match type_count {
            Ok(_) => process
                .read_pointer::<u64>(self.image + module.offsets.monoimage_metadatahandle as u64),
            _ => Err(MemoryError::ReadError {}),
        };
        let metadata_handle = match type_count {
            Ok(0) => None,
            Ok(_) => match metadata_ptr {
                Ok(x) => process.read::<u32>(x).ok(),
                _ => None,
            },
            _ => None,
        };

        let ptr = module.type_info_definition_table
            + (metadata_handle.unwrap() as u64 * module.size_of_ptr());

        let range = 0..type_count.unwrap_or_default() as u64;
        (range).filter_map(move |i| {
            let ptr = ptr + (i * module.size_of_ptr());
            let class = process
                .read_pointer::<u64>(ptr)
                .ok()
                .filter(|val| *val != 0x0)?;
            Some(Class { class })
        })
    }

    /// Tries to find the specified [.NET class](struct@Class) in the image.
    pub fn get_class(&self, process: &Process, module: &Module, class_name: &str) -> Option<Class> {
        self.classes(process, module).find(|class| {
            class
                .get_name::<CSTR>(process, module)
                .is_ok_and(|name| name.matches(class_name))
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Class {
    pub class: u64,
}

impl Class {
    fn get_name<const N: usize>(
        &self,
        process: &Process,
        module: &Module,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path(self.class, &[module.offsets.monoclass_name.into(), 0x0])
    }

    fn get_name_space<const N: usize>(
        &self,
        process: &Process,
        module: &Module,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path::<ArrayCString<N>>(
            self.class,
            &[module.offsets.monoclass_name_space.into(), 0x0],
        )
    }

    pub fn fields<'a>(
        &'a self,
        process: &'a Process,
        module: &'a Module,
    ) -> impl Iterator<Item = Field> + 'a {
        let mut this_class = Class { class: self.class };
        let mut iter_break = this_class.class == 0;

        iter::from_fn(move || {
            if iter_break {
                None
            } else if this_class.class != 0
                && this_class
                    .get_name::<CSTR>(process, module)
                    .is_ok_and(|name| !name.matches("Object"))
                && this_class
                    .get_name_space::<CSTR>(process, module)
                    .is_ok_and(|name| !name.matches("UnityEngine"))
            {
                let field_count = process
                    .read::<u16>((this_class.class) + module.offsets.monoclass_field_count as u64);

                let fields = match field_count {
                    Ok(_) => process
                        .read_pointer::<u64>(
                            this_class.class + module.offsets.monoclass_fields as u64,
                        )
                        .ok(),
                    _ => None,
                };

                let monoclassfield_structsize = module.offsets.monoclassfield_structsize as u64;

                if let Some(x) = this_class.get_parent(process, module) {
                    this_class = x;
                } else {
                    iter_break = true;
                }

                Some((0..field_count.unwrap_or_default()).filter_map(move |i| {
                    Some(Field {
                        field: fields? + (i as u64 * monoclassfield_structsize),
                    })
                }))
            } else {
                iter_break = true;
                None
            }
        })
        .flatten()
    }

    pub fn follow_fields<T: Pod>(
        &self,
        singleton: Class,
        process: &Process,
        module: &Module,
        fields: &[&str],
    ) -> Result<T, MemoryError> {
        if fields.is_empty() {
            return Err(MemoryError::Unset);
        }

        if singleton.class == 0 {
            return Err(MemoryError::NullPointer);
        }

        let last = fields.last().unwrap();

        let mut address = Class {
            class: singleton.class,
        };
        let mut fields_base = Class { class: self.class };
        for field in fields {
            match fields_base.get_field_offset(process, module, field) {
                Some(offset) => {
                    if field == last {
                        address.class += offset as u64;
                    } else {
                        address.class =
                            process.read_pointer::<u64>(address.class + offset as u64)?;
                        fields_base.class = process.read_pointer::<u64>(address.class)?;
                    }
                }
                None => return Err(MemoryError::ReadError),
            };
        }
        process.read_pointer::<T>(address.class)
    }

    pub fn follow_fields_without_read(
        &self,
        singleton: Class,
        process: &Process,
        module: &Module,
        fields: &[&str],
    ) -> Result<u64, MemoryError> {
        if fields.is_empty() {
            return Err(MemoryError::Unset);
        }

        if singleton.class == 0 {
            return Err(MemoryError::NullPointer);
        }

        let last = fields.last().unwrap();

        let mut address = Class {
            class: singleton.class,
        };
        let mut fields_base = Class { class: self.class };
        for field in fields {
            match fields_base.get_field_offset(process, module, field) {
                Some(offset) => {
                    if address.class == 0 {
                        return Err(MemoryError::NullPointer);
                    }
                    if field == last {
                        address.class += offset as u64;
                    } else {
                        address.class =
                            process.read_pointer::<u64>(address.class + offset as u64)?;
                        fields_base.class = process.read_pointer::<u64>(address.class)?;
                    }
                }
                None => return Err(MemoryError::ReadError),
            };
        }
        Ok(address.class)
    }

    /// Tries to find a field with the specified name in the class. This returns
    /// the offset of the field from the start of an instance of the class. If
    /// it's a static field, the offset will be from the start of the static
    /// table.
    pub fn get_field_offset(
        &self,
        process: &Process,
        module: &Module,
        field_name: &str,
    ) -> Option<u32> {
        let fields = self.fields(process, module).find(|field| {
            field
                .get_name::<CSTR>(process, module)
                .is_ok_and(|name| name.matches(field_name))
        })?;
        fields.get_offset(process, module)
    }

    /// Tries to find the address of a static instance of the class based on its
    /// field name. This waits until the field is not null.
    // pub async fn wait_get_static_instance(
    //     &self,
    //     process: &Process,
    //     module: &Module,
    //     field_name: &str,
    // ) -> u64 {
    //     let static_table = self.wait_get_static_table(process, module).await;
    //     let field_offset = self
    //         .wait_get_field_offset(process, module, field_name)
    //         .await;
    //     let singleton_location = static_table + field_offset;

    //     retry(|| {
    //         process
    //             .read_pointer(singleton_location, module.pointer_size)
    //             .ok()
    //             .filter(|val| *val != 0)
    //     })
    //     .await
    // }

    fn get_static_table_pointer(&self, module: &Module) -> u64 {
        self.class + module.offsets.monoclass_static_fields as u64
    }

    /// Returns the address of the static table of the class. This contains the
    /// values of all the static fields.
    pub fn get_static_table(&self, process: &Process, module: &Module) -> Option<u64> {
        process
            .read_pointer::<u64>(self.get_static_table_pointer(module))
            .ok()
            .filter(|val| *val != 0)
    }

    /// Tries to find the parent class.
    pub fn get_parent(&self, process: &Process, module: &Module) -> Option<Class> {
        let parent = process
            .read_pointer::<u64>(self.class + module.offsets.monoclass_parent as u64)
            .ok()
            .filter(|val| *val != 0)?;
        Some(Class { class: parent })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Field {
    field: u64,
}

impl Field {
    fn get_name<const N: usize>(
        &self,
        process: &Process,
        module: &Module,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path::<ArrayCString<N>>(
            self.field,
            &[module.offsets.monoclassfield_name.into(), 0x0],
        )
    }

    fn get_offset(&self, process: &Process, module: &Module) -> Option<u32> {
        match process.read(self.field + module.offsets.monoclassfield_offset as u64) {
            Ok(value) => Some(value),
            Err(_error) => None,
        }
    }
}

///// An IL2CPP-specific implementation for automatic pointer path resolution
//#[derive(Clone)]
//pub struct UnityPointer<const CAP: usize> {
//cache: RefCell<UnityPointerCache<CAP>>,
//class_name: &'static str,
//nr_of_parents: usize,
//fields: [&'static str; CAP],
//depth: usize,
//}

//#[derive(Clone, Copy)]
//struct UnityPointerCache<const CAP: usize> {
//base_address: u64,
//offsets: [u64; CAP],
//resolved_offsets: usize,
//current_instance_pointer: Option<u64>,
//starting_class: Option<Class>,
//}

//impl<const CAP: usize> UnityPointer<CAP> {
///// Creates a new instance of the Pointer struct
/////
///// `CAP` should be higher or equal to the number of offsets defined in `fields`.
/////
///// If a higher number of offsets is provided, the pointer path will be truncated
///// according to the value of `CAP`.
//pub fn new(class_name: &'static str, nr_of_parents: usize, fields: &[&'static str]) -> Self {
//    let this_fields: [&str; CAP] = {
//        let mut iter = fields.iter();
//        array::from_fn(|_| iter.next().copied().unwrap_or_default())
//    };

//    let cache = RefCell::new(UnityPointerCache {
//        base_address: 0,
//        offsets: [u64::default(); CAP],
//        current_instance_pointer: None,
//        starting_class: None,
//        resolved_offsets: usize::default(),
//    });

//    Self {
//        cache,
//        class_name,
//        nr_of_parents,
//        fields: this_fields,
//        depth: fields.len().min(CAP),
//    }
//}

///// Tries to resolve the pointer path for the `IL2CPP` class specified
//fn find_offsets(&self, process: &Process, module: &Module, image: &Image) -> Result<(), Error> {
//    let mut cache = self.cache.borrow_mut();

//    // If the pointer path has already been found, there's no need to continue
//    if cache.resolved_offsets == self.depth {
//        return Ok(());
//    }

//    // Logic: the starting class can be recovered with the get_class() function,
//    // and parent class can be recovered if needed. However, this is a VERY
//    // intensive process because it involves looping through all the main classes
//    // in the game. For this reason, once the class is found, we want to store it
//    // into the cache, where it can be recovered if this function need to be run again
//    // (for example if a previous attempt at pointer path resolution failed)
//    let starting_class = match cache.starting_class {
//        Some(starting_class) => starting_class,
//        _ => {
//            let mut current_class = image
//                .get_class(process, module, self.class_name)
//                .ok_or(Error {})?;

//            for _ in 0..self.nr_of_parents {
//                current_class = current_class.get_parent(process, module).ok_or(Error {})?;
//            }

//            cache.starting_class = Some(current_class);
//            current_class
//        }
//    };

//    // Recovering the address of the static table is not very CPU intensive,
//    // but it might be worth caching it as well
//    if cache.base_address == 0 {
//        let s_table = starting_class
//            .get_static_table(process, module)
//            .ok_or(Error {})?;
//        cache.base_address = s_table;
//    };

//    // As we need to be able to find instances in a more reliable way,
//    // instead of the Class itself, we need the address pointing to an
//    // instance of that Class. If the cache is empty, we start from the
//    // pointer to the static table of the first class.
//    let mut current_instance_pointer = match cache.current_instance_pointer {
//        Some(val) => val,
//        _ => starting_class.get_static_table_pointer(module),
//    };

//    // We keep track of the already resolved offsets in order to skip resolving them again
//    for i in cache.resolved_offsets..self.depth {
//        let class_instance = process
//            .read_pointer(current_instance_pointer)
//            .ok()
//            .filter(|val| *val != 0)
//            .ok_or(Error {})?;

//        // Try to parse the offset, passed as a string, as an actual hex or decimal value
//        let offset_from_string = super::value_from_string(self.fields[i]);

//        let current_offset = match offset_from_string {
//            Some(offset) => offset as u64,
//            _ => {
//                let current_class = match i {
//                    0 => starting_class,
//                    _ => {
//                        let class = process
//                            .read_pointer(class_instance)
//                            .ok()
//                            .filter(|val| *val != 0)
//                            .ok_or(Error {})?;
//                        Class { class }
//                    }
//                };

//                let val = current_class
//                    .fields(process, module)
//                    .find(|field| {
//                        field
//                            .get_name::<CSTR>(process, module)
//                            .is_ok_and(|name| name.matches(self.fields[i]))
//                    })
//                    .ok_or(Error {})?
//                    .get_offset(process, module)
//                    .ok_or(Error {})? as u64;

//                // Explicitly allowing this clippy because of borrowing rules shenanigans
//                #[allow(clippy::let_and_return)]
//                val
//            }
//        };

//        cache.offsets[i] = current_offset;

//        current_instance_pointer = class_instance + current_offset;
//        cache.current_instance_pointer = Some(current_instance_pointer);
//        cache.resolved_offsets += 1;
//    }

//    Ok(())
//}

///// Dereferences the pointer path, returning the memory address of the value of interest
//pub fn deref_offsets(
//    &self,
//    process: &Process,
//    module: &Module,
//    image: &Image,
//) -> Result<u64, Error> {
//    self.find_offsets(process, module, image)?;
//    let cache = self.cache.borrow();
//    let mut address = cache.base_address;
//    let (&last, path) = cache.offsets[..self.depth].split_last().ok_or(Error {})?;
//    for &offset in path {
//        address = process.read_pointer(address + offset)?;
//    }
//    Ok(address + last)
//}

///// Dereferences the pointer path, returning the value stored at the final memory address
//pub fn deref<T: CheckedBitPattern>(
//    &self,
//    process: &Process,
//    module: &Module,
//    image: &Image,
//) -> Result<T, Error> {
//    self.find_offsets(process, module, image)?;
//    let cache = self.cache.borrow();
//    process.read_pointer_path(
//        cache.base_address,
//        &cache.offsets[..self.depth],
//    )
//}

// /// Generates a `DeepPointer` struct based on the offsets
// /// recovered from this `UnityPointer`.
// // pub fn get_deep_pointer(
// //     &self,
// //     process: &Process,
// //     module: &Module,
// //     image: &Image,
// // ) -> Option<DeepPointer<CAP>> {
// //     self.find_offsets(process, module, image).ok()?;
// //     let cache = self.cache.borrow();
// //     Some(DeepPointer::<CAP>::new(
// //         cache.base_address,
// //         module.pointer_size,
// //         &cache.offsets[..self.depth],
// //     ))
// // }
// }

#[derive(Debug)]
struct Offsets {
    monoassembly_image: u8,
    monoassembly_aname: u8,
    monoassemblyname_name: u8,
    monoimage_typecount: u8,
    monoimage_metadatahandle: u8,
    monoclass_name: u8,
    monoclass_name_space: u8,
    monoclass_fields: u8,
    monoclass_field_count: u16,
    monoclass_static_fields: u8,
    monoclass_parent: u8,
    monoclassfield_structsize: u8,
    monoclassfield_name: u8,
    monoclassfield_offset: u8,
}

impl Offsets {
    const fn new() -> Option<&'static Self> {
        Some(&Self {
            monoassembly_image: 0x0,
            monoassembly_aname: 0x18,
            monoassemblyname_name: 0x0,
            monoimage_typecount: 0x18,
            monoimage_metadatahandle: 0x28,
            monoclass_name: 0x10,
            monoclass_name_space: 0x18,
            monoclass_fields: 0x80,
            monoclass_field_count: 0x120,
            monoclass_static_fields: 0xB8,
            monoclass_parent: 0x58,
            monoclassfield_structsize: 0x20,
            monoclassfield_name: 0x0,
            monoclassfield_offset: 0x18,
        })
    }
}
