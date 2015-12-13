--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function gadget:GetInfo()
   return {
      name      = "Mushroom spawner",
      desc      = "Spawns mushrooms.",
      author    = "gajop",
      date      = "December 2015",
      license   = "GNU GPL, v2 or later",
      layer     = 200,
      enabled   = true
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

local mushroomSpawnDefID   = UnitDefNames["mushroomspawn"].id

local normalMushroomDefID  = UnitDefNames["normalmushroom"].id
local smallMushroomDefID   = UnitDefNames["smallmushroom"].id
local bigMushroomDefID     = UnitDefNames["bigmushroom"].id
local mushroomclusterDefID = UnitDefNames["mushroomcluster"].id
local poisonMushroomDefID  = UnitDefNames["poisonmushroom"].id
local bombmushroomDefID    = UnitDefNames["bombmushroom"].id

local spireDefID = UnitDefNames["spire"].id
local spireID = nil

local currentWave = 0
local startSpawnFrame = 100
local spawnPoints = nil

local waveConfig = {
	[1] = {
		[normalMushroomDefID] = 5,
	},
	[2] = {
		[normalMushroomDefID] = 5,
		[smallMushroomDefID] = 3,
	},
}

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local function SpawnUnit(unitDefID, x, z, noRotate)
	local unitID = Spring.CreateUnit(unitDefID, x, 0, z, 0, 0, false, false)
	if not noRotate then
		Spring.SetUnitRotation(unitID, 0, math.random()*2*math.pi, 0)
	end
	local sx, sy, sz = Spring.GetUnitPosition(spireID)
	Spring.GiveOrderToUnit(unitID, CMD.FIGHT, {sx, sy, sz}, 0 )
end

local function CleanUnits()
	for _, unitID in ipairs(Spring.GetAllUnits()) do
		local unitDefID = Spring.GetUnitDefID(unitID)
		local unitDef = UnitDefs[unitDefID]
		if unitDef.customParams.mushroom then
			Spring.DestroyUnit(unitID, false, false)
		else
			OnUnitCreated(unitID, unitDefID)
		end
	end
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function gadget:Initialize()
	CleanUnits()
	startSpawnFrame = 100 + Spring.GetGameFrame()
-- 	GG.SpawnField(2650, 2125, 2930, 2380, 4, 5)
-- 	GG.SpawnField(2320, 3050, 2505, 3550, 8, 3)
-- 	GG.SpawnField(3230, 3900, 3890, 4190, 4, 11)
-- 	GG.SpawnField(3745, 2360, 3870, 2570, 4, 3)
-- 
-- 	GG.SpawnBurrow(3600, 1350)
-- 	GG.SpawnBurrow(4740, 1940)
-- 	GG.SpawnBurrow(4650, 3340)
-- 	GG.SpawnBurrow(4920, 4620)
-- 	GG.SpawnBurrow(3680, 5270)
-- 	GG.SpawnBurrow(2190, 4820)
-- 	GG.SpawnBurrow(1000, 3660)
-- 	GG.SpawnBurrow( 800, 2170)
-- 	GG.SpawnBurrow(2050, 1250)
-- 	
-- 	SpawnUnit(lighthouseDefID, 2560, 2500, true)
-- 	SpawnUnit(lighthouseDefID, 3130, 4320, true)
-- 	SpawnUnit(lighthouseDefID, 3980, 3810, true)
-- 	SpawnUnit(lighthouseDefID, 3630, 2670, true)
-- 	SpawnUnit(lighthouseDefID, 2200, 3650, true)
-- 	
-- 	startFrame = Spring.GetGameFrame()
-- 	
-- 	Spring.SetGameRulesParam("score", 0)
-- 	Spring.SetGameRulesParam("survivalTime", 0)
-- 	currentDifficult = 1
end

function gadget:GameFrame(frame)
	if frame < startSpawnFrame then
		return
	end
	if spawnPoints == nil then
		spawnPoints = {}
		for _, unitID in ipairs(Spring.GetAllUnits()) do
			local unitDefID = Spring.GetUnitDefID(unitID)
			if unitDefID == mushroomSpawnDefID then
				table.insert(spawnPoints, unitID)
			end
		end
	end
	if frame % 33 * 2 == 0 then
		SpawnWave()
	end
end

function OnUnitCreated(unitID, unitDefID)
	if unitDefID == spireDefID then
		spireID = unitID
	end
end

function gadget:UnitCreated(unitID, unitDefID)
	OnUnitCreated(unitID, unitDefID)
end

function SpawnWave()
	currentWave = currentWave + 1
	local config = waveConfig[currentWave]
	if config == nil then
		return
	end
	Spring.Log("spawn", LOG.NOTICE, "Spawning wave " .. tostring(currentWave))
	
	for _, spawnPointID in pairs(spawnPoints) do
		local x, _, z = Spring.GetUnitPosition(spawnPointID)
		for unitDefID, count in pairs(config) do
			SpawnUnit(unitDefID, x, z)
		end
	end
end