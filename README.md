# Slip

Floating markdown notes for [Omarchy](https://omarchy.org). Press Super+N and a pinned note appears over whatever you are doing. Press it again and it is gone.

Each note is a plain markdown file. Point Slip at a folder named `floating_notes` inside an Obsidian vault and Obsidian shows the same files. Images pasted in Slip are saved under `floating_notes/assets/` and linked as `![](assets/...)`.

Slip is derived from [Karatasi](https://github.com/HemalR/karatasi) by Hemal, used under the MIT license in `LICENSE`.

## Install on Omarchy (Apple Silicon)

Omarchy on a Mac is Arch Linux ARM. These steps are for that machine, where `uname -m` prints `aarch64`. Build Slip on the Mac. An x86_64 binary will not run there.

`~/.local/bin` is already on `PATH` in a stock Omarchy session. Super+N is free. Super+Shift+N still opens your editor.

### 1. Packages and Rust

```bash
sudo pacman -S --needed base-devel webkit2gtk-4.1 gtk3 nodejs npm curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
. "$HOME/.cargo/env"
```

The first release build compiles Tauri and takes a few minutes on an M-series Mac.

### 2. Build and install

```bash
git clone https://github.com/xavier-castro/slip.git "$HOME/src/slip"
cd "$HOME/src/slip"
npm ci
bash scripts/install.sh
```

That installs `~/.local/bin/slip` and starts it hidden. A second `slip` command talks to the running instance, so Hyprland only has to run `slip toggle`.

### 3. Keep the notes in Obsidian

Create `floating_notes` at the root of your vault, then tell Slip to use it. Replace the vault path with yours:

```bash
mkdir -p "$HOME/Documents/your-vault/floating_notes"
mkdir -p "$HOME/.config/slip"
```

`~/.config/slip/config.toml`:

```toml
notes_dir = "~/Documents/your-vault/floating_notes"
```

`config.example.toml` in this repo is the same file with the other knobs. Restart Slip after changing `notes_dir` (`pkill -x slip`, then `slip start`, or run `bash scripts/install.sh` again).

If `notes_dir` is unset, notes go to `~/Documents/floating_notes`. Slip creates the folder on first launch and writes `welcome-to-slip.md` when the folder is empty.

Obsidian reads those files as normal notes. Renaming a note in Slip renames the file. Wikilinks Obsidian already made to the old filename are not rewritten.

### 4. Hyprland

At the bottom of `~/.config/hypr/hyprland.lua`, after Omarchy's defaults:

```lua
dofile(os.getenv("HOME") .. "/src/slip/hypr/slip.lua")
```

Use the path you actually cloned into. Set any of these on the lines above that `dofile` to change the defaults:

```lua
slip_toggle_key = "SUPER + N"
slip_close_key = "SUPER + W" -- false to leave Omarchy's close key alone
slip_autostart = true
```

Super+W is Omarchy's close-window binding. The snippet unbinds it and binds a replacement. When Slip is focused, Super+W hides Slip. Every other window still closes. Escape still hides the main Slip window.

```bash
hyprctl reload
hyprctl configerrors
```

`configerrors` should be empty. Press Super+N. The window class is `slip`, the title is `Slip`, and the window is floating and pinned. `slip_autostart` starts `slip start` on the next login so the first Super+N is instant. `scripts/install.sh` already starts it for the current session.

## Keys

| Where | Key | Action |
| --- | --- | --- |
| Anywhere | Super N | Show or hide the notes window |
| Anywhere | Super W | Hide Slip, or close any other window |
| Anywhere | Super T | Tile the window, or float it again |
| Editor | Ctrl K / Ctrl P | Search every note |
| Editor | Ctrl N | New note |
| Editor | Ctrl Shift N | New note in its own window |
| Editor | Ctrl Enter | Finish the line: tick a todo, strike anything else; again to reopen |
| Editor | Ctrl Shift Enter | Turn the line into a todo, or back |
| Editor | Ctrl Shift K | Delete the line or list item |
| Editor | Ctrl Up / Ctrl Down | Move the line or list item |
| Editor | Ctrl [ / Ctrl ] | Previous / next note |
| Editor | Ctrl Tab | Cycle recently opened notes |
| Editor | Ctrl X | Delete the note (system trash; toast to undo) |
| Editor | Shift Insert | Paste from the clipboard |
| Editor | Esc | Hide (main window) or close (any other) |

The first line is the title, and the file is named after it. A new note is a draft until the first keystroke. Leaving an untitled empty note removes its file.

## Command line

`slip [show|toggle|hide|search|new|start]`. A second invocation forwards the action to the running instance over `$XDG_RUNTIME_DIR/slip.sock`. `slip start` launches it hidden.

## Configuration

Optional `~/.config/slip/config.toml`. Colors follow the active Omarchy theme. The font is the active Omarchy font, or `JetBrainsMono Nerd Font` when that file is missing. `font_size` defaults to 16.

The primary instance re-executes itself inside a transient systemd user scope with `MemoryMax=1500M` and `MemorySwapMax=0`. Override the cap with `SLIP_MEMORY_MAX`. If `systemd-run` is missing, Slip logs that and keeps running. Set `SLIP_DEBUG=1` to dump editor HTML to `$XDG_RUNTIME_DIR/slip-debug-html.txt` on every note load.

## Layout

- `src-tauri/src/lib.rs` — commands, window actions, single-instance socket, file watcher
- `src-tauri/src/store.rs` — markdown note store
- `src-tauri/src/theme.rs` — Omarchy theme and `config.toml`
- `src/main.ts` — editor window
- `src/titles.ts` — window titles shared with the Hyprland rules
- `hypr/slip.lua` — key bindings and window rules
