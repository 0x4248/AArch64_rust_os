/**
 * aarch64_rust_os
 *
**/
.globl _start
.extern LD_STACK_PTR

.section ".text.boot"

// Main entry point for the kernel.
// This is the first function called by the bootloader.
_start:
    ldr     x30, =LD_STACK_PTR
    mov     sp, x30
    bl      start_kernel

.equ PSCI_SYSTEM_OFF, 0x84000008
.globl system_off

// PSCI call to turn off the system.
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0
