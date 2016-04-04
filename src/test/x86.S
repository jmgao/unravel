.globl x86_cfi
x86_cfi:
.cfi_startproc
  push $0
  .cfi_adjust_cfa_offset 4

  push %eax
  .cfi_def_cfa_offset 12
  .cfi_rel_offset eax, 0

  push %ebx
  .cfi_def_cfa_offset 16
  .cfi_rel_offset ebx, 0

  pop %ebx
  .cfi_adjust_cfa_offset -4
  .cfi_restore ebx

  pop %eax
  .cfi_adjust_cfa_offset -4
  .cfi_restore eax

  pop %eax
  .cfi_adjust_cfa_offset -4
  .cfi_undefined eax

  ret
.cfi_endproc
.size x86_cfi, .-x86_cfi
