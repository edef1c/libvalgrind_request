pub type Value = u32;

#[inline(always)]
pub unsafe fn do_client_request(default: Value, args: &[Value; 6]) -> Value {
  let result;
  asm!("roll $$3,  %edi ; roll $$13, %edi
        roll $$29, %edi ; roll $$19, %edi
        xchgl %ebx, %ebx"
      : "={edx}" (result)
      : "{eax}" (args.as_ptr())
        "{edx}" (default)
      : "cc", "memory"
      : "volatile");
  result
}
