use std::{env, ffi::CString, ptr::null};

use libtauri;

extern "C" fn ready_callback() {
    println!("Ready callback called!");
    let label = CString::new("main").unwrap();
    let title = CString::new("libtauri").unwrap();
    let url = CString::new("mounted://index.html").unwrap();
    let cwd = CString::new(env::current_dir().unwrap().to_str().unwrap()).unwrap();
    libtauri::mount_frontend(cwd.as_ptr());
    libtauri::create_window(label.as_ptr(), title.as_ptr(),url.as_ptr());
    
}

fn main() {
    // Register the callback function
    libtauri::on_ready(Some(ready_callback));
    let identifier = CString::new("com.libtauri.dev").unwrap();
    let product_name = CString::new("libtauri").unwrap();
    libtauri::run(identifier.as_ptr(), product_name.as_ptr(), null());
}