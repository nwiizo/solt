extern crate libc;
//use std::path::Path;
//use libc::c_int;

type c_int = i32;
type c_char = i8;

#[link(name = "chroot")]
extern {
//fn snappy_max_compressed_length(source_length: size_t) -> size_t;
    fn chroot(name: c_char) -> c_int;
}

pub fn sio(name: c_char) {
    let x = unsafe { chroot(name) };
    println!("chroot: {}",x);
}
