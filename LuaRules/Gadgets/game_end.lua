--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function gadget:GetInfo()
	return {
		name      = "Game End",
		desc      = "Handles team/allyteam deaths and declares gameover",
		author    = "Andrea Piras",
		date      = "June, 2013",
		license   = "GNU GPL, v2 or later",
		layer     = 0,
		enabled   = true  --  loaded by default?
	}
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

--SYNCED
if (not gadgetHandler:IsSyncedCode()) then
   return false
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local initializeFrame = 0

function gadget:Initialize()
	initializeFrame = Spring.GetGameFrame() or 0
	Spring.SetGameRulesParam("gameOver", 0)
end

local spireDefID = UnitDefNames["spire"].id
local spireID = nil

function CheckForSpire(unitID, unitDefID)
	if unitDefID == spireDefID then
		spireID = unitID
	end
end

function gadget:UnitCreated(unitID, unitDefID)
	CheckForSpire(unitID, unitDefID)
end

function gadget:UnitDestroyed(unitID, unitDefID)
    if unitID == spireID then
        Spring.SetGameRulesParam("gameOver", 1)
    end
end