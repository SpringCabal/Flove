if not gadgetHandler:IsSyncedCode() then
	return
end
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function gadget:GetInfo()
	return {
		name 	= "Tree",
		desc	= "Tree control gadget.",
		author	= "gajop",
		date	= "12 December 2015",
		license	= "GNU GPL, v2 or later",
		layer	= 20,
		enabled = true
	}
end

local treeConfiguration = {
	treelevel1 = {
		level = 1,
		wisps = 3,
		minWispCreationSpeed = 33 * 2,
		maxWispCreationSpeed = 33 * 3,
	},
	treelevel2 = {
		level = 2,
		wisps = 7,
		minWispCreationSpeed = 33 * 1.5,
		maxWispCreationSpeed = 33 * 2.5,
	},
	treelevel3 = {
		level = 3,
		wisps = 12,
		minWispCreationSpeed = 33 * 1,
		maxWispCreationSpeed = 33 * 2,
	}
}
local wispConfiguration = {
	totalStages = 3,
	stageFrameDuration = 33 * 5,
}

local UPGRADE_PROGRESS = 3

local trees = {}

local function TreeCreated(unitID)
	local unitDefID = Spring.GetUnitDefID(unitID)
	local unitDefName = UnitDefs[unitDefID].name
	local config = treeConfiguration[unitDefName]
	if config == nil then
		Spring.Log("tree", LOG.ERROR, "No config for tree of type: " .. tostring(unitDefName))
		return
	end
	Spring.Log("tree", LOG.NOTICE, "Created tree of type: " .. tostring(unitDefName))
	local tree = {}
	trees[unitID] = tree
	
	tree.wisps = {}
	tree.createWispFrame = math.random(config.minWispCreationSpeed, config.maxWispCreationSpeed) + Spring.GetGameFrame()
	Spring.SetUnitRulesParam(unitID, "upgradeProgress", 0)
end

local function TreeDestroyed(unitID)
	local tree = trees[unitID]
	trees[unitID] = nil
end

local function UpgradeTree(unitID)
	local unitDefID = Spring.GetUnitDefID(unitID)
	local unitDefName = UnitDefs[unitDefID].name
	local config = treeConfiguration[unitDefName]
	local level = config.level + 1
	local newConfig, newDefName
	for defName, config in pairs(treeConfiguration) do
		if config.level == level then
			newDefName, newConfig = defName, config
			break
		end
	end

	local tree = trees[unitID] -- not necessary?
	
	local x, y, z = Spring.GetUnitPosition(unitID)
	local teamID = Spring.GetUnitTeam(unitID)
	local dx, dy, dz = Spring.GetUnitDirection(unitID)
	Spring.DestroyUnit(unitID)
	Spring.CreateUnit(newDefName, x, y, z, 0, teamID)
	Spring.SetUnitDirection(dx, dy, dz)
end

local function AddUpgradeProgress(unitID)
	local tree = trees[unitID]
	local unitDefID = Spring.GetUnitDefID(unitID)
	local unitDefName = UnitDefs[unitDefID].name
	local config = treeConfiguration[unitDefName]
	if config.level == 3 then
		return false
	end
	local progress = Spring.GetUnitRulesParam(unitID, "upgradeProgress") or 0
	progress = progress + 1
	Spring.SetUnitRulesParam(unitID, "upgradeProgress", progress, {public=true})
	if progress >= UPGRADE_PROGRESS then
		UpgradeTree(unitID)
	end
	return true
end

local function UpdateWisps()
	local frame = Spring.GetGameFrame()
	local stageDur = wispConfiguration.stageFrameDuration
	local totalStages = wispConfiguration.totalStages
	local mana = Spring.GetGameRulesParam("mana") or 0
	for unitID, tree in pairs(trees) do
		local wisps = tree.wisps
		for i, wisp in pairs(wisps) do
			if frame - wisp.stageFrame >= stageDur then
				if wisp.stage >= totalStages then
					mana = mana + 1
					wisp.stage = 1
					wisp.stageFrame = frame
					wisp.creationFrame = frame
					-- TODO: Some sort of notification/visual representation that the wisp turned into mana
				else
					wisp.stage = wisp.stage + 1
					wisp.stageFrame = frame
				end
				Spring.SetUnitRulesParam(unitID, "wispStage" .. tostring(i), wisp.stage)
			end
		end
		
		local unitDefID = Spring.GetUnitDefID(unitID)
		local unitDefName = UnitDefs[unitDefID].name
		local config = treeConfiguration[unitDefName]
		if tree.createWispFrame == frame and #tree.wisps < config.wisps then
			tree.createWispFrame = math.random(config.minWispCreationSpeed, config.maxWispCreationSpeed) + frame
			table.insert(wisps, { creationFrame = frame, stageFrame = frame, stage = 1})
			Spring.SetUnitRulesParam(unitID, "wispStage" .. tostring(#tree.wisps), 1)
			Spring.SetUnitRulesParam(unitID, "wispOffset" .. tostring(#tree.wisps), math.random() * 0.25)
		end
	end
	Spring.SetGameRulesParam("mana", mana)
end

function gadget:UnitCreated(unitID, unitDefID, ...)
	local unitDefName = UnitDefs[unitDefID].name
	local config = treeConfiguration[unitDefName]
	if config ~= nil then
		TreeCreated(unitID)
	end
end

function gadget:UnitDestroyed(unitID, unitDefID, ...)
	local unitDefName = UnitDefs[unitDefID].name
	local config = treeConfiguration[unitDefName]
	if config ~= nil then
		TreeDestroyed(unitID)
	end
end

function gadget:GameFrame()
	UpdateWisps()
end

function gadget:Initialize()
	for _, unitID in ipairs(Spring.GetAllUnits()) do
		local unitDefID = Spring.GetUnitDefID(unitID)
		gadget:UnitCreated(unitID, unitDefID)
	end
end

function gadget:Shutdown()
	for _, unitID in ipairs(Spring.GetAllUnits()) do
		local unitDefID = Spring.GetUnitDefID(unitID)
		gadget:UnitDestroyed(unitID, unitDefID)
	end
end

GG.trees = trees
GG.AddUpgradeProgress = AddUpgradeProgress