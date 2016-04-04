.globl arm_cfi
arm_cfi:
.cfi_startproc
  push {lr}
  .cfi_adjust_cfa_offset 4
  .cfi_rel_offset lr, 0
  
  push {r5}
  .cfi_adjust_cfa_offset 4
  .cfi_rel_offset r5, 0

  mov r5, #0
  str r5, [sp, #0]
  .cfi_undefined r5
  
  pop {r5, pc}
.cfi_endproc
.size arm_cfi, .-arm_cfi
