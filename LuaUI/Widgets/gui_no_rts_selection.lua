function widget:GetInfo()
    return {
        name = "Disable RTS selection",
        desc = "Flove uses direct spire controls, not selectable unit orders",
        author = "Flove",
        layer = 100000,
        enabled = true,
    }
end

function widget:KeyPress(key, mods)
    if mods.ctrl and (key == Spring.GetKeyCode("a") or key == Spring.GetKeyCode("d")) then
        Spring.SelectUnitArray({})
        return true
    end
    return false
end

function widget:Update()
    -- SetUnitNoSelect would also remove trees from TraceScreenRay, breaking RMB.
    if Spring.GetSelectedUnitsCount() > 0 then
        Spring.SelectUnitArray({})
    end
end

function widget:CommandNotify()
    return true
end
