//! Support for identifying the current scene in Unity games.

// References:
// https://gist.githubusercontent.com/just-ero/92457b51baf85bd1e5b8c87de8c9835e/raw/8aa3e6b8da01fd03ff2ff0c03cbd018e522ef988/UnityScene.hpp
// (some offsets seem to be wrong anyway, but it's a very good starting point)
//
// Offsets and logic for Transforms and GameObjects taken from https://github.com/Micrologist/UnityInstanceDumper

use core::{array, iter, mem::MaybeUninit};

use crate::process::MemoryError;
use crate::process::Process;
use crate::signature::Signature;
use crate::string::ArrayCString;

const CSTR: usize = 128;

/// The scene manager allows you to easily identify the current scene loaded in
/// the attached Unity game.
///
/// It can be useful to identify splitting conditions or as an alternative to
/// the traditional class lookup in games with no useful static references.
#[derive(Debug)]
pub struct SceneManager {
    is_il2cpp: bool,
    address: u64,
    offsets: &'static Offsets,
}

impl SceneManager {
    /// Attaches to the scene manager in the given process.
    pub fn attach(process: &mut Process) -> Option<Self> {
        const SIG_64_BIT: Signature<13> = Signature::new("48 83 EC 20 4C 8B ?5 ???????? 33 F6");

        let unity_player = process.module_range("UnityPlayer.dll").ok()?;
        let is_il2cpp = process.module_address("GameAssembly.dll").is_ok();

        // There are multiple signatures that can be used, depending on the version of Unity
        // used in the target game.
        let base_address: u64 = {
            let addr = SIG_64_BIT.scan_process_range(process, unity_player)? + 7;
            addr + 0x4 + process.read::<i32>(addr).ok()? as u64
        };

        let offsets = Offsets::new();

        // Dereferencing one level because this pointer never changes as long as the game is open.
        // It might not seem a lot, but it helps make things a bit faster when querying for scene stuff.
        let address = process
            .read_pointer(base_address)
            .ok()
            .filter(|val| *val != 0)?;

        Some(Self {
            is_il2cpp,
            address,
            offsets,
        })
    }

    #[inline]
    const fn size_of_ptr(&self) -> u64 {
        8
    }

    /// Tries to retrieve the current active scene.
    fn get_current_scene(&self, process: &Process) -> Result<Scene, MemoryError> {
        Ok(Scene {
            address: process
                .read_pointer(self.address + self.offsets.active_scene as u64)
                .ok()
                .filter(|val| *val != 0)
                .ok_or(MemoryError::ReadError)?,
        })
    }

    /// `DontDestroyOnLoad` is a special Unity scene containing game objects
    /// that must be preserved when switching between different scenes (eg. a
    /// `scene1` starting some background music that continues when `scene2`
    /// loads).
    fn get_dont_destroy_on_load_scene(&self) -> Scene {
        Scene {
            address: self.address + self.offsets.dont_destroy_on_load_scene as u64,
        }
    }

    /// Returns the current scene index.
    ///
    /// The value returned is a [`i32`] because some games will show `-1` as their
    /// current scene until fully initialized.
    pub fn get_current_scene_index(&self, process: &Process) -> Result<i32, MemoryError> {
        self.get_current_scene(process)?.index(process, self)
    }

    /// Returns the full path to the current scene. Use [`get_scene_name`]
    /// afterwards to get the scene name.
    pub fn get_current_scene_path<const N: usize>(
        &self,
        process: &Process,
    ) -> Result<ArrayCString<N>, MemoryError> {
        self.get_current_scene(process)?.path(process, self)
    }

    /// Returns the number of currently loaded scenes in the attached game.
    pub fn get_scene_count(&self, process: &Process) -> Result<u32, MemoryError> {
        process.read(self.address + self.offsets.scene_count as u64)
    }

    /// Iterates over all the currently loaded scenes in the attached game.
    pub fn scenes<'a>(
        &'a self,
        process: &'a Process,
    ) -> impl DoubleEndedIterator<Item = Scene> + 'a {
        let (num_scenes, addr): (usize, u64) = {
            let [first, _, third] = process
                .read::<[u64; 3]>(self.address + self.offsets.scene_count as u64)
                .unwrap_or_default();
            (first as usize, third)
        };

        (0..num_scenes).filter_map(move |index| {
            Some(Scene {
                address: process
                    .read_pointer(addr + (index as u64).wrapping_mul(self.size_of_ptr()))
                    .ok()
                    .filter(|val| *val != 0)?,
            })
        })
    }

    /// Iterates over all root [`Transform`]s declared for the
    /// specified scene.
    ///
    /// Each Unity scene normally has a linked list of [`Transform`]s.
    /// Each one can, recursively, have one or more children [`Transform`]s
    /// (and so on), as well as a list of `Component`s, which are classes (eg.
    /// `MonoBehaviour`) containing data we might want to retrieve for the auto
    /// splitter logic.
    fn root_game_objects<'a>(
        &'a self,
        process: &'a Process,
        scene: &Scene,
    ) -> impl Iterator<Item = Transform> + 'a {
        let list_first: u64 = process
            .read_pointer(scene.address + self.offsets.root_storage_container as u64)
            .unwrap_or_default();

        let mut current_list = list_first;
        let mut iter_break = current_list == 0;

        iter::from_fn(move || {
            if iter_break {
                None
            } else {
                let [first, _, third]: [u64; 3] =
                    { process.read::<[u64; 3]>(current_list).ok()?.map(|a| a) };

                if first == list_first {
                    iter_break = true;
                } else {
                    current_list = first;
                }

                Some(Transform { address: third })
            }
        })
    }

    /// Tries to find the specified root [`Transform`] from the currently
    /// active Unity scene.
    pub fn get_root_game_object(
        &self,
        process: &Process,
        name: &str,
    ) -> Result<Transform, MemoryError> {
        self.root_game_objects(process, &self.get_current_scene(process)?)
            .find(|obj| {
                obj.get_name::<CSTR>(process, self)
                    .is_ok_and(|obj_name| obj_name.matches(name))
            })
            .ok_or(MemoryError::ReadError)
    }

    /// Tries to find the specified root [`Transform`] from the
    /// `DontDestroyOnLoad` Unity scene.
    pub fn get_game_object_from_dont_destroy_on_load(
        &self,
        process: &Process,
        name: &str,
    ) -> Result<Transform, MemoryError> {
        self.root_game_objects(process, &self.get_dont_destroy_on_load_scene())
            .find(|obj| {
                obj.get_name::<CSTR>(process, self)
                    .is_ok_and(|obj_name| obj_name.matches(name))
            })
            .ok_or(MemoryError::ReadError)
    }
}

pub struct Transform {
    address: u64,
}

impl Transform {
    /// Tries to return the name of the current `Transform`.
    pub fn get_name<const N: usize>(
        &self,
        process: &Process,
        scene_manager: &SceneManager,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path(
            self.address,
            &[
                scene_manager.offsets.game_object as u64,
                scene_manager.offsets.game_object_name as u64,
                0x0,
            ],
        )
    }

    /// Iterates over the classes referred to in the current `Transform`.
    pub fn classes<'a>(
        &'a self,
        process: &'a Process,
        scene_manager: &'a SceneManager,
    ) -> Result<impl Iterator<Item = u64> + 'a, MemoryError> {
        let game_object: u64 = process
            .read_pointer(self.address + scene_manager.offsets.game_object as u64)
            .ok()
            .filter(|&val| val != 0)
            .unwrap_or_default();

        let (number_of_components, main_object): (usize, u64) = {
            let array =
                process.read::<[u64; 3]>(game_object + scene_manager.offsets.game_object as u64)?;
            (array[2] as usize, array[0])
        };

        if number_of_components == 0 {
            return Err(MemoryError::ReadError);
        }

        const ARRAY_SIZE: usize = 128;

        let components: [u64; ARRAY_SIZE] = {
            let mut buf = [MaybeUninit::<[u64; 2]>::uninit(); ARRAY_SIZE];
            let slice =
                process.read_into_uninit_slice(main_object, &mut buf[..number_of_components])?;

            let mut iter = slice.iter_mut();
            array::from_fn(|_| {
                iter.next()
                    .map(|&mut [_, second]| second)
                    .unwrap_or_default()
            })
        };

        Ok((1..number_of_components).filter_map(move |m| {
            process
                .read_pointer(components[m] + scene_manager.offsets.klass as u64)
                .ok()
                .filter(|val| *val != 0)
        }))
    }

    /// Tries to find the base address of a class in the current `GameObject`.
    pub fn get_class(
        &self,
        process: &Process,
        scene_manager: &SceneManager,
        name: &str,
    ) -> Result<u64, MemoryError> {
        self.classes(process, scene_manager)?
            .find(|&addr| {
                let val: Result<ArrayCString<CSTR>, MemoryError> = match scene_manager.is_il2cpp {
                    true => process.read_pointer_path(
                        addr,
                        &[0x0, scene_manager.size_of_ptr().wrapping_mul(2), 0x0],
                    ),
                    false => process.read_pointer_path(
                        addr,
                        &[0x0, 0x0, scene_manager.offsets.klass_name as u64, 0x0],
                    ),
                };

                val.is_ok_and(|class_name| class_name.matches(name))
            })
            .ok_or(MemoryError::ReadError)
    }

    /// Iterates over children `Transform`s referred by the current one
    pub fn children<'a>(
        &'a self,
        process: &'a Process,
        scene_manager: &'a SceneManager,
    ) -> Result<impl Iterator<Item = Self> + 'a, MemoryError> {
        let (child_count, child_pointer): (usize, u64) = {
            let [first, _, third] = process
                .read::<[u64; 3]>(self.address + scene_manager.offsets.children_pointer as u64)?;
            (third as usize, first)
        };

        // Define an empty array and fill it later with the addresses of all child classes found for the current Transform.
        // Reading the whole array of pointers is (slightly) faster than reading each address in a loop
        const ARRAY_SIZE: usize = 128;

        if child_count == 0 || child_count > ARRAY_SIZE {
            return Err(MemoryError::ReadError);
        }

        let children: [u64; ARRAY_SIZE] = {
            let mut buf = [MaybeUninit::<u64>::uninit(); ARRAY_SIZE];
            let slice = process.read_into_uninit_slice(child_pointer, &mut buf[..child_count])?;

            let mut iter = slice.iter_mut();
            array::from_fn(|_| iter.next().copied().map(Into::into).unwrap_or_default())
        };

        Ok((0..child_count).map(move |f| Self {
            address: children[f],
        }))
    }

    /// Tries to find a child `Transform` from the current one.
    pub fn get_child(
        &self,
        process: &Process,
        scene_manager: &SceneManager,
        name: &str,
    ) -> Result<Self, MemoryError> {
        self.children(process, scene_manager)?
            .find(|p| {
                p.get_name::<CSTR>(process, scene_manager)
                    .is_ok_and(|obj_name| obj_name.matches(name))
            })
            .ok_or(MemoryError::ReadError)
    }
}

#[derive(Debug)]
struct Offsets {
    scene_count: u8,
    active_scene: u8,
    dont_destroy_on_load_scene: u8,
    asset_path: u8,
    build_index: u8,
    root_storage_container: u8,
    game_object: u8,
    game_object_name: u8,
    klass: u8,
    klass_name: u8,
    children_pointer: u8,
}

impl Offsets {
    pub const fn new() -> &'static Self {
        &Self {
            scene_count: 0x18,
            active_scene: 0x48,
            dont_destroy_on_load_scene: 0x70,
            asset_path: 0x10,
            build_index: 0x98,
            root_storage_container: 0xB0,
            game_object: 0x30,
            game_object_name: 0x60,
            klass: 0x28,
            klass_name: 0x48,
            children_pointer: 0x70,
        }
    }
}

/// A scene loaded in the attached game.
pub struct Scene {
    address: u64,
}

impl Scene {
    /// Returns the address of the scene in the attached game.
    pub const fn address(&self) -> u64 {
        self.address
    }

    /// Returns [`true`] if the address of the scene still points to valid
    /// memory.
    pub fn is_valid(&self, process: &Process) -> bool {
        process.read::<u8>(self.address).is_ok()
    }

    /// Returns the build index of the scene. This index is unique to each
    /// scene in the game.
    pub fn index(
        &self,
        process: &Process,
        scene_manager: &SceneManager,
    ) -> Result<i32, MemoryError> {
        process.read(self.address + scene_manager.offsets.build_index as u64)
    }

    /// Returns the full path to the scene.
    pub fn path<const N: usize>(
        &self,
        process: &Process,
        scene_manager: &SceneManager,
    ) -> Result<ArrayCString<N>, MemoryError> {
        process.read_pointer_path(
            self.address,
            &[scene_manager.offsets.asset_path as u64, 0x0],
        )
    }
}

/// Returns the name of the scene from the given scene path. This path is
/// usually retrieved from [`SceneManager::get_current_scene_path`].
pub fn get_scene_name(scene_path: &[u8]) -> &[u8] {
    scene_path
        .rsplit(|&b| b == b'/')
        .next()
        .unwrap_or_default()
        .split(|&b| b == b'.')
        .next()
        .unwrap_or_default()
}
