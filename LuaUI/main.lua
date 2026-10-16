-- LayoutButtons is currently a Lua-only engine hook. All gameplay/UI remains
-- in the Core WASM modules; this matches the original layout.lua widget.
function LayoutButtons(xIcons, yIcons, _cmdCount, _commands)
    return "", xIcons, yIcons, {}, {}, {}, {}, {}, {}, {}, {}
end
