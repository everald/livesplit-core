// TODO Don't allow unreachable_pub but that throws false positives on the wasm
// platform.
#![allow(unreachable_pub)]

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
mod wasm;
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub use self::wasm::*;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
mod normal;
#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
crate use self::normal::*;
