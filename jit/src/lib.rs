//! Top-level lib.rs for `cranelift_jit`.

#![deny(
    missing_docs,
    trivial_numeric_casts,
    unused_extern_crates,
    unstable_features,
    unreachable_pub
)]
#![warn(unused_import_braces)]
#![cfg_attr(feature = "clippy", plugin(clippy(conf_file = "../../clippy.toml")))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::float_arithmetic,
        clippy::mut_mut,
        clippy::nonminimal_bool,
        clippy::map_unwrap_or,
        clippy::clippy::print_stdout,
        clippy::unicode_not_nfc,
        clippy::use_self
    )
)]
#![no_std]

mod backend;
mod compiled_blob;
mod memory;

extern crate alloc;

pub use crate::backend::{JITBuilder, JITModule};
use spin::{Mutex, MutexGuard};
use alloc::boxed::Box;
use lazy_static::lazy_static;
use cranelift_entity::__core::ffi::c_void;
use core::any::Any;

/// Version number of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");


lazy_static! {
    static ref MANAGER: Mutex<Box<dyn MemoryManager + Send>> = Mutex::new(Box::new(DummyManager));
}

fn mem_manage() -> MutexGuard<'static, Box<dyn MemoryManager + Send>> {
    MANAGER.lock()
}

/// Set the memory manager. See below.
/// Only call once.
pub fn set_manager(new_mgr: Box<dyn MemoryManager + Send>) {
    let mut manager = MANAGER.lock();
    assert_eq!((**manager).type_id(), DummyManager.type_id());
    *manager = new_mgr
}

/// Trait to be implemented by consumers, to then set their impl
/// as the memory manager.
pub trait MemoryManager {
    /// Returns the page size on the current platform
    fn page_size(&self) -> usize;
    /// Sets the pointer obtained from `alloc_page_aligned` as R only
    fn set_r(&mut self, ptr: *mut u8);
    /// Sets the pointer obtained from `alloc_page_aligned` as RX
    fn set_rx(&mut self, ptr: *mut u8);
    /// Sets the pointer obtained from `alloc_page_aligned` as RW
    fn set_rw(&mut self, ptr: *mut u8);
    /// Allocates a new page-aligned pointer of `size`, which should be a multiple of page size
    fn alloc_page_aligned(&mut self, size: usize) -> *mut u8;
    /// Deallocates pointer obtained from `alloc_page_aligned`
    fn dealloc(&mut self, ptr: *mut u8);
}

struct DummyManager;

impl MemoryManager for DummyManager {
    fn page_size(&self) -> usize {
        panic!()
    }

    fn set_r(&mut self, _ptr: *mut u8) {
        panic!()
    }

    fn set_rx(&mut self, _ptr: *mut u8) {
        panic!()
    }

    fn set_rw(&mut self, _ptr: *mut u8) {
        panic!()
    }

    fn alloc_page_aligned(&mut self, _size: usize) -> *mut u8 {
        panic!()
    }

    fn dealloc(&mut self, _ptr: *mut u8) {
        panic!()
    }
}