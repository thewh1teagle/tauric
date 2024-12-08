"""
pip install tauripy
python main.py
"""
from tauripy import Tauri, create_command_callback, create_ready_callback
from pathlib import Path

def command_callback(message: bytes) -> None:
    print(f"Command: {message.decode('utf-8')}")

def on_ready(tauric: Tauric) -> None:
    tauric.create_window("example_window", "local://index.html")

def main() -> None:
    tauric = Tauri("com.tauric.dev", "tauric")
    
    command_callback_c = create_command_callback(command_callback)
    ready_callback_c = create_ready_callback(lambda: on_ready(tauric))

    current_path = Path(__file__).parent
    dist_path = current_path / 'dist'
    tauric.mount_frontend(str(dist_path.absolute()))
    tauric.on_command(command_callback_c)
    tauric.on_ready(ready_callback_c)
    tauric.run_app()

if __name__ == "__main__":
    main()
