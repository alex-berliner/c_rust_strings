use std::sync::{Arc, Mutex};

/*
init thread
init pointer holder
send pointer holder to thread
store pointer holder
*/

pub type MyCallback = Option<unsafe extern "C" fn(a: usize, b: usize)>;

#[derive(Clone)]
struct PointerHolder {
    ptr: Arc<Mutex<dyn Send + Fn(i32, i32) -> i32>>,
}

#[no_mangle]
pub extern "C" fn my_function(a: MyCallback) {}
