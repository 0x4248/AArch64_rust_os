#![no_std]
#![no_main]
/**
 * AArch64_rust_os
 * A simple rust OS for AArch64
 * Github://github.com/0x4248/AArch64_rust_os
 * By: 0x4248
*/

use core::arch::global_asm;
use core::ptr;

mod printk;
mod power;

global_asm!(include_str!("arch/boot/start.s"));

const UART: *mut u8 = 0x0900_0000 as *mut u8;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    printk::printk("Kernel Panic was called!\n");
    power::shutdown();
    loop {}
}

#[no_mangle]
pub extern "C" fn start_kernel() {
    printk::printk("AArch64 OS!\n");
    printk::printk("Press any key to exit:");
    loop {
        unsafe {
            let byte = ptr::read_volatile(UART);
            if byte == 0x0D {
                break;
            }
        }
    }
    printk::printk("\n");
    power::shutdown();
}
