#![feature(asm)]
#![no_std]
pub use abi::ClientRequest;
pub use arch::*;

mod arch;
mod abi;

#[inline]
pub fn running_on_valgrind() -> Value {
  unsafe {
    do_client_request(0, &[ClientRequest::RunningOnValgrind as Value, 0, 0, 0, 0, 0])
  }
}

#[inline]
pub fn discard_translations(addr: *const u8, len: usize) -> Value {
  unsafe {
    do_client_request(0, &[ClientRequest::DiscardTranslations as Value,
                          addr as usize as Value,
                          len as Value,
                          0, 0, 0])
  }
}

#[inline]
pub fn count_errors() -> Value {
  unsafe {
    do_client_request(0, &[ClientRequest::CountErrors as Value, 0, 0, 0, 0, 0])
  }
}

#[inline]
pub fn stack_register(start: *const u8, end: *const u8) -> Value {
  unsafe {
    do_client_request(0, &[ClientRequest::StackRegister as Value,
                          start as usize as Value,
                          end   as usize as Value,
                          0, 0, 0])
  }
}

#[inline]
pub fn stack_change(id: Value, start: *const u8, end: *const u8) -> Value {
  unsafe {
    do_client_request(0, &[ClientRequest::StackChange as Value,
                          id,
                          start as usize as Value,
                          end   as usize as Value,
                          0, 0])
  }
}

#[inline]
pub fn stack_deregister(id: Value) {
  unsafe {
    do_client_request(0, &[ClientRequest::StackDeregister as Value,
                          id,
                          0, 0, 0, 0]);
  }
}
