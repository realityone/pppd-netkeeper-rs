extern crate libc;
extern crate netkeeper;

use std::ffi::CStr;
use netkeeper::netkeeper::dialer::{NetkeeperDialer, Configuration};
use netkeeper::common::dialer::Dialer;

extern "C" {
    static mut user: [libc::c_schar; 256];
}

#[no_mangle]
pub extern "C" fn plugin_init() {
    println!("Netkeeper Plugin Init");

    let origin;
    unsafe {
        origin = CStr::from_ptr(&user as *const i8).to_string_lossy().into_owned();
    }
    println!("Origin username is: {:?}", origin);

    let dialer = NetkeeperDialer::load_from_config(Configuration::Zhejiang);
    let encrypted = dialer.encrypt_account(&origin, None);

    println!("Encrypted username is: {:?}", encrypted);
    unsafe {
        let encrypted_bytes = encrypted.as_bytes().iter().map(|c| *c as i8).collect::<Vec<i8>>();
        user[0..encrypted.len()].copy_from_slice(&encrypted_bytes);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
