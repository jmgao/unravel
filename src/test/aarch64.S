.globl aarch64_cfi
aarch64_cfi:
.cfi_startproc
  stp  x29, x30, [sp, -16]!
  .cfi_adjust_cfa_offset 16
  .cfi_rel_offset x29, 0
  .cfi_rel_offset x30, 8

  str xzr, [sp, 0]
  .cfi_undefined x29

  ldp x29, x30, [sp], 16
  .cfi_adjust_cfa_offset -16

  ret
.cfi_endproc
.size aarch64_cfi, .-aarch64_cfi
