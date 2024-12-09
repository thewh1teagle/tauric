"""
pip install tauripy
python main.py
"""
from tauripy import Tauri, create_command_callback, create_ready_callback
from pathlib import Path
from threading import Timer
import json

def on_command(message: bytes):
    print(f"Received: {message.decode('utf-8')}")
    return json.dumps({'message': 'Hello from Python!'}).encode('utf-8')

def main() -> None:
    tauric = Tauri("com.tauric.dev", "tauric")
    
    command_callback_c = create_command_callback(on_command)

    current_path = Path(__file__).parent
    dist_path = current_path / 'dist'
    tauric.mount_frontend(str(dist_path.absolute()))
    tauric.on_command(command_callback_c)
    tauric.run(lambda: tauric.create_window("example_window", "tauric", "fs://index.html"))
    

if __name__ == "__main__":
    main()
