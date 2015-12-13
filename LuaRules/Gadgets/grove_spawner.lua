--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function gadget:GetInfo()
   return {
      name      = "Grove spawner",
      desc      = "Spawns groves",
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

local baseDefID   = UnitDefNames["base"].id

local treeLevel1DefID  = UnitDefNames["treelevel1"].id

local flower1DefID  = UnitDefNames["flower1"].id
local flower2DefID  = UnitDefNames["flower2"].id
local flower3DefID  = UnitDefNames["flower3"].id
local flower4DefID  = UnitDefNames["flower4"].id
local flower5DefID  = UnitDefNames["flower5"].id
local flower6DefID  = UnitDefNames["flower6"].id
local flowers = { flower1DefID, flower2DefID, flower3DefID, flower4DefID, flower5DefID, flower6DefID }

local grass1DefID  = UnitDefNames["grass1"].id
local grass2DefID  = UnitDefNames["grass2"].id
local grass3DefID  = UnitDefNames["grass3"].id
local grass4DefID  = UnitDefNames["grass4"].id
local grass5DefID  = UnitDefNames["grass5"].id
local grass = { grass1DefID, grass2DefID, grass3DefID, grass4DefID, grass5DefID }

local spireDefID = UnitDefNames["spire"].id
local spireID = nil

local startSpawnFrame = 100
local spawnPoints = nil


local MIN_GRASS = 5
local MAX_GRASS = 10
local MIN_FLOWERS = 3
local MAX_FLOWERS = 6
local SHRUB_SPAWN_RADIUS = 500

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local function SpawnUnit(unitDefID, x, z, noRotate)
	local unitID = Spring.CreateUnit(unitDefID, x, 0, z, 0, 1, false, false)
	if not noRotate then
		Spring.SetUnitRotation(unitID, 0, math.random()*2*math.pi, 0)
	end
end

local function CleanUnits()
	for _, unitID in ipairs(Spring.GetAllUnits()) do
		local unitDefID = Spring.GetUnitDefID(unitID)
		local unitDef = UnitDefs[unitDefID]
		if unitDef.customParams.tree or unitDef.customParams.shrubs then
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
	startSpawnFrame = 10 + Spring.GetGameFrame()
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
			if unitDefID == baseDefID then
				table.insert(spawnPoints, unitID)
			end
		end
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
	Spring.Echo("SPAWN WAVE", #spawnPoints)
	for _, spawnPointID in pairs(spawnPoints) do
		local x, _, z = Spring.GetUnitPosition(spawnPointID)
		SpawnUnit(treeLevel1DefID, x, z)
		SpawnShrubs(x, z)
	end
end

function SpawnShrubs(x, z)
	local grassCount = math.random(MIN_GRASS, MAX_GRASS)
	local flowerCount = math.random(MIN_FLOWERS, MAX_FLOWERS)
	for i = 1, grassCount do
		local ux, uz = x + math.random() * SHRUB_SPAWN_RADIUS - SHRUB_SPAWN_RADIUS/2, z + math.random() * SHRUB_SPAWN_RADIUS - SHRUB_SPAWN_RADIUS/2
		local indx = math.random(1, #grass)
		local grassDefID = grass[indx]
		SpawnUnit(grassDefID, ux, uz)
	end
	for i = 1, flowerCount do
		local ux, uz = x + math.random() * SHRUB_SPAWN_RADIUS - SHRUB_SPAWN_RADIUS/2, z + math.random() * SHRUB_SPAWN_RADIUS - SHRUB_SPAWN_RADIUS/2
		local indx = math.random(1, #flowers)
		local flowerDefID = flowers[indx]
		SpawnUnit(flowerDefID, ux, uz)
	end
end