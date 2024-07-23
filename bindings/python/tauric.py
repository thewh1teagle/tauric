import ctypes
import platform
from pathlib import Path
from typing import Callable


def create_command_callback(command_callback):
    return ctypes.CFUNCTYPE(None, ctypes.c_char_p)(command_callback)

def create_ready_callback(ready_callback):
    return ctypes.CFUNCTYPE(None)(ready_callback)

class Tauric:
    def __init__(self) -> None:
        self.dylib_path = self.find_library_path()
        self.tauric = ctypes.CDLL(self.dylib_path)
        self.setup_functions()

    def find_library_path(self) -> Path:
        system = platform.system()
        lib_name = ""

        if system == "Windows":
            lib_name = "tauric.dll"
        elif system == "Darwin":
            lib_name = "libtauric.dylib"
        else:
            lib_name = "libtauric.so"

        base_path = Path(__file__).resolve().parent / '../../target'
        debug_path = base_path / 'debug' / lib_name
        release_path = base_path / 'release' / lib_name

        if debug_path.exists():
            return debug_path
        elif release_path.exists():
            return release_path
        else:
            raise FileNotFoundError(f"Library file not found in 'debug' or 'release' directories.")

    def setup_functions(self) -> None:
        self.tauric.run.restype = ctypes.c_int
        self.tauric.create_window.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
        self.tauric.create_window.restype = None
        self.tauric.close.restype = None
        self.tauric.on_ready.restype = None
        self.tauric.on_command.restype = None

    def on_command(self, command_c_callback) -> None:
        self.tauric.on_command(command_c_callback)

    def on_ready(self, ready_c_callback) -> None:
        self.tauric.on_ready(ready_c_callback)

    def run_app(self) -> None:
        result = self.tauric.run()
        if result != 0:
            print("Failed to start the Tauri application")
        else:
            print("Tauri application started successfully")

    def mount_frontend(self, path) -> None:
        path = path.encode('utf-8')
        self.tauric.mount_frontend(path)

    def create_window(self, label: str, url: str) -> None:
        label_encoded = label.encode('utf-8')
        url_encoded = url.encode('utf-8')
        print('Creating window...')
        self.tauric.create_window(label_encoded, url_encoded)
        print("Created an example window")

    def close(self) -> None:
        self.tauric.close()
