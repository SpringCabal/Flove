function gadget:GetInfo()
    return {
        name = "Flove command policy",
        desc = "Allow scripted AI orders, not player RTS commands or self-destruct",
        author = "Flove",
        layer = 100000,
        enabled = true,
    }
end

if not gadgetHandler:IsSyncedCode() then return false end

function gadget:AllowCommand(unitID, unitDefID, teamID, cmdID, cmdParams, cmdOptions, cmdTag, playerID, fromSynced)
    return fromSynced == true and cmdID ~= CMD.SELFD
end
