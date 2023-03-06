use std::sync::{Arc, Mutex};

pub type EventCallback = Option<unsafe extern "C" fn(a: i32, b: i32) -> i32>;

#[derive(Clone)]
pub struct FFIDataHandle {
    ptr: Arc<Mutex<EventCallback>>,
}

#[no_mangle]
pub extern "C" fn lib_init(a: EventCallback) -> Box<FFIDataHandle> {
    let ph = FFIDataHandle {
        ptr: Arc::new(Mutex::new(a)),
    };
    Box::new(ph)
}

#[no_mangle]
pub extern "C" fn poller(h: Option<Box<FFIDataHandle>>) {
    let d = match h {
        Some(s) => *s,
        None => { println!("Pointer was null"); return }
    };
    let dt = d.clone();
    std::thread::spawn(move || {
        let binding = dt.ptr.lock().expect("Couldn't lock");
        let closure_ref = binding.as_ref().expect("Null pointer");
        loop {
            let result = unsafe { closure_ref(1, 2) };
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }).join().unwrap();
}
