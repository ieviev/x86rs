#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

use core::{ffi::CStr, mem::MaybeUninit};

mod sys;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {

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
