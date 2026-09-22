# Slip

Floating markdown notes for [Omarchy](https://omarchy.org). Press Super+N and a pinned note appears over whatever you are doing. Press it again and it is gone.

Slip is derived from [Karatasi](https://github.com/HemalR/karatasi) by Hemal, used under the MIT license in `LICENSE`.

Every note is a plain markdown file in `~/Documents/xavier-obsidian/floating_notes`, which is a folder in the open Obsidian vault. Obsidian reads the same files. Images pasted in Slip are saved under `floating_notes/assets/` and linked as `![](assets/...)`.

## Install

Requires the Rust toolchain (`~/.cargo/bin`) and the Tauri system libraries (`webkit2gtk-4.1`, `gtk3`).

```bash
npm ci
bash scripts/install.sh
```

That builds the binary, installs `~/.local/bin/slip`, and starts it hidden. Then load the Hyprland snippet from `~/.config/hypr/hyprland.lua`, after Omarchy's defaults:

```lua
dofile("/home/xavier/Work/slip/hypr/slip.lua")
```

Set any of these before that line to change the defaults:

```lua
slip_toggle_key = "SUPER + N"
slip_close_key = "SUPER + W" -- false to leave Omarchy's close key alone
slip_autostart = true
```

Super+W was Omarchy's close-window binding. The snippet unbinds it. When Slip is focused, Super+W hides Slip. Every other window still closes.

Reload Hyprland after editing the snippet (`hyprctl reload`).

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

`slip [show|toggle|hide|search|new|start]`. A second invocation forwards the action to the running instance over a unix socket at `$XDG_RUNTIME_DIR/slip.sock`. `slip start` launches it hidden.

## Configuration

Optional `~/.config/slip/config.toml`:

```toml
notes_dir = "~/Documents/xavier-obsidian/floating_notes"
font_size = 16

[keys]
search = ["Ctrl+K", "Ctrl+P"]
new = "Ctrl+N"
delete = "Ctrl+X"
```

When `notes_dir` is omitted, notes go to `~/Documents/xavier-obsidian/floating_notes`. Colors follow the active Omarchy theme. The font is the active Omarchy font, or `JetBrainsMono Nerd Font` when that file is missing.

The primary instance re-executes itself inside a transient systemd scope with `MemoryMax=1500M` and `MemorySwapMax=0`. Override the cap with `SLIP_MEMORY_MAX`. Set `SLIP_DEBUG=1` to dump editor HTML to `$XDG_RUNTIME_DIR/slip-debug-html.txt` on every note load.

## Layout

- `src-tauri/src/lib.rs` — commands, window actions, single-instance socket, file watcher
- `src-tauri/src/store.rs` — markdown note store
- `src-tauri/src/theme.rs` — Omarchy theme and `config.toml`
- `src/main.ts` — editor window
- `src/titles.ts` — window titles shared with the Hyprland rules
- `hypr/slip.lua` — key bindings and window rules
