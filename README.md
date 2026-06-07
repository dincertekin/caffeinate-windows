# Caffeinate Windows

I got too used to running `caffeinate -d` on my Mac to keep my laptop awake during long tasks. When I switched to Windows, I missed it enough to just build it myself.

This is a minimal Rust CLI that replicates `caffeinate` on Windows. Run it, your system won't sleep. Kill it with Ctrl+C, everything goes back to normal.

## Usage

Keep system awake until you stop it:
```sh
caffeinate
```

Keep the **display** awake too (mirrors `caffeinate -d` on macOS):
```sh
caffeinate -d
```

Keep awake for a fixed duration (in seconds):
```sh
caffeinate -t 3600
```

Keep awake **only while a command runs**, then exit automatically:
```sh
caffeinate -i npm run build
caffeinate -i rsync -av ~/Pictures/ /Backup/Pictures/
caffeinate -d -i docker build .
```

Flags can be combined. `-t` and `-i` are mutually exclusive.

### Flags

| Flag | What it does |
|------|--------------|
| _(none)_ | Keep system awake until Ctrl+C |
| `-d` | Also keep the display on |
| `-t <seconds>` | Exit automatically after N seconds |
| `-i <command>` | Exit when the given command finishes |

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) installed

### Option A: System32 (quickest, no PATH setup needed)

System32 is already on PATH everywhere, so this just works:

```sh
cargo build --release
copy target\release\caffeinate.exe C:\Windows\System32\caffeinate.exe
```

Open a new terminal and you're done. This is what I do personally.

### Option B: your own folder

If you'd rather not touch System32, make a folder for personal CLI tools and add it to PATH once:

```sh
mkdir C:\tools
cargo build --release
copy target\release\caffeinate.exe C:\tools\caffeinate.exe
setx PATH "%PATH%;C:\tools"
```

Restart your terminal after `setx` for the PATH change to kick in.

### Option C: `cargo install`

```sh
git clone https://github.com/dincertekin/caffeinate-windows.git
cd caffeinate-windows
cargo install --path .
```

Cargo drops `caffeinate.exe` into `~/.cargo/bin`, which is already on your PATH after installing Rust.

## How It Works

Windows exposes a `SetThreadExecutionState` API that lets a process tell the OS to stay awake. Two relevant flags:

- `ES_SYSTEM_REQUIRED` prevents idle sleep
- `ES_DISPLAY_REQUIRED` also prevents the display from turning off (what `-d` adds)
- `ES_CONTINUOUS` holds the state until explicitly cleared or the process exits

On Ctrl+C or natural exit, the state is cleared so Windows immediately goes back to normal sleep behavior.

## Dependencies

- [`windows`](https://crates.io/crates/windows) for Win32 API bindings
- [`ctrlc`](https://crates.io/crates/ctrlc) for clean Ctrl+C handling

## License

[MIT License](./LICENSE)
