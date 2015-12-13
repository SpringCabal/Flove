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
local expansion1DefID   = UnitDefNames["expansion1"].id
local expansion2DefID   = UnitDefNames["expansion2"].id
local expansion3DefID   = UnitDefNames["expansion3"].id

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
local firstSpawnFrame = nil
local spawnPoints = nil
local currentWave = 1

-- grove spawning
local MIN_GRASS = 8
local MAX_GRASS = 12
local SHRUB_SPAWN_RADIUS = 500

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local function SpawnUnit(unitDefID, x, z, noRotate)
	local unitID = Spring.CreateUnit(unitDefID, x, 0, z, 0, 1, false, false)
	if not noRotate then
		Spring.SetUnitRotation(unitID, 0, math.random()*2*math.pi, 0)
	end
	return unitID
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
		local basePoints = {}
		spawnPoints.base = basePoints
		for _, unitID in ipairs(Spring.GetAllUnits()) do
			local unitDefID = Spring.GetUnitDefID(unitID)
			if unitDefID == baseDefID then
				table.insert(basePoints, unitID)
			end
		end
		SpawnWave(spawnPoints.base)
	end
end

function OnUnitCreated(unitID, unitDefID)
	local frame = Spring.GetGameFrame()
	Spring.SetUnitRulesParam(unitID, "createdFrame", frame, {public=true})
	if unitDefID == spireDefID then
		spireID = unitID
	end
end

function gadget:UnitCreated(unitID, unitDefID)
	OnUnitCreated(unitID, unitDefID)
end

function SpawnWave(spawns)
	for _, spawnPointID in pairs(spawns) do
		local x, _, z = Spring.GetUnitPosition(spawnPointID)
		SpawnUnit(treeLevel1DefID, x, z)
		SpawnGrass(x, z, MIN_GRASS, MAX_GRASS, SHRUB_SPAWN_RADIUS)
		--SpawnFlowers(x, z, MIN_FLOWERS, MAX_FLOWERS, SHRUB_SPAWN_RADIUS)
	end
	currentWave = currentWave + 1
end

function IsSteep(x,z)
	local mtta = math.acos(1.0 - 0.05) - 0.02 --http://springrts.com/wiki/Movedefs.lua#How_slope_is_determined
	local a1,a2,a3,a4 = 0,0,0,0
	local d = 5
	local y = Spring.GetGroundHeight(x,z)
	local y1 = Spring.GetGroundHeight(x+d,z)
	if math.abs(y1 - y) > 0.1 then a1 = math.atan((y1-y)/d) end
	local y2 = Spring.GetGroundHeight(x,z+d)
	if math.abs(y2 - y) > 0.1 then a2 = math.atan((y2-y)/d) end
	local y3 = Spring.GetGroundHeight(x-d,z)
	if math.abs(y3 - y) > 0.1 then a3 = math.atan((y3-y)/d) end
	local y4 = Spring.GetGroundHeight(x,z+d)
	if math.abs(y4 - y) > 0.1 then a4 = math.atan((y4-y)/d) end
	if math.abs(a1) > mtta or math.abs(a2) > mtta or math.abs(a3) > mtta or math.abs(a4) > mtta then 
		return true --too steep
	else
		return false --ok
	end	
end

function SpawnGrass(x, z, minGrass, maxGrass, radius)
	local units = {}
	local grassCount = math.random(minGrass, maxGrass)
	for i = 1, grassCount do
		local ux = x + math.random() * radius - radius/2
        local uz = z + math.random() * radius - radius/2
		if not IsSteep(ux,uz) then
            local indx = math.random(1, #grass)
            local grassDefID = grass[indx]
            local unitID = SpawnUnit(grassDefID, ux, uz)
            table.insert(units, unitID)
        end
	end
	return units
end

function IsInCircle(x,z, r, px,pz)
    return (x-px)*(x-px) + (z-pz)*(z-pz) <= r*r
end

function SignedRandom()
    return math.random()*2-1
end

function SpawnFlowers(x, z, inverseDensity, radius)
    -- used by field of flowers, only
    -- spawn approximately one flower per square of sidelength inverseDensity, evenly spaced
    if radius<=0 then Spring.Echo("NO") end
	local units = {}
    local gridPoints = 2*radius/inverseDensity
    local gridSize = inverseDensity
    local peturbSize = inverseDensity/3
    local ox = x - radius -- grid origin
    local oz = z - radius
    local cx,cz = ox,oz -- current
    while cx < ox + 2*radius do
    while cz < oz + 2*radius do
        cz = cz + gridSize        
        local ux = cx + peturbSize*SignedRandom()
        local uz = cz + peturbSize*SignedRandom()
		if not IsSteep(ux,uz) and IsInCircle(cx,cz, radius, x, z) then
            local index = math.random(1, #flowers)
            local flowerDefID = flowers[index]
            local unitID = SpawnUnit(flowerDefID, ux, uz)
            table.insert(units, unitID)
        end
	end
    cx = cx + gridSize
    cz = oz
    end
    
	return units
end

GG.SpawnGrass = SpawnGrass
GG.SpawnFlowers = SpawnFlowers