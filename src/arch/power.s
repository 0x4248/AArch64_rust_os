/**
 * AArch64_rust_os
 * A simple rust OS for AArch64
 * Github://github.com/0x4248/AArch64_rust_os
 * By: 0x4248
*/

.equ PSCI_SYSTEM_OFF, 0x84000008
.equ PSCI_SYSTEM_RESET, 0x84000009
.globl system_off
.globl system_reset

// PSCI call to turn off the system.
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0

// PSCI call to reset the system.
system_reset:
    ldr     x0, =PSCI_SYSTEM_RESET
    hvc     #0