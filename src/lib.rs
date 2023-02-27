use std::sync::{Arc, Mutex};

pub type MyCallback = Option<unsafe extern "C" fn(a: i32, b: i32) -> i32>;

#[derive(Clone)]
struct PointerHolder {
    ptr: Arc<Mutex<dyn Send + Fn(i32, i32) -> i32>>,
}

#[no_mangle]
pub extern "C" fn my_function(a: MyCallback) -> bool {
    // let ptr = Arc::new(Mutex::new( a ));
    match a {
        Some(v) => { println!("Not null"); true },
        None => { println!("null"); false },
    }
}

fn main () { }
