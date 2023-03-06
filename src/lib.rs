use std::sync::{Arc, Mutex};
use std::ffi::CString;
use libc::{c_char, c_int};

pub type EventCallback = Option<unsafe extern "C" fn(size: i32) -> i32>;

// rust <--> C interop handle
#[derive(Clone)]
pub struct FFIDataHandle {
    ptr: Arc<Mutex<EventCallback>>,
    data: Vec<String>,
}

#[no_mangle]
pub extern "C" fn ffi_get_string(ptr: *mut FFIDataHandle, out: *mut u8) -> bool {
    if ptr == std::ptr::null_mut() {
        return false;
    }
    let s = "Hello, world!";
    let bytes = s.as_bytes();
    let len = bytes.len();
    unsafe {
        std::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            out,
            len
        );
        std::ptr::write(out.add(len), 0);
    }

    true
}

#[no_mangle]
pub extern "C" fn ffi_lib_init(a: EventCallback) -> *mut FFIDataHandle {
    let ph = FFIDataHandle {
        ptr: Arc::new(Mutex::new(a)),
        data: vec![],
    };
    Box::into_raw(Box::new(ph))
}

#[no_mangle]
pub extern "C" fn poller(ffi_handle: Option<Box<FFIDataHandle>>) {
    let d = match ffi_handle {
        Some(s) => *s,
        None => { println!("Pointer was null"); return }
    };
    let dt = d.clone();
    std::thread::spawn(move || {
        loop {
            let binding = dt.ptr.lock().expect("Couldn't lock FFI handle");
            let closure_ref = binding.as_ref().expect("FFI callback null pointer");
            let s = "Hello, world!";
            let bytes = s.as_bytes();
            let len = bytes.len();
            let result = unsafe { closure_ref(len as i32) };
            drop(binding);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }).join().unwrap();
}
