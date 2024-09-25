pub mod il2cpp;

// mod scene;
// pub use self::scene::*;

fn value_from_string(value: &str) -> Option<u32> {
    if let Some(rem) = value.strip_prefix("0x") {
        u32::from_str_radix(rem, 16).ok()
    } else {
        value.parse().ok()
    }
}
