pub type Value = u64;

#[inline(always)]
pub unsafe fn do_client_request(default: Value, args: &[Value; 6]) -> Value {
  let result;
  llvm_asm!("ror x12, x12, #3  ;  ror x12, x12, #13
        ror x12, x12, #51 ;  ror x12, x12, #61
        orr x10, x10, x10"
      : "={x3}" (result)
      : "{x4}" (args.as_ptr())
        "{x3}" (default)
      : "cc", "memory"
      : "volatile");
  result
}
