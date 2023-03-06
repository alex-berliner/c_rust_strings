use std::sync::{Arc, Mutex};

pub type EventCallback = Option<unsafe extern "C" fn(a: i32, b: i32) -> i32>;

#[derive(Clone)]
pub struct FFIDataHandle {
    ptr: Arc<Mutex<EventCallback>>,
}

#[no_mangle]
pub extern "C" fn reg_event_callback(a: EventCallback) -> Box<FFIDataHandle> {
    let ph = FFIDataHandle {
        ptr: Arc::new(Mutex::new(a)),
    };
    Box::new(ph)
}

#[no_mangle]
pub extern "C" fn poller() {
    std::thread::spawn(move || {
        loop {
            println!("polling");
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }).join().unwrap();
}
