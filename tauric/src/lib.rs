use std::{
    ffi::{c_char, CStr, CString}, fs, path::PathBuf, str::FromStr, sync::Mutex
};
use serde_json::Value;
use tauri::{utils::config::TrayIconConfig, AppHandle, Builder, Url, WebviewWindowBuilder};

type IPCCallbackFn = extern "C" fn(*const c_char) -> *const c_char;
type ReadyCallbackFn = extern "C" fn();

static APP_HANDLE: Mutex<Option<AppHandle>> = Mutex::new(None);
static IPC_CALLBACK: Mutex<Option<IPCCallbackFn>> = Mutex::new(None);
static READY_CALLBACK: Mutex<Option<ReadyCallbackFn>> = Mutex::new(None);
static FRONTEND_DIR: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
fn command(args: serde_json::Value) -> Result<Option<Value>, String> {
    let args = serde_json::to_string_pretty(&args).unwrap();
    let callback = IPC_CALLBACK.lock().unwrap().clone();

    if let Some(callback) = callback {
        // Convert the string to a C string
        let c_str = CString::new(args).map_err(|e| e.to_string())?;
        // Call the callback function
        let resp = callback(c_str.as_ptr());
        
        if !resp.is_null() {
            let c_str_resp = unsafe { CStr::from_ptr(resp) };
            
            let resp_str = c_str_resp.to_str().map_err(|e| e.to_string())?;

            // Try to deserialize the string into serde_json::Value
            let deserialized: Value = serde_json::from_str(resp_str).map_err(|e| e.to_string())?;
            return Ok(Some(deserialized));
        }
    } else {
        return Err("Please register command callback using register_commands()".into());
    }
    Ok(None)
}

#[no_mangle]
pub extern "C" fn TauricOnCommand(callback: IPCCallbackFn) {
    // Store the callback function in the global variable
    *IPC_CALLBACK.lock().unwrap() = Some(callback);
}

#[no_mangle]
pub extern "C" fn TauricOnReady(callback: Option<extern "C" fn()>) {
    // Store the callback function in the global variable
    *READY_CALLBACK.lock().unwrap() = callback;
}

#[no_mangle]
pub extern "C" fn TauricMountFrontend(path: *const c_char) {
    // Store the callback function in the global variable
    let path = unsafe { CStr::from_ptr(path).to_str().unwrap().to_owned() };
    *FRONTEND_DIR.lock().unwrap() = Some(path);
}

#[no_mangle]
pub extern "C" fn TauricCreateWindow(label: *const c_char, title: *const c_char, url: *const c_char) {
    let label = unsafe {
        assert!(!label.is_null());
        CStr::from_ptr(label).to_str().unwrap().to_owned()
    };

    let url = unsafe {
        assert!(!url.is_null());
        CStr::from_ptr(url).to_str().unwrap().to_owned()
    };
    let title = unsafe {
        assert!(!title.is_null());
        CStr::from_ptr(title).to_str().unwrap().to_owned()
    };
    let app_handle = APP_HANDLE.lock().unwrap().clone().unwrap();
    WebviewWindowBuilder::new(
        &app_handle,
        label,
        tauri::WebviewUrl::External(Url::from_str(&url).unwrap()),
    )
    .title(title)
    .inner_size(800.0, 600.0)
    .visible(true)
    .initialization_script("window.invoke = (args) => window.__TAURI__.core.invoke('command', {args: args})")
    .build()
    .unwrap();
}

#[no_mangle]
pub extern "C" fn TauricClose() {
    if let Some(app_handle) = APP_HANDLE.lock().unwrap().clone() {
        app_handle.exit(0);
    }    
}

#[no_mangle]
pub extern "C" fn TauricRun(identifier: *const c_char, product_name: *const c_char, icon_path: *const c_char, on_ready: Option<extern "C" fn()>) -> i32 {
    ctrlc::set_handler(move || {
        TauricClose();
    })
    .unwrap();
    let mut context = tauri::generate_context!();
    let config = context.config_mut();
    let identifier = unsafe {
        assert!(!identifier.is_null());
        CStr::from_ptr(identifier).to_str().unwrap().to_owned()
    };
    let product_name = unsafe {
        assert!(!product_name.is_null());
        CStr::from_ptr(product_name).to_str().unwrap().to_owned()
    };
    config.identifier = identifier;
    config.product_name = Some(product_name);
    config.app.with_global_tauri = true;
    if !icon_path.is_null() {
        
        let icon_path = unsafe {
            CStr::from_ptr(icon_path).to_str().unwrap().to_owned()
        };
        
        config.bundle.icon = vec![icon_path.clone()];
        config.app.tray_icon = Some(TrayIconConfig{icon_path: PathBuf::from(icon_path), ..Default::default()});
    }
    
    
    let result = Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .register_uri_scheme_protocol("fs", |_app, request| {
            let front_dir = FRONTEND_DIR.lock().unwrap();
            let front_dir_opt = front_dir.as_ref().unwrap();
            let mut request_path = request.uri().path();
            if request_path == "/" {
                request_path = "index.html";
            } else if request_path.starts_with('/') {
                request_path = request_path.strip_prefix("/").unwrap(); // Remove the leading '/'
            }
            let path = std::path::PathBuf::from(front_dir_opt).join(request_path);

            if path.exists() {
                let content = fs::read(path).unwrap();
                let result = tauri::http::Response::builder()
                    .status(200)
                    .body(content)
                    .unwrap();
                return result;
            } else {
                let result = tauri::http::Response::builder()
                    .status(404)
                    .body(Vec::new())
                    .unwrap();
                return result;
            }
        })
        .setup(move |app| {
            let mut app_handle = APP_HANDLE.lock().unwrap();
            
            *app_handle = Some(app.handle().clone());

            let on_ready = on_ready.clone();

            tauri::async_runtime::spawn(async move {
                if let Some(callback) = on_ready {
                    callback();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![command])
        .run(context);

    // Check if the application started successfully
    match result {
        Ok(_) => 0,  // Success
        Err(_) => 1, // Failure
    }
}
