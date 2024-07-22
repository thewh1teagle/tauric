from tauric import Tauric, create_command_callback, create_ready_callback

def command_callback(message: bytes) -> None:
    print(f"Command: {message.decode('utf-8')}")

def on_ready(tauric: Tauric) -> None:
    tauric.create_window("example_window", "https://www.example.com")

def main() -> None:
    tauric = Tauric()
    
    command_callback_c = create_command_callback(command_callback)
    ready_callback_c = create_ready_callback(lambda: on_ready(tauric))

    tauric.on_command(command_callback_c)
    tauric.on_ready(ready_callback_c)
    tauric.run_app()

if __name__ == "__main__":
    main()
