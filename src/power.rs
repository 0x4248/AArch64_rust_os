/**
 * AArch64_rust_os
 * A simple rust OS for AArch64
 * Github://github.com/lewisevans2007/AArch64_rust_os
 * By: Lewis Evans
*/

use core::arch::asm;

//use printk.rs

use crate::printk;


pub fn shutdown() {
    printk::printk("Shutting down\n");
    unsafe {
        asm!("bl system_off");
    }
}