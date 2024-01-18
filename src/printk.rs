/**
 * AArch64_rust_os
 * A simple rust OS for AArch64
 * Github://github.com/lewisevans2007/AArch64_rust_os
 * By: Lewis Evans
*/

use core::ptr;

const UART: *mut u8 = 0x0900_0000 as *mut u8;

pub fn printk(s: &str) {
    for byte in s.as_bytes() {
        unsafe {
            ptr::write_volatile(UART, *byte);
        }
    }
}