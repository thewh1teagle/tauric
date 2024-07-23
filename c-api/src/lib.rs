use std::{
    ffi::{c_char, CStr, CString},
    fs,
    path::PathBuf,
    str::FromStr,
    sync::Mutex,
};
use tauri::{AppHandle, Builder, Url, WebviewWindowBuilder};

type IPCCallbackFn = extern "C" fn(*const c_char);
type ReadyCallbackFn = extern "C" fn();

static APP_HANDLE: Mutex<Option<AppHandle>> = Mutex::new(None);
static IPC_CALLBACK: Mutex<Option<IPCCallbackFn>> = Mutex::new(None);
static READY_CALLBACK: Mutex<Option<ReadyCallbackFn>> = Mutex::new(None);
static FRONTEND_DIR: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
fn command(args: serde_json::Value) -> Result<(), String> {
    let args = serde_json::to_string_pretty(&args).unwrap();
    let callback = IPC_CALLBACK.lock().unwrap().clone();

    if let Some(callback) = callback {
        // Convert the string to a C string
        let c_str = CString::new(args).map_err(|e| e.to_string())?;
        // Call the callback function
        callback(c_str.as_ptr());
    } else {
        return Err("Please register command callback using register_commands()".into());
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn on_command(callback: Option<extern "C" fn(*const c_char)>) {
    // Store the callback function in the global variable
    *IPC_CALLBACK.lock().unwrap() = callback;
}

#[no_mangle]
pub extern "C" fn on_ready(callback: Option<extern "C" fn()>) {
    // Store the callback function in the global variable
    *READY_CALLBACK.lock().unwrap() = callback;
}

#[no_mangle]
pub extern "C" fn mount_frontend(path: *const c_char) {
    // Store the callback function in the global variable
    println!("mount frontend");
    let path = unsafe { CStr::from_ptr(path).to_str().unwrap().to_owned() };
    *FRONTEND_DIR.lock().unwrap() = Some(path);
}

#[no_mangle]
pub extern "C" fn create_window(label: *const c_char, url: *const c_char) {
    let label = unsafe {
        assert!(!label.is_null());
        CStr::from_ptr(label).to_str().unwrap().to_owned()
    };

    let url = unsafe {
        assert!(!url.is_null());
        CStr::from_ptr(url).to_str().unwrap().to_owned()
    };
    let app_handle = APP_HANDLE.lock().unwrap().clone().unwrap();
    WebviewWindowBuilder::new(
        &app_handle,
        label,
        tauri::WebviewUrl::External(Url::from_str(&url).unwrap()),
    )
    .inner_size(800.0, 600.0)
    .visible(true)
    .initialization_script("window.addEventListener('DOMContentLoaded', () => { window.invoke = window.__TAURI__.core.invoke; });")
    .build()
    .unwrap();
}

#[no_mangle]
pub extern "C" fn close() {
    let app_handle = APP_HANDLE.lock().unwrap().clone().unwrap();
    app_handle.exit(0);
}

#[no_mangle]
pub extern "C" fn run() -> i32 {
    ctrlc::set_handler(move || {
        close();
    })
    .unwrap();
    let result = Builder::default()
        .register_uri_scheme_protocol("local", |app, request| {
            println!("local request");
            let front_dir = FRONTEND_DIR.lock().unwrap();
            let front_dir_opt = front_dir.as_ref();
            tauri::http::Response::builder()
            .status(200)
                .body("<html><h1>Hello world!</h1><button>click</button><script>document.querySelector('button').onclick = () => { invoke('command', {args: {'hello': 'world'}}) }</script></html>".to_string().into_bytes())
            .unwrap()
        })
        .setup(|app| {
            
            let mut app_handle = APP_HANDLE.lock().unwrap();
            *app_handle = Some(app.handle().clone());

            tauri::async_runtime::spawn(async {
                let ready_callback = READY_CALLBACK.lock().unwrap().clone();
                if let Some(callback) = ready_callback {
                    callback();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command])
        .run(tauri::generate_context!());

    // Check if the application started successfully
    match result {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Failure
    }
}
