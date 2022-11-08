#![crate_type = "staticlib"]

use std::mem;

#[repr(C)]
pub struct Slice {
  ptr: *mut i32,
  len: usize,
}

fn vec_return() -> Vec<i32> {
  vec![11, 13, 17, 19, 23, 29]
}

#[no_mangle]
pub extern fn wrapper() -> Slice {
  let mut v = vec_return();
  let p = v.as_mut_ptr();
  let len = v.len();

  //unsafe {
    // so that no destructor is run on vector v
    mem::forget(v);
  //}

  Slice {ptr: p, len: len}
}
