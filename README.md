# caffeinate for Windows

<p>
  <a href="https://github.com/dincertekin/caffeinate-windows/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/dincertekin/dincertekin/main/assets/badges/download-dark.svg"><img src="https://raw.githubusercontent.com/dincertekin/dincertekin/main/assets/badges/download-light.svg" alt="Download the latest release" height="28"></picture></a>
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/dincertekin/dincertekin/main/assets/badges/windows-dark.svg"><img src="https://raw.githubusercontent.com/dincertekin/dincertekin/main/assets/badges/windows-light.svg" alt="Windows 10 and 11" height="28"></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/dincertekin/dincertekin/main/assets/badges/rust-dark.svg"><img src="https://raw.githubusercontent.com/dincertekin/dincertekin/main/assets/badges/rust-light.svg" alt="Rust" height="28"></picture>
  <a href="LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/dincertekin/dincertekin/main/assets/badges/mit-dark.svg"><img src="https://raw.githubusercontent.com/dincertekin/dincertekin/main/assets/badges/mit-light.svg" alt="MIT license" height="28"></picture></a>
</p>

**macOS's `caffeinate`, on Windows.** Keep your PC and display awake from the
terminal, for as long as a command runs, or for a set time.

```sh
caffeinate -d -i npm run build   # screen stays on until the build finishes
```

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

### Download (no Rust needed)

1. Download `caffeinate.exe` from the
   [latest release](https://github.com/dincertekin/caffeinate-windows/releases/latest).
2. Put it in a folder for your command-line tools, such as `C:\tools`, and add
   that folder to your PATH once. In PowerShell:

   ```powershell
   mkdir C:\tools
   move $HOME\Downloads\caffeinate.exe C:\tools\
   [Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";C:\tools", "User")
   ```

3. Open a new terminal and run `caffeinate`.

The exe isn't code-signed, so SmartScreen may warn you the first time. Click
**More info**, then **Run anyway**.

### With Cargo

If you have [Rust](https://rustup.rs/) installed:

```sh
cargo install --git https://github.com/dincertekin/caffeinate-windows
```

Cargo puts `caffeinate.exe` in `~/.cargo/bin`, which is already on your PATH.

## How It Works

Windows exposes a `SetThreadExecutionState` API that lets a process tell the OS to stay awake. Three flags matter:

- `ES_SYSTEM_REQUIRED` prevents idle sleep
- `ES_DISPLAY_REQUIRED` also prevents the display from turning off (what `-d` adds)
- `ES_CONTINUOUS` holds the state until explicitly cleared or the process exits

On Ctrl+C or natural exit, the state is cleared so Windows immediately goes back to normal sleep behavior.

## Dependencies

- [`windows`](https://crates.io/crates/windows) for Win32 API bindings
- [`ctrlc`](https://crates.io/crates/ctrlc) for clean Ctrl+C handling

## License

[MIT License](./LICENSE)
