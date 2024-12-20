use std::{
    env,
    ffi::{c_char, CStr, CString},
    path::PathBuf,
    ptr::null,
};

use serde_json::Value;
use tauric;

#[no_mangle]
extern "C" fn OnReady() {
    let label = CString::new("main").unwrap();
    let title = CString::new("tauric").unwrap();
    let url = CString::new("fs://index.html").unwrap();
    tauric::TauricCreateWindow(
        label.as_ptr(),
        title.as_ptr(),
        url.as_ptr(),
        null(),
        1200,
        1200,
        0,
        1,
    );
}

#[no_mangle]
extern "C" fn OnCommand(message: *const c_char) -> *const c_char {
    let message_c = unsafe { CStr::from_ptr(message) };
    let message = message_c.to_str().unwrap();
    let message: Value = serde_json::from_str(message).unwrap();
    println!("Received: {:?}", message);
    let message = serde_json::json!({"message": "Hello from Rust!"}).to_string();
    let cstring = CString::new(message).unwrap();
    println!("cstring: {:?}", cstring);
    cstring.into_raw()
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let identifier = CString::new("com.tauric.dev").unwrap();
    let product_name = CString::new("tauric").unwrap();
    let dist_dir = PathBuf::from(manifest_dir)
        .join("../examples/dist")
        .canonicalize()
        .unwrap();
    let dist_dir = CString::new(dist_dir.to_str().unwrap().to_owned()).unwrap();
    tauric::TauricMountFrontend(dist_dir.as_ptr());
    tauric::TauricOnCommand(OnCommand);
    tauric::TauricRun(
        identifier.as_ptr(),
        product_name.as_ptr(),
        null(),
        Some(OnReady),
    );
}
