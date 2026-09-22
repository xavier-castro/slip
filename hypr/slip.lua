-- Slip: floating markdown notes for Omarchy, derived from Karatasi.
--
-- Load this from ~/.config/hypr/hyprland.lua after Omarchy's defaults:
--
--   dofile(os.getenv("HOME") .. "/src/slip/hypr/slip.lua")
--
-- Set any of these before the dofile line to change the defaults:
--
--   slip_toggle_key = "SUPER + N"
--   slip_close_key  = "SUPER + W"  -- false to leave Omarchy's close key alone
--   slip_autostart  = true

local toggle_key = slip_toggle_key or "SUPER + N"
local close_key = slip_close_key
if close_key == nil then close_key = "SUPER + W" end
local autostart = slip_autostart
if autostart == nil then autostart = true end

o.bind(toggle_key, "Notes: toggle Slip", "slip toggle")

-- Super+W was Omarchy's "Close window". When Slip is focused it hides; every other window still closes.
if close_key then
  hl.unbind(close_key)
  o.bind(close_key, "Close window (hides Slip)", function()
    local window = hl.get_active_window()
    if window and window.class == "slip" then
      hl.dispatch(hl.dsp.exec_cmd("slip hide"))
    else
      hl.dispatch(hl.dsp.window.close())
    end
  end)
end

-- Float and pin apply when the window maps. Size and position are restored by Slip
-- itself (default: 460×420 at the top-right of the built-in display). The main window
-- is titled exactly "Slip"; other note windows are "Slip - <note title>".
o.window({ class = "^slip$", title = "^Slip Search$" }, {
  tag = "-default-opacity",
  opacity = "1 1",
  float = true,
  pin = true,
  size = { 680, 440 },
  center = true,
  border_size = 1,
})
-- Ctrl+Shift+N from a tiled note spawns with this title so it tiles instead of floating.
-- Slip retitles it once the note loads.
o.window({ class = "^slip$", title = "^Slip Tiled$" }, {
  tag = "-default-opacity",
  opacity = "1 1",
  float = false,
})
o.window({ class = "^slip$", title = "^Slip( - .*)?$" }, {
  tag = "-default-opacity",
  opacity = "1 1",
  float = true,
  pin = true,
  focus_on_activate = false,
})

if autostart then
  o.launch_on_start("slip start")
end
