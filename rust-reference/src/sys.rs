#![allow(dead_code)]
use core::{arch::asm, fmt::Arguments, mem::MaybeUninit, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { exit(1) }
}

pub unsafe fn exit(code: isize) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("eax") 60,
            in("edi") code,
            options(nostack, nomem, preserves_flags, noreturn),
        );
    }
}

/// ret: num_written
pub unsafe fn write(fd: usize, buf: *const MaybeUninit<u8>, count: usize) -> isize {
    unsafe {
        let mut ret: isize = 1;
        asm!(
            "syscall",
            inout("rax") ret,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            options(nostack, nomem, preserves_flags,),
        );
        ret
    }
}

/// ret: num_read
pub unsafe fn read(fd: usize, buf: *mut MaybeUninit<u8>, count: usize) -> isize {
    unsafe {
        let mut ret: isize = 0;
        asm!(
            "syscall",
            inout("rax") ret,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            options(nostack, nomem, preserves_flags,),
        );
        ret
    }
}

// https://linux.die.net/man/2/openat
// AT_FDCWD: pathname is interpreted relative to the current working directory of the calling process
pub unsafe fn open(path: *const u8, flags: i32, mode: u16) -> isize {
    unsafe {
        let mut ret: isize = 0x101;
        asm!(
            "syscall",
            inout("rax") ret,
            in("rdi") -100isize,         // AT_FDCWD
            in("rsi") path,
            in("rdx") flags,
            in("r10") mode,
            options(nostack, nomem, preserves_flags),
        );
        ret
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct linux_dirent64 {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [u8; 0],
}

pub unsafe fn getdents64(fd: isize, dirp: *mut MaybeUninit<u8>, count: usize) -> isize {
    unsafe {
        let mut ret: isize;
        asm!(
            "syscall",
            in("rax") 217,
            in("rdi") fd,
            in("rsi") dirp,
            in("rdx") count,
            lateout("rax") ret,
            options(nostack, nomem, preserves_flags),
        );
        ret
    }
}

pub unsafe fn close(fd: isize) -> isize {
    unsafe {
        let mut ret: isize;
        asm!(
            "syscall",
            in("rax") 3,
            in("rdi") fd,
            lateout("rax") ret,
            options(nostack, nomem, preserves_flags),
        );
        ret
    }
}

pub unsafe fn write_slice(fd: usize, buf: &[u8]) -> isize {
    unsafe { write(fd, buf.as_ptr() as _, buf.len()) }
}

pub unsafe fn dbg(s: &[u8]) -> isize {
    unsafe { write_slice(1, s) }
}
pub unsafe fn dbg_isize(i: isize) -> isize {
    unsafe { write_slice(1, itoa(i).as_slice()) }
}

pub struct ItoaResult {
    buf: [u8; 20],
    len: usize,
}

impl ItoaResult {
    pub fn new() -> Self {
        Self {
            buf: [0u8; 20],
            len: 0,
        }
    }

    fn push(&mut self, c: u8) {
        self.len += 1;
        let pos: *mut u8 = unsafe { self.buf.as_mut_ptr().add(20usize - self.len) };
        unsafe { *pos = c };
        // self.buf[20usize - self.len] = c;
    }

    // pub fn as_str(&self) -> &str {
    //     core::str::from_utf8(&self.buf[20usize.checked_sub(self.len).unwrap_or(0)..])
    //         .unwrap_or("failed")
    // }

    pub fn as_slice(&self) -> &[u8] {
        b"abcd"
        // &self.buf[20usize.checked_sub(self.len).unwrap_or(0)..]
    }
}

// without inline segfaults on dbg
#[inline(always)]
pub fn itoa(mut value: isize) -> ItoaResult {
    let mut result = ItoaResult::new();

    let neg = value < 0;

    loop {
        result.push(b'0' + u8::try_from((value % 10).abs()).unwrap_or(1));
        value /= 10;
        if value == 0 {
            break;
        }
    }

    if neg {
        result.push(b'-');
    }

    result
}

// https://github.com/rust-lang/rust/blob/edb368491551a77d77a48446d4ee88b35490c565/src/libpanic_unwind/gcc.rs#L282
#[cfg(debug_assertions)]
#[unsafe(no_mangle)]
unsafe extern "C" fn rust_eh_personality(
    exceptionRecord: usize,
    establisherFrame: usize,
    contextRecord: usize,
    dispatcherContext: usize,
) -> ! {
    exit(1)
}
