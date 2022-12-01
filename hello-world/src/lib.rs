#![no_std]
use gstd::{msg, prelude::*};

#[no_mangle]
unsafe extern "C" fn handle() {
    msg::reply("Hello", 0).expect("Error in sending a reply message");
}
