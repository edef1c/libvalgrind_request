#![feature(asm)]
#![no_std]
pub use abi::Vg_ClientRequest;
pub use abi::Vg_ClientRequest::*;
pub use arch::*;

mod arch;
mod abi;

#[inline]
pub fn running_on_valgrind() -> Value {
  unsafe {
    do_client_request(0, &[VG_USERREQ__RUNNING_ON_VALGRIND as Value, 0, 0, 0, 0, 0])
  }
}

#[inline]
pub fn discard_translations(addr: *const u8, len: usize) -> Value {
  unsafe {
    do_client_request(0, &[VG_USERREQ__DISCARD_TRANSLATIONS as Value,
                          addr as usize as Value,
                          len as Value,
                          0, 0, 0])
  }
}

#[inline]
pub fn count_errors() -> Value {
  unsafe {
    do_client_request(0, &[VG_USERREQ__COUNT_ERRORS as Value, 0, 0, 0, 0, 0])
  }
}

#[inline]
pub fn stack_register(start: *const u8, end: *const u8) -> Value {
  unsafe {
    do_client_request(0, &[VG_USERREQ__STACK_REGISTER as Value,
                          start as usize as Value,
                          end   as usize as Value,
                          0, 0, 0])
  }
}

#[inline]
pub fn stack_change(id: Value, start: *const u8, end: *const u8) -> Value {
  unsafe {
    do_client_request(0, &[VG_USERREQ__STACK_CHANGE as Value,
                          id,
                          start as usize as Value,
                          end   as usize as Value,
                          0, 0])
  }
}

#[inline]
pub fn stack_deregister(id: Value) {
  unsafe {
    do_client_request(0, &[VG_USERREQ__STACK_DEREGISTER as Value,
                          id,
                          0, 0, 0, 0]);
  }
}
