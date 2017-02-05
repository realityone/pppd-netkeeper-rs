extern crate libc;
extern crate netkeeper;

use std::ffi::{CStr, CString};
use std::mem;

use netkeeper::common::dialer::Dialer;
use netkeeper::netkeeper::dialer::{NetkeeperDialer, Configuration};

const MAXNAMELEN: usize = 256;
const MAXSECRETLEN: usize = 256;

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static pppd_version: &'static [libc::c_uchar; 6] = b"2.4.4\x00";

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut user: [libc::c_char; MAXNAMELEN] = [0; MAXNAMELEN];

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut passwd: [libc::c_char; MAXSECRETLEN] = [0; MAXSECRETLEN];

pub extern "C" fn check() -> libc::c_int {
    1
}

pub extern "C" fn plugin_init(_: *const libc::c_void) {
    println!("Netkeeper Plugin Init");
    let origin_user;
    unsafe {
        origin_user = CStr::from_ptr(&user as *const i8);
    }

    let dialer = NetkeeperDialer::load_from_config(Configuration::Zhejiang);
    let encrypted = dialer.encrypt_account("05802278989@HYXY.XY", None);
    unsafe {
        let encrypted_user = CString::new(encrypted.clone()).unwrap();
        let i8_ptr: &[i8] = mem::transmute(encrypted_user);
        user[0..encrypted.len()].copy_from_slice(i8_ptr as &[i8]);
    }
    println!("{:?}", origin_user);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
