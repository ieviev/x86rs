#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

use core::{ffi::CStr, mem::MaybeUninit};
mod sys;

#[unsafe(no_mangle)]
unsafe fn hello() {
    let outbuf = b"hello world!\n";
    sys::dbg(outbuf);
    unsafe { sys::write_slice(1, outbuf) };
}
