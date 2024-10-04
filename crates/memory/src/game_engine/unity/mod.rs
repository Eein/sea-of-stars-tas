//! Support for attaching to Unity games that are using the standard Mono
//! backend.

use crate::pe;
use crate::process::MemoryError;
use crate::process::Process;
use crate::signature::Signature;
use crate::string::ArrayCString;
use bytemuck::Pod;
use core::iter;

const CSTR: usize = 128;

/// Represents access to a Unity game that is using the standard Mono backend.
pub struct Module {
    version: Version,
    offsets: &'static Offsets,
    assemblies: u64,
}

impl Module {
    /// Tries attaching to a Unity game that is using the standard Mono backend.
    /// This function automatically detects the [Mono version](Version). If you
    /// know the version in advance or it fails detecting it, use
    /// [`attach`](Self::attach) instead.
    pub fn attach_auto_detect(process: &mut Process) -> Option<Self> {
        let version = detect_version(process)?;
        Self::attach(process, version)
    }

    /// Tries attaching to a Unity game that is using the standard Mono backend
    /// with the [Mono version](Version) provided. The version needs to be
    /// correct for this function to work. If you don't know the version in
    /// advance, use [`attach_auto_detect`](Self::attach_auto_detect) instead.
    pub fn attach(process: &mut Process, version: Version) -> Option<Self> {
        let module = ["mono.dll", "mono-2.0-bdwgc.dll"]
            .iter()
            .find_map(|&name| process.module_address(name).ok())?;

        let offsets = Offsets::new(version)?;

        let root_domain_function_address = pe::symbols(process, module)
            .find(|symbol| {
                symbol
                    .get_name::<25>(process)
                    .is_ok_and(|name| name.matches("mono_assembly_foreach"))
            })?
            .address;

        let assemblies_pointer: u64 = {
            const SIG_MONO_64: Signature<3> = Signature::new("48 8B 0D");
            let scan_address: u64 = SIG_MONO_64
                .scan_process_range(process, (root_domain_function_address, 0x100))?
            + 3;
            scan_address + 0x4 + process.read::<i32>(scan_address).ok()? as u64
        };

        let assemblies = process
            .read_pointer::<u64>(assemblies_pointer as u64)
            .ok()
            .filter(|val| *val != 0)?;

        Some(Self {
            version,
            offsets,
            assemblies,
        })
    }

    fn assemblies<'a>(&'a self, process: &'a Process) -> impl Iterator<Item = Assembly> + 'a {
        let mut assembly = self.assemblies;
        let mut iter_break = assembly == 0;
        iter::from_fn(move || {
            if iter_break {
                None
            } else {
                let [data, next_assembly]: [u64; 2] = process
                        .read::<[u64; 2]>(assembly)
                        .ok()?
                        .map(|item| item.into());

                if next_assembly == 0 {
                    iter_break = true;
                } else {
                    assembly = next_assembly;
                }

                Some(Assembly { assembly: data })
            }
        })
    }

    /// Looks for the specified binary [image](Image) inside the target process.
    /// An [image](Image) is a .NET DLL that is loaded
    /// by the game. The `Assembly-CSharp` [image](Image) is the main game
    /// assembly, and contains all the game logic. The
    /// [`get_default_image`](Self::get_default_image) function is a shorthand
    /// for this function that accesses the `Assembly-CSharp` [image](Image).
    pub fn get_image(&self, process: &Process, assembly_name: &str) -> Option<Image> {
        self.assemblies(process)
            .find(|assembly| {
                assembly
                    .get_name::<CSTR>(process, self)
                    .is_ok_and(|name| name.matches(assembly_name))
            })?
            .get_image(process, self)
    }

    /// Looks for the `Assembly-CSharp` binary [image](Image) inside the target
    /// process. An [image](Image) is a .NET DLL that is loaded
    /// by the game. The `Assembly-CSharp` [image](Image) is the main
    /// game assembly, and contains all the game logic. This function is a
    /// shorthand for [`get_image`](Self::get_image) that accesses the
    /// `Assembly-CSharp` [image](Image).
    pub fn get_default_image(&self, process: &Process) -> Option<Image> {
        self.get_image(process, "Assembly-CSharp")
    }


    #[inline]
    const fn size_of_ptr(&self) -> u64 {
        8
    }
}

#[derive(Copy, Clone)]
pub struct Assembly {
    assembly: u64,
}

impl Assembly {
    fn get_name<const N: usize>(
        &self,
        process: &Process,
        module: &Module,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path(
            self.assembly,
            &[module.offsets.monoassembly_aname.into(), 0x0],
        )
    }

    fn get_image(&self, process: &Process, module: &Module) -> Option<Image> {
        Some(Image {
            image: process
                .read_pointer(
                    self.assembly + module.offsets.monoassembly_image as u64
                )
                .ok()
                .filter(|val| *val != 0)?,
        })
    }
}

/// An image is a .NET DLL that is loaded by the game. The `Assembly-CSharp`
/// image is the main game assembly, and contains all the game logic.
#[derive(Copy, Clone)]
pub struct Image {
    pub image: u64,
}

impl Image {
    /// Iterates over all [.NET classes](struct@Class) in the image.
    pub fn classes<'a>(
        &self,
        process: &'a Process,
        module: &'a Module,
    ) -> impl Iterator<Item = Class> + 'a {
        let class_cache_size = process
            .read::<i32>(
                self.image
                    + module.offsets.monoimage_class_cache as u64
                    + module.offsets.monointernalhashtable_size as u64
            )
            .ok()
            .filter(|&val| val != 0);

        let table_addr = match class_cache_size {
            Some(_) => process.read_pointer::<u64>(
                self.image
                    + module.offsets.monoimage_class_cache as u64
                    + module.offsets.monointernalhashtable_table as u64
            ),
            _ => Err(MemoryError::Unset {}),
        };

        (0..class_cache_size.unwrap_or_default()).flat_map(move |i| {
            let mut table = match table_addr {
                Ok(table_addr) => process
                    .read_pointer(
                        table_addr + (i as u64).wrapping_mul(module.size_of_ptr()),
                    )
                    .ok(),
                _ => None,
            };

            iter::from_fn(move || {
                let class = process.read_pointer(table?).ok()?;

                table = process
                    .read_pointer(
                        table? + module.offsets.monoclassdef_next_class_cache as u64
                    )
                    .ok()
                    .filter(|val| *val != 0);

                Some(Class { class })
            })
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

/// A .NET class that is part of an [`Image`](Image).
#[derive(Copy, Clone)]
pub struct Class {
    pub class: u64,
}

impl Class {
    fn get_name<const N: usize>(
        &self,
        process: &Process,
        module: &Module,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path(
            self.class,
            &[
                module.offsets.monoclassdef_klass as u64 + module.offsets.monoclass_name as u64,
                0x0,
            ],
        )
    }

    fn get_name_space<const N: usize>(
        &self,
        process: &Process,
        module: &Module,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path(
            self.class,
            &[
                module.offsets.monoclassdef_klass as u64 + module.offsets.monoclass_name_space as u64,
                0x0,
            ],
        )
    }

    fn fields<'a>(
        &'a self,
        process: &'a Process,
        module: &'a Module,
    ) -> impl Iterator<Item = Field> + 'a {
        let mut this_class = Class { class: self.class };
        let mut iter_break = this_class.class == 0;

        iter::from_fn(move || {
            if iter_break {
                None
            } else if !this_class.class == 0
                && this_class
                    .get_name::<CSTR>(process, module)
                    .is_ok_and(|name| !name.matches("Object"))
                && this_class
                    .get_name_space::<CSTR>(process, module)
                    .is_ok_and(|name| !name.matches("UnityEngine"))
            {
                let field_count = process
                    .read::<u32>(this_class.class + module.offsets.monoclassdef_field_count as u64)
                    .ok()
                    .filter(|&val| val != 0);

                let fields = match field_count {
                    Some(_) => process
                        .read_pointer::<u64>(
                            this_class.class
                                + module.offsets.monoclassdef_klass as u64
                                + module.offsets.monoclass_fields as u64
                        )
                        .ok(),
                    _ => None,
                };

                let monoclassfieldalignment = module.offsets.monoclassfieldalignment as u64;

                if let Some(x) = this_class.get_parent(process, module) {
                    this_class = x;
                } else {
                    iter_break = true;
                }

                Some(
                    (0..field_count.unwrap_or_default()).filter_map(move |i| {
                        Some(Field {
                            field: fields? + (i as u64 * monoclassfieldalignment),
                        })
                    }),
                )
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
            return Err(MemoryError::InvalidParameters);
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
            return Err(MemoryError::InvalidParameters);
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

    /// Tries to find the offset for a field with the specified name in the class.
    /// If it's a static field, the offset will be from the start of the static
    /// table.
    pub fn get_field_offset(
        &self,
        process: &Process,
        module: &Module,
        field_name: &str,
    ) -> Option<u32> {
        self.fields(process, module)
            .find(|field| {
                field
                    .get_name::<CSTR>(process, module)
                    .is_ok_and(|name| name.matches(field_name))
            })?
            .get_offset(process, module)
    }

    fn get_static_table_pointer(&self, process: &Process, module: &Module) -> Option<u64> {
        let runtime_info = process
            .read_pointer::<u64>(
                self.class
                    + module.offsets.monoclassdef_klass as u64
                    + module.offsets.monoclass_runtime_info as u64
            )
            .ok()?;

        let mut vtables = process
            .read_pointer::<u64>(
                runtime_info + module.offsets.monoclassruntimeinfo_domain_vtables as u64,
            )
            .ok()?;

        // Mono V1 behaves differently when it comes to recover the static table
        match module.version {
            Version::V1 => Some(vtables + module.offsets.monoclass_vtable_size as u64),
            _ => {
                vtables = vtables + module.offsets.monovtable_vtable as u64;

                let vtable_size = process
                    .read::<u32>(
                        self.class
                            + module.offsets.monoclassdef_klass as u64
                            + module.offsets.monoclass_vtable_size as u64
                    )
                    .ok()?;

                Some(vtables + (vtable_size as u64).wrapping_mul(module.size_of_ptr()))
            }
        }
    }

    /// Returns the address of the static table of the class. This contains the
    /// values of all the static fields.
    pub fn get_static_table(&self, process: &Process, module: &Module) -> Option<u64> {
        process
            .read_pointer::<u64>(
                self.get_static_table_pointer(process, module)?
            )
            .ok()
            .filter(|val| *val != 0)
    }

    /// Tries to find the parent class.
    pub fn get_parent(&self, process: &Process, module: &Module) -> Option<Class> {
        let parent_addr = process
            .read_pointer::<u64>(
                self.class + module.offsets.monoclassdef_klass as u64 + module.offsets.monoclass_parent as u64
            )
            .ok()
            .filter(|val| *val != 0)?;

        Some(Class {
            class: process
                .read_pointer::<u64>(parent_addr)
                .ok()
                .filter(|val| *val != 0)?,
        })
    }
}

#[derive(Copy, Clone)]
pub struct Field {
    field: u64,
}

impl Field {
    fn get_name<const N: usize>(
        &self,
        process: &Process,
        module: &Module,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path(
            self.field,
            &[module.offsets.monoclassfield_name.into(), 0x0],
        )
    }

    fn get_offset(&self, process: &Process, module: &Module) -> Option<u32> {
        process
            .read(self.field + module.offsets.monoclassfield_offset as u64)
            .ok()
    }
}

pub struct Offsets {
    monoassembly_aname: u8,
    monoassembly_image: u8,
    monoimage_class_cache: u16,
    monointernalhashtable_table: u8,
    monointernalhashtable_size: u8,
    monoclassdef_next_class_cache: u16,
    monoclassdef_klass: u8,
    monoclass_name: u8,
    monoclass_name_space: u8,
    monoclass_fields: u8,
    monoclassdef_field_count: u16,
    monoclass_runtime_info: u8,
    monoclass_vtable_size: u8,
    monoclass_parent: u8,
    monoclassfield_name: u8,
    monoclassfield_offset: u8,
    monoclassruntimeinfo_domain_vtables: u8,
    monovtable_vtable: u8,
    monoclassfieldalignment: u8,
}

impl Offsets {
    const fn new(version: Version) -> Option<&'static Self> {
        match version {
                Version::V1 => Some(&Self {
                    monoassembly_aname: 0x10,
                    monoassembly_image: 0x58,
                    monoimage_class_cache: 0x3D0,
                    monointernalhashtable_table: 0x20,
                    monointernalhashtable_size: 0x18,
                    monoclassdef_next_class_cache: 0x100,
                    monoclassdef_klass: 0x0,
                    monoclass_name: 0x48,
                    monoclass_name_space: 0x50,
                    monoclass_fields: 0xA8,
                    monoclassdef_field_count: 0x94,
                    monoclass_runtime_info: 0xF8,
                    monoclass_vtable_size: 0x18, // MonoVtable.data
                    monoclass_parent: 0x30,
                    monoclassfield_name: 0x8,
                    monoclassfield_offset: 0x18,
                    monoclassruntimeinfo_domain_vtables: 0x8,
                    monovtable_vtable: 0x48,
                    monoclassfieldalignment: 0x20,
                }),
                Version::V2 => Some(&Self {
                    monoassembly_aname: 0x10,
                    monoassembly_image: 0x60,
                    monoimage_class_cache: 0x4C0,
                    monointernalhashtable_table: 0x20,
                    monointernalhashtable_size: 0x18,
                    monoclassdef_next_class_cache: 0x108,
                    monoclassdef_klass: 0x0,
                    monoclass_name: 0x48,
                    monoclass_name_space: 0x50,
                    monoclass_fields: 0x98,
                    monoclassdef_field_count: 0x100,
                    monoclass_runtime_info: 0xD0,
                    monoclass_vtable_size: 0x5C,
                    monoclass_parent: 0x30,
                    monoclassfield_name: 0x8,
                    monoclassfield_offset: 0x18,
                    monoclassruntimeinfo_domain_vtables: 0x8,
                    monovtable_vtable: 0x40,
                    monoclassfieldalignment: 0x20,
                }),
                Version::V3 => Some(&Self {
                    monoassembly_aname: 0x10,
                    monoassembly_image: 0x60,
                    monoimage_class_cache: 0x4D0,
                    monointernalhashtable_table: 0x20,
                    monointernalhashtable_size: 0x18,
                    monoclassdef_next_class_cache: 0x108,
                    monoclassdef_klass: 0x0,
                    monoclass_name: 0x48,
                    monoclass_name_space: 0x50,
                    monoclass_fields: 0x98,
                    monoclassdef_field_count: 0x100,
                    monoclass_runtime_info: 0xD0,
                    monoclass_vtable_size: 0x5C,
                    monoclass_parent: 0x30,
                    monoclassfield_name: 0x8,
                    monoclassfield_offset: 0x18,
                    monoclassruntimeinfo_domain_vtables: 0x8,
                    monovtable_vtable: 0x48,
                    monoclassfieldalignment: 0x20,
                }),
        }
    }
}

/// The version of Mono that was used for the game. These don't correlate to the
/// Mono version numbers.
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub enum Version {
    /// Version 1
    V1,
    /// Version 2
    V2,
    /// Version 3
    V3,
}

fn detect_version(process: &mut Process) -> Option<Version> {
    if process.module_address("mono.dll").is_ok() {
        return Some(Version::V1);
    }

    let unity_module = {
        let address = process.module_address("UnityPlayer.dll").ok()?;
        let range = pe::read_size_of_image(process, address)? as u64;
        (address, range)
    };

    const SIG_202X: Signature<6> = Signature::new("00 32 30 32 ?? 2E");

    let Some(addr) = SIG_202X.scan_process_range(process, unity_module) else {
        return Some(Version::V2);
    };

    const ZERO: u8 = b'0';
    const NINE: u8 = b'9';

    let version_string = process.read::<[u8; 6]>(addr + 1).ok()?;

    let (before, after) = version_string.split_at(version_string.iter().position(|&x| x == b'.')?);

    let mut unity: u32 = 0;
    for &val in before {
        match val {
            ZERO..=NINE => unity = unity * 10 + (val - ZERO) as u32,
            _ => break,
        }
    }

    let mut unity_minor: u32 = 0;
    for &val in &after[1..] {
        match val {
            ZERO..=NINE => unity_minor = unity_minor * 10 + (val - ZERO) as u32,
            _ => break,
        }
    }

    Some(if (unity == 2021 && unity_minor >= 2) || (unity > 2021) {
        Version::V3
    } else {
        Version::V2
    })
}
