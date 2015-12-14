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

-- SYNCED ONLY
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
local nextTreeSpawnTime = -1

-- grass spawning
local MIN_GRASS = 8
local MAX_GRASS = 12
local GRASS_SPAWN_RADIUS = 500

-- non-base tree spawning, one by one
local treeSpawnInterval = 5*30 -- mean time in between trees spawning
local treeSpawnStDev = 1*30 -- approx std dev of time in between trees spawning
local MAX_TREE_HEIGHT = 1600

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Event helper funcs
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
			RecordUnitCreatedFrame(unitID, unitDefID)
		end
	end
end

function RecordUnitCreatedFrame(unitID, unitDefID)
	local frame = Spring.GetGameFrame()
	Spring.SetUnitRulesParam(unitID, "createdFrame", frame, {public=true})
	if unitDefID == spireDefID then
		spireID = unitID
	end
end

function gadget:UnitCreated(unitID, unitDefID)
	RecordUnitCreatedFrame(unitID, unitDefID)
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Event Handler
--------------------------------------------------------------------------------

function gadget:Initialize()
	CleanUnits()
	startSpawnFrame = 10 + Spring.GetGameFrame()
    nextTreeSpawnTime = NewTreeSpawnTime()
end

function gadget:GameFrame(frame)
	if frame < startSpawnFrame then
		return
	end
    
    -- spawn base wave
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
    
    -- gradually spawn new trees
    if frame >= nextTreeSpawnTime then
        nextTreeSpawnTime = NewTreeSpawnTime()
        SpawnTree()
    end
    
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Helper funcs
--------------------------------------------------------------------------------

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

function IsInCircle(x,z, r, px,pz)
    return (x-px)*(x-px) + (z-pz)*(z-pz) <= r*r
end

function SignedRandom()
    return math.random()*2-1
end


--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Base (wave) spawning
--------------------------------------------------------------------------------

function SpawnWave(spawns)
	for _, spawnPointID in pairs(spawns) do
		local x, _, z = Spring.GetUnitPosition(spawnPointID)
		SpawnUnit(treeLevel1DefID, x, z)
		SpawnGrass(x, z, MIN_GRASS, MAX_GRASS, GRASS_SPAWN_RADIUS)
	end
	currentWave = currentWave + 1
end

function SpawnGrass(x, z, minGrass, maxGrass, radius, minRadius)
    minRadius = minRadius or 0
	local units = {}
	local grassCount = math.random(minGrass, maxGrass)
	for i = 1, grassCount do
		local ux = x + math.random() * radius - radius/2
        local uz = z + math.random() * radius - radius/2
        local r = (minRadius>0) and math.sqrt((x-ux)*(x-ux)+(z-ux)*(z-ux)) or 1
		if not IsSteep(ux,uz) and r>minRadius then
            local indx = math.random(1, #grass)
            local grassDefID = grass[indx]
            local unitID = SpawnUnit(grassDefID, ux, uz)
            table.insert(units, unitID)
        end
	end
	return units
end

GG.SpawnGrass = SpawnGrass

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Single Tree Growth
--------------------------------------------------------------------------------

function NewTreeSpawnTime()
    local f = Spring.GetGameFrame()
    return f + math.max(1,treeSpawnInterval + treeSpawnStDev*SignedRandom())
end

function IsTooHigh(x,z)
    -- hard coded hacks ftw
    local gy = Spring.GetGroundHeight(x,z)
    --Spring.Echo(x,gy,z,MAX_TREE_HEIGHT)
    if not gy then return true end
    return (gy>MAX_TREE_HEIGHT)    
end

function SpawnTree()
    -- now choose a position in which to spawn a new (L1) tree
    local units = Spring.GetAllUnits()
    
    -- knuth shuffle, because Spring.GetAllUnits order is predictable
    local perm = {}
    for i=1,#units do
		perm[i] = i
	end
    for i=1,#units-1 do
        local j = math.random(i,#units)
		local temp = perm[i]
		perm[i] = perm[j]
		perm[j] = temp
	end
    
    -- cycle over trees in random order
    local success = false
    local tries = 2
    for try=1,tries do
        for i=1,#units do
            local uID = units[perm[i]]
            local uDID = Spring.GetUnitDefID(uID)
            if UnitDefs[uDID].customParams.tree then
                success = TryToSpawnANewTreeSomewhereNearToThisTree(uID)
                if success then break end
            end
        end
        if success then break end
    end    
    if not success then
        --Spring.Echo("OH NOOOO") --TODO
    end
end

function TryToSpawnANewTreeSomewhereNearToThisTree(unitID)
    local x,_,z = Spring.GetUnitPosition(unitID)
    local minDist = 225 -- tress are not allowed to be closer together than this 
    local maxDist = 325
    local distInterval = maxDist-minDist
    local circleDivs = 5
    local thetaDiv = 2*math.pi/circleDivs
    local theta = math.random()*thetaDiv
    for i=1,circleDivs do
        theta = theta + thetaDiv
        local r = minDist + math.random()*distInterval
        local tx = x + r*math.cos(theta)
        local tz = z + r*math.sin(theta)
        local success = (not IsSteep(tx,tz)) and (not IsTooHigh(tx,tz)) and ILoveExcessivelyLongFunctionNames(tx,tz,minDist)
        if success then return true end
    end
    return false
end

function ILoveExcessivelyLongFunctionNames(tx,tz, minDist)
    -- try to spawn a tree at tx, tz
    -- but first check that we are far enough away from the nearest tree
    local possibleOtherTrees = Spring.GetUnitsInCylinder(tx,tz, minDist)
    for _,uID in ipairs(possibleOtherTrees) do
        local uDID = Spring.GetUnitDefID(uID)
        if UnitDefs[uDID].customParams.tree or uID==spireID then
            return false
        end    
    end
    
    -- thank fuck
    SpawnUnit(treeLevel1DefID, tx, tz)
    SpawnGrass(tx, tz, 3, 5, minDist, 10)
    return true
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Field of Flowers spawning (TODO: move to field of flowers gadget)
--------------------------------------------------------------------------------

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

GG.SpawnFlowers = SpawnFlowers


