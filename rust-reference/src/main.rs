#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![feature(naked_functions_rustic_abi)]
#![allow(unused)]

use core::{
    arch::{asm, naked_asm, x86_64::_mm_test_all_ones},
    ffi::CStr,
    mem::MaybeUninit,
    ops::Sub,
};

use crate::sys::{dbg, dbg_isize, exit};

mod sys;

unsafe fn get_args() -> isize {
    let rsp: *const isize;
    let argc: isize;
    asm!(
        "mov {0}, rsp",
        "mov {1}, [rsp - 8]",
        out(reg) rsp,
        out(reg) argc,
    );
    (argc)
}

fn get_rsp() -> *const isize {
    let mut argc;
    unsafe {
        asm!(
            "
            mov rax, rsp
            ",
            out("rax") argc,
            options(nostack, nomem, preserves_flags,),
        );
    }
    argc
}

unsafe fn main(argc: isize, argv: *const isize) {
    dbg(b"argc: ");
    dbg_isize(argc);
    dbg(b"\n");
    dbg(b"arg0: ");
    sys::write_cstr(1, *argv.offset(0));
    dbg(b"\n");
    dbg(b"arg1: ");
    sys::write_cstr(1, *argv.offset(1));
    dbg(b"\n");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        // there is no way to remove .cfi sections from output in rust
        // so the stack pointer may point to whereever and i have no guarantees
        // unless you strip the binary
        let argv: *const *const u8;
        let rsp = get_rsp();
        let argc = *(rsp.offset(1));
        let argv = (rsp.offset(2));
        main(argc, argv);

        // if argc == 1 {
        //     exit(1);
        // }
        // if argc == 2 {
        //     exit(2);
        // }
        // if argc == 3 {
        //     exit(3);
        // } else {
        //     exit(1);
        // }

        // dbg_isize((argc) as isize);
        // dbg(b", ");
        // dbg(b"\n");

        // // let mut buffer: [_; 8192] = MaybeUninit::uninit().assume_init();
        // // let n_read = sys::read(0, buffer.as_mut_ptr(), buffer.len());
        // // let outbuf = b"hello world!\n";
        // // sys::write_buf(1, outbuf);
        // // let _ = sys::write(1, buffer.as_ptr() as _, n_read as _);
        // sys::write_buf(1, b"starting read!\n");
        // // let path = b"/\0";
        // let path = b"/home/ian/f\0";
        // // O_NOATIME | O_DIRECTORY
        // let fd = sys::open(path.as_ptr(), 0x1000000 | 0x2000000, 0);
        // if fd < 0 {
        //     sys::exit(1);
        // }

        // sys::write_buf(1, b"open ok!\n");

        // // let (vbuf, vlen) = int_to_str_stack_buf(-1);
        // // sys::write_buf(1, &vbuf[..vlen]);

        // let mut buf = [MaybeUninit::<u8>::uninit(); 1024];
        // let nread = sys::getdents64(fd, buf.as_mut_ptr(), buf.len());
        // if nread < 0 {
        //     sys::close(fd);
        //     sys::write_buf(1, b"failed to open file!\n");
        //     let num_entries = sys::itoa(nread);
        //     sys::write_buf(1, num_entries.as_slice());
        //     sys::exit(1);
        // } else {
        //     sys::write_buf(1, b"bytes_read: ");

        //     let num_entries = sys::itoa(nread);
        //     sys::write_buf(1, num_entries.as_slice());
        //     sys::write_buf(1, b"\n");
        //     let mut bpos: isize = 0;
        //     if nread == 48 {
        //         sys::write_buf(1, b"empty!\n");
        //     }
        //     while bpos < nread {
        //         let d = (buf.as_ptr().offset(bpos) as *const sys::linux_dirent64)
        //             .as_ref()
        //             .unwrap();
        //         bpos = bpos + d.d_reclen as isize;
        //         let dname_ptr = d.d_name.as_ptr();
        //         let cstr = CStr::from_ptr(d.d_name.as_ptr() as _);
        //         if *dname_ptr == '.' as u8
        //             && (*dname_ptr.add(1) == '\0' as u8 || *dname_ptr.add(1) == '.' as u8)
        //         {
        //         } else {
        //             sys::write_buf(1, cstr.to_bytes());
        //             sys::write_buf(1, b"\n");
        //         }
        //     }
        // }
        // sys::write_buf(1, b"noerr!\n");

        // // sys::write_buf(1, b"read ok!\n");

        // sys::close(fd);

        sys::exit(0)
    }
}
