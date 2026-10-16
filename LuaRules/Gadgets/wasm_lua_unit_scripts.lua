-- Compatibility between the WASM rules and the unchanged Lua unit scripts.
function gadget:GetInfo()
    return {name = "Wasm Lua unit scripts", layer = 10, enabled = true}
end
if not gadgetHandler:IsSyncedCode() then return false end
function gadget:UnitCreated(unitID)
    Spring.SetUnitRulesParam(unitID, "createdFrame", Spring.GetGameFrame(), {public=true})
end
function gadget:Initialize()
    for _, id in ipairs(Spring.GetAllUnits()) do self:UnitCreated(id) end
end
function gadget:GameFrame()
    for _, id in ipairs(Spring.GetTeamUnits(1)) do
        if Spring.GetUnitDefID(id) == UnitDefNames.spire.id then
            -- The Lua spire's eye task reads grove power from game params;
            -- the WASM HUD publishes it on its carrier instead.
            Spring.SetGameRulesParam("power", Spring.GetUnitRulesParam(id, "power") or 0)
            local desired = Spring.GetUnitRulesParam(id, "desiredBranches") or 0
            local current = Spring.GetUnitRulesParam(id, "branches") or 0
            local env = Spring.UnitScript.GetScriptEnv(id)
            if env then
                for _ = current + 1, desired do
                    Spring.UnitScript.CallAsUnit(id, env.ShowBranchNoThread)
                end
                for _ = current, desired + 1, -1 do
                    Spring.UnitScript.CallAsUnit(id, env.HideBranchNoThread)
                end
            end
        end
    end
end
