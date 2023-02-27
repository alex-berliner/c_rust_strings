use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct PointerHolder {
    ptr: Arc<Mutex<dyn Send + Fn(i32, i32) -> i32>>,
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn c_func(f: fn(i32, i32) -> i32) {
    let p = PointerHolder {
        ptr: Arc::new(Mutex::new(f)),
    };
    todo!()
}

fn oldmain() {
    let add_ptr = add as fn(i32, i32) -> i32;
    let ph = PointerHolder {
        ptr: Arc::new(Mutex::new(add_ptr)),
    };
    let mut t_handles = vec![];
    for i in 0..10 {
        let pc = ph.clone();
        let handle = std::thread::spawn(move || {
            let p = pc.ptr;
            let add_ptr_value = p.lock().unwrap();
            let result = add_ptr_value(i, i*10);
            println!("result: {:?}", result);
        });
        t_handles.push(handle);
    }

    for h in t_handles {
        h.join();
    }
}
