"""
pip install tauripy
python main.py
"""
from tauripy import Tauri
from pathlib import Path
import json

def on_command(message: bytes):
    print(f"Received: {message.decode()}")
    return json.dumps({'message': 'Hello from Python!'}).encode()

tauric = Tauri("com.tauric.dev", "tauric")
tauric.mount_frontend('./dist')
tauric.on_command(on_command)
tauric.run(lambda: tauric.create_window("example_window", "tauric", "fs://index.html"))