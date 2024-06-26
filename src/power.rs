/**
 * AArch64_rust_os
 * A simple rust OS for AArch64
 * Github://github.com/0x4248/AArch64_rust_os
 * By: 0x4248
*/

use core::arch::global_asm;
use core::arch::asm;


//use printk.rs

use crate::printk;

global_asm!(include_str!("arch/power.s"));

pub fn shutdown() {
    printk::printk("Shutting down\n");
    unsafe {
        asm!("bl system_off");
    }
}

pub fn reboot() {
    printk::printk("Rebooting\n");
    unsafe {
        asm!("bl system_reset");
    }
}