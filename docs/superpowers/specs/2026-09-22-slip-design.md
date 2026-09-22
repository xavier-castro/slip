# Slip

Floating markdown notes for Omarchy, derived from [Karatasi](https://github.com/HemalR/karatasi) (MIT, copyright Hemal). Slip is the local app. Its notes are plain markdown files in the open Obsidian vault, so Obsidian shows them without a sync plugin.

## Outcome

Press Super+N anywhere and a pinned, floating Slip window appears over the current workspace. Press it again and it hides. Every note is a `.md` file in `~/Documents/xavier-obsidian/floating_notes`. Opening that folder in Obsidian shows the same files Slip just wrote. Editing a file in Obsidian updates the open Slip note.

Success means all of the following:

- The binary is `slip`, installed at `~/.local/bin/slip`.
- Window titles are `Slip`, `Slip - <note title>`, `Slip Search`, and `Slip Tiled`.
- A new note typed in Slip appears as a markdown file under `floating_notes/` and is visible in Obsidian.
- An edit saved in Obsidian reloads in Slip, and an edit saved in Slip reloads in Obsidian, without the two apps rewriting the file back and forth.
- Search, a second note window, line todos, image paste, theme colors, and delete-with-undo work as they do in Karatasi.

## Source

The project lives at `~/Work/slip`. It starts as a clone of `HemalR/karatasi` at the `main` commit `7764ca9d4e600b30bdcc905e19ca8627f06f3294`, then the renames and deletions in this spec are applied on top.

`LICENSE` stays the MIT license, with Hemal's copyright notice kept. `README.md` states that Slip is derived from Karatasi.

These paths are deleted because this copy is the local app, not the public release:

- `site/`
- `wrangler.toml`
- `scripts/deploy-site.sh`
- `scripts/release.sh`
- `.github/workflows/release.yml`
- `packaging/` (`PKGBUILD`, `.SRCINFO`, `karatasi-setup`, `karatasi.desktop`, `karatasi.lua`)

`scripts/install.sh` stays and is updated for the `slip` binary. `scripts/recent-notes.test.mjs` stays.

## Identity

| Karatasi | Slip |
| --- | --- |
| binary and Cargo package `karatasi` | `slip` |
| lib `karatasi_lib` | `slip_lib` |
| Tauri product name `karatasi` | `Slip` |
| identifier `dev.hemalr.karatasi` | `local.xavier.slip` |
| window class `karatasi` | `slip` |
| titles `Karatasi`, `Karatasi - …`, `Karatasi Search`, `Karatasi Tiled` | `Slip`, `Slip - …`, `Slip Search`, `Slip Tiled` |
| config `~/.config/karatasi/config.toml` | `~/.config/slip/config.toml` |
| default notes directory `~/Notes` | `~/Documents/xavier-obsidian/floating_notes` |
| socket `$XDG_RUNTIME_DIR/karatasi.sock` | `$XDG_RUNTIME_DIR/slip.sock` (the temp dir if the runtime dir is unset) |
| env `KARATASI_SCOPED`, `KARATASI_MEMORY_MAX`, `KARATASI_DEBUG` | `SLIP_SCOPED`, `SLIP_MEMORY_MAX`, `SLIP_DEBUG` |
| welcome note id `welcome-to-karatasi` | `welcome-to-slip` |

The welcome note text is rewritten so it describes Slip and `floating_notes`. User-facing strings, Hyprland title matches, notify-send app name, and the process name all say Slip. A `notes_dir` entry in `~/.config/slip/config.toml` still overrides the default.

GTK/Tauri may derive the Hyprland class from the identifier rather than the product name. After the first real window maps, `hyprctl clients` must show class `slip`. If it does not, the Hyprland match and the Rust class checks change to whatever class is actually set, and the two stay identical.

## Notes

`floating_notes/` is created on first launch if it is missing. It sits at the vault root, beside `00 - Inbox` and the other numbered folders. That is outside the vault's usual layout, and it is the folder this design is for. The welcome note is written only when the directory has no notes.

One note is one `.md` file:

- The first non-empty line is the title. A leading `#` is stripped when computing the title.
- The filename is the slug of that title (`Grocery list` becomes `grocery-list.md`), at most 60 characters, unique inside the folder.
- Saving a note whose title slug no longer matches the filename renames the file.
- A draft with no content lives only in the editor. Leaving it untitled and empty deletes the file if one was created, and does not send it to the trash.
- Images pasted or dropped in Slip are written to `floating_notes/assets/<title-slug>-<unix-seconds>.<ext>` and linked as `![](assets/...)`. A failed paste does not insert a link.
- Deleting a note with content uses `gio trash`. If that fails, the file is removed. A toast stays up for about five seconds and can restore the note.

Obsidian reads these files as normal notes. Slip does not add YAML frontmatter, and it does not rewrite links. The vault has `alwaysUpdateLinks` on, which rewrites wikilinks only when Obsidian itself renames a file. A rename performed by Slip leaves existing `[[wikilinks]]` from other notes pointing at the old filename. Notes are expected to be renamed from Slip. Wikilinks from daily notes to these files are out of scope.

Pasting an image inside Obsidian still uses the vault's configured attachment folder, `assets/` at the vault root. Pasting inside Slip uses `floating_notes/assets/`. Both links render. This design does not merge those two folders.

## Desktop

Hyprland integration is one snippet, `hypr/slip.lua`, in the repo. `~/.config/hypr/hyprland.lua` loads it with `dofile` after the existing `require` lines. These variables, set before the `dofile`, override the defaults:

- `slip_toggle_key`, default `SUPER + N`
- `slip_close_key`, default `SUPER + W`. Set it to `false` to leave the close key alone.
- `slip_autostart`, default `true`

Super+N is not bound on this machine. Super+Shift+N already opens the editor and is left alone. Super+W is Omarchy's close-window binding. The snippet unbinds it and binds a replacement: if the focused window's class is `slip`, run `slip hide`; otherwise close the window as before. Escape still hides the main window and closes any other Slip window.

The snippet's window rules match class `slip`:

- `Slip Search`: floating, pinned, centered, 680×440, fully opaque.
- `Slip Tiled`: not floating, fully opaque, so a tiled spawn stays tiled.
- `Slip` and `Slip - <title>`: floating, pinned, centered, 960×720, fully opaque, and not focused just because the window activated.

`slip_autostart` runs `slip start` at login, hidden, so the first Super+N is instant. `scripts/install.sh` builds the frontend, runs `npx tauri build --no-bundle`, installs `~/.local/bin/slip`, and restarts a running instance with `uwsm-app`.

Colors come from the active Omarchy theme (`~/.local/state/omarchy/current`), updating while Slip is open. The font is `font.name` in that directory. If `font.name` is missing, the font is `JetBrainsMono Nerd Font`. `font` and `font_size` in the config file override either one. `font_size` defaults to 16. In-app chords stay the Karatasi defaults and can still be replaced from `[keys]` in the config file.

The primary process re-executes itself in a transient systemd user scope with `MemoryMax=1500M` and `MemorySwapMax=0`, and sets `SLIP_SCOPED=1` so the child does not scope itself again. `SLIP_MEMORY_MAX` overrides the cap. If `systemd-run` is missing, Slip logs that and keeps running without the cap. `SLIP_DEBUG=1` dumps editor HTML to `$XDG_RUNTIME_DIR/slip-debug-html.txt` on each note load.

A second `slip <show|toggle|hide|search|new|start>` connects to the running instance over its unix socket and exits. `slip start` on an already running instance does not open a second editor.

## Failure behavior

- A missing notes directory is created. A config file that is missing or is not valid TOML is ignored, and the defaults above are used.
- A missing theme file leaves the color map empty and the font at its default. Slip still opens.
- If the socket cannot be bound, Slip logs the error and keeps running. Hyprland commands cannot reach that instance until it is restarted.
- If renaming a note's file fails, the new content is kept under the existing filename.
- The file watcher reacts to create, modify, and remove events only. Read events are ignored, so reloading a note cannot schedule another reload. External writes, including Obsidian saves, reload the affected note. Slip does not write the file again when the content is unchanged.
- The memory scope exists so a leak stops Slip and leaves the Hyprland session up.

## Testing

Before calling the install done:

1. Run the Rust tests in `src-tauri` (the store tests, including image attachment) and `node scripts/recent-notes.test.mjs`.
2. `hyprctl reload`, then `hyprctl configerrors`, must report no new errors from `hypr/slip.lua`.
3. Launch Slip, confirm `hyprctl clients` shows class `slip` and a title from the table above, and confirm the window is floating and pinned.
4. Type a note, confirm `floating_notes/<slug>.md` exists, and confirm the same text is visible in Obsidian.
5. Edit that note in Obsidian and confirm Slip shows the edit without a rewrite loop (the file mtime settles after one save from each side).
6. Change the title, confirm the file is renamed, paste an image, confirm `floating_notes/assets/` and the markdown link, delete the note, and confirm the undo toast restores it.

## Out of scope

- The Karatasi website, Cloudflare deploy, Arch packaging, and AUR metadata.
- Changing note shape to vault conventions (required H1, YAML frontmatter, wikilinks, vault-root `assets/`).
- TaskNotes, daily notes, and Templater integration.
- Publishing Slip or tracking upstream Karatasi after the initial clone.
