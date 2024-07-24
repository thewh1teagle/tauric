use std::{env, ffi::CString, path::PathBuf};

use tauric;

extern "C" fn ready_callback() {
    println!("Ready callback called!");
    let label = CString::new("main").expect("CString::new failed");
    let url = CString::new("mounted://index.html").expect("CString::new failed");
    let cwd = CString::new(env::current_dir().unwrap().to_str().unwrap()).unwrap();
    tauric::mount_frontend(cwd.as_ptr());
    tauric::create_window(label.as_ptr(), url.as_ptr());
    
}

fn main() {
    // Register the callback function
    tauric::on_ready(Some(ready_callback));
    tauric::run();
}