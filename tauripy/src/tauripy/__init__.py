import ctypes
import platform
import os

def load_library():
    try:
        ext = ".dll" if platform.system() == "Windows" else ".so" if platform.system() == "Linux" else ".dylib"
        lib_name = "tauric" + ext if platform.system() == "Windows" else 'libtauric' + ext

        # Same dir as this file
        lib_path = os.path.join(os.path.dirname(__file__), lib_name)

        lib = ctypes.CDLL(lib_path)

        return lib

    except OSError as e:
        print(f"Error loading shared library from {lib_path}: {e}")
        return None

def create_command_callback(command_callback):
    return ctypes.CFUNCTYPE(ctypes.c_char_p, ctypes.c_char_p)(command_callback)

def create_ready_callback(ready_callback):
    return ctypes.CFUNCTYPE(None)(ready_callback)

class Tauri:
    def __init__(self, identifier: str, product_name: str, icon: str = None) -> None:
        self.identifier = identifier
        self.product_name = product_name
        self.icon = icon
        self.tauric = load_library()
        self.setup_functions()


    def setup_functions(self) -> None:
        self.tauric.TauricRun.restype = ctypes.c_int
        self.tauric.TauricRun.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p, ctypes.CFUNCTYPE(None)]
        self.tauric.TauricCreateWindow.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_char_p]
        self.tauric.TauricCreateWindow.restype = None
        self.tauric.TauricClose.restype = None
        self.tauric.TauricOnReady.restype = None
        self.tauric.TauricOnCommand.restype = None

    def on_command(self, command_c_callback) -> None:
        self.tauric.TauricOnCommand(command_c_callback)

    def on_ready(self, ready_c_callback) -> None:
        self.tauric.TauricOnReady(ready_c_callback)

    def run(self, on_ready = None) -> None:
        if on_ready:
            on_ready = ctypes.CFUNCTYPE(None)(on_ready)
        icon = None
        if self.icon:
            icon = self.icon
            icon = icon.encode('utf-8')
        result = self.tauric.TauricRun(self.identifier.encode('utf-8'), self.product_name.encode('utf-8'), icon, on_ready)
        if result != 0:
            print("Failed to start the Tauri application")
        else:
            print("Tauri application started successfully")

    def mount_frontend(self, path) -> None:
        path = path.encode('utf-8')
        self.tauric.TauricMountFrontend(path)

    def create_window(self, label: str, title: str, url: str) -> None:
        label_encoded = label.encode('utf-8')
        url_encoded = url.encode('utf-8')
        title = title.encode('utf-8')
        self.tauric.TauricCreateWindow(label_encoded, title, url_encoded)

    def close(self) -> None:
        self.tauric.TauricClose()
