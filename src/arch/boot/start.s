/**
 * AArch64_rust_os
 * A simple rust OS for AArch64
 * Github://github.com/0x4248/AArch64_rust_os
 * By: 0x4248
*/

.globl _start
.extern LD_STACK_PTR

.section ".text.boot"

// Main entry point for the kernel.
// This is the first function called by the bootloader.
_start:
    ldr     x30, =LD_STACK_PTR
    mov     sp, x30
    bl      start_kernel


