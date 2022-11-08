#![crate_type = "staticlib"]


// #[no_mange] lets us find the name in the symbol table
// extern make the function externally visible
#[no_mangle]
pub extern "C" fn square(x: i32) -> i32 {
  x * x
}
