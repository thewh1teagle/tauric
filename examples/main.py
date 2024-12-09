"""
pip install tauripy
python main.py
"""
from tauripy import Tauri
from pathlib import Path
import json

def on_command(message: bytes):
    print(f"Received: {message.decode('utf-8')}")
    return json.dumps({'message': 'Hello from Python!'}).encode('utf-8')

def on_ready(tauric):
    tauric.create_window("example_window", "tauric", "fs://index.html")

def main() -> None:
    tauric = Tauri("com.tauric.dev", "tauric")
    current_path = Path(__file__).parent
    dist_path = current_path / 'dist'
    tauric.mount_frontend(str(dist_path.absolute()))
    tauric.on_command(on_command)
    tauric.run(lambda: on_ready(tauric))

if __name__ == "__main__":
    main()
