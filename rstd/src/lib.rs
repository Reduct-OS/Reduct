#![no_std]
#![no_main]
#![allow(asm_sub_register)]
#![feature(macro_metavar_expr)]

use core::usize;

pub extern crate alloc;

#[macro_use]
pub mod macros;
pub mod dma;
pub mod fs;
pub mod mm;
pub mod proc;
pub mod stat;
pub mod stdio;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    proc::exit(usize::MAX);
    loop {}
}

pub fn addr_of<T>(reffer: &T) -> usize {
    reffer as *const T as usize
}

pub fn ref_to_mut<T>(reffer: &T) -> &mut T {
    unsafe { &mut *(addr_of(reffer) as *const T as *mut T) }
}

pub fn ref_to_static<T>(reffer: &T) -> &'static T {
    unsafe { &*(addr_of(reffer) as *const T) }
}

#[macro_export]
macro_rules! unsafe_trait_impl {
    ($struct: ident, $trait: ident) => {
        unsafe impl $trait for $struct {}
    };
    ($struct: ident, $trait: ident, $life: tt) => {
        unsafe impl<$life> $trait for $struct<$life> {}
    };
}
