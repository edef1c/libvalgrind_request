pub type Value = u64;

#[inline(always)]
pub unsafe fn do_client_request(default: Value, args: &[Value; 6]) -> Value {
  let result;
  asm!("rolq $$3,  %rdi ; rolq $$13, %rdi
        rolq $$61, %rdi ; rolq $$51, %rdi
        xchgq %rbx, %rbx"
      : "={rdx}" (result)
      : "{rax}" (args.as_ptr())
        "{rdx}" (default)
      : "cc", "memory"
      : "volatile");
  result
}
