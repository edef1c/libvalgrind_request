pub type Value = u32;

#[inline(always)]
pub unsafe fn do_client_request(default: Value, args: &[Value; 6]) -> Value {
    let result;
    llvm_asm!("mov r12, r12, ror #3  ; mov r12, r12, ror #13
               mov r12, r12, ror #29 ; mov r12, r12, ror #19
               orr r10, r10, r10"
              : "={r3}" (result)
              : "{r4}" (args.as_ptr())
              "{r3}" (default)
              : "cc", "memory"
              : "volatile");
    result
}
