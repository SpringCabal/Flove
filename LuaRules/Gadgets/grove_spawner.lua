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

-- SYNCED ONLY (only suppress trigger drawing)
if (not gadgetHandler:IsSyncedCode()) then

function IsEffect(unitID)
	local unitDefID = Spring.GetUnitDefID(unitID)
	local unitDef = UnitDefs[unitDefID]
	return unitDef.customParams.effect
end

function gadget:Initialize(unitID)
	for _, unitID in ipairs(Spring.GetAllUnits()) do
		local unitDefID = Spring.GetUnitDefID(unitID)
		gadget:UnitCreated(unitID, unitDefID)
	end
end

function gadget:UnitCreated(unitID)
	if IsEffect(unitID) then
		Spring.UnitRendering.SetUnitLuaDraw(unitID, true)
	end
end
	
function gadget:DrawUnit(unitID)
	if IsEffect(unitID) then
		return true
	end
end
	
else

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

local startSpawnFrame
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
    nextTreeSpawnTime = NewTreeSpawnTime()       
    
    -- fix this, it breaks with luarules reload but no idea why
    local units = Spring.GetAllUnits()
    for _,uID in pairs(units) do
        local uDID = Spring.GetUnitDefID(uID)
        gadget:UnitCreated(uID,uDID)
    end 
end

local _loadFrame
function gadget:GameFrame(frame)
	-- spawn base wave
	if _loadFrame == nil then
		_loadFrame = frame + 3
	end
	if _loadFrame == frame and spawnPoints == nil then
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
	
	local story = Spring.GetGameRulesParam("story")
	if story ~= 0 then
		return
	elseif startSpawnFrame == nil then
		startSpawnFrame = 10 + Spring.GetGameFrame()
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
		--Spring.Echo("SPAWN")
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

local minTreeDist = 225 -- tress are not allowed to be closer together than this 
local maxTreeDist = 325

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
    
    -- cycle over trees in random order, picking up potential spawn points
    local potentialSpawnPoints = {}
    for i=1,#units do
        local uID = units[perm[i]]
        local uDID = Spring.GetUnitDefID(uID)
        if UnitDefs[uDID].customParams.tree then
            local success,t = TryToSpawnANewTreeSomewhereNearToThisTree(uID)
            if success then 
                table.insert(potentialSpawnPoints, t)                    
            end
        end
    end
    
    if #potentialSpawnPoints>0 then
        -- pick potential spawn points with probability proportional to 1/d^2 where d is square distance to spire
        local sx,_,sz = Spring.GetUnitPosition(spireID) 
        local totalWeight = 0
        for i,t in ipairs(potentialSpawnPoints) do
            local d = (sx-t.x)*(sx-t.x) + (sz-t.z)*(sz-t.z)
            local weight = (d>0) and 1/(d*d) or 0
            potentialSpawnPoints[i].weight = weight
            totalWeight = totalWeight + weight
        end
        if totalWeight<=0 then 
            --Spring.Echo("BALLS!")
            return fuck_up 
        end
        
        local p = math.random()*totalWeight
        local q = 0
        for i,t in ipairs(potentialSpawnPoints) do
            q = q + t.weight
            if q>p then
                SpawnATreeHereNOW(t.x,t.z)
                return
            end    
        end
    else
        -- Spring.Echo("OH NOOOO") 
    end
end

function TryToSpawnANewTreeSomewhereNearToThisTree(unitID)
    local x,_,z = Spring.GetUnitPosition(unitID)
    local distInterval = maxTreeDist-minTreeDist
    local circleDivs = 5
    local thetaDiv = 2*math.pi/circleDivs
    local theta = math.random()*thetaDiv
    for i=1,circleDivs do
        theta = theta + thetaDiv
        local r = minTreeDist + math.random()*distInterval
        local tx = x + r*math.cos(theta)
        local tz = z + r*math.sin(theta)
        if (IsSteep(tx,tz)) or (IsTooHigh(tx,tz)) then
            return false, nil
        end
        local success, t = ILoveExcessivelyLongFunctionNames(tx,tz,minTreeDist)
        if success then 
            return true, t
        end
    end
    return false, nil
end

function ILoveExcessivelyLongFunctionNames(tx,tz, minTreeDist)
    -- try to spawn a tree at tx, tz
    -- but first check that we are far enough away from the nearest tree
    local possibleOtherTrees = Spring.GetUnitsInCylinder(tx,tz, minTreeDist)
    for _,uID in ipairs(possibleOtherTrees) do
        local uDID = Spring.GetUnitDefID(uID)
        if UnitDefs[uDID].customParams.tree or uID==spireID then
            return false, nil
        end    
    end
    
    local t = {x=tx,z=tz}
    return true, t
end

function SpawnATreeHereNOW(tx,tz)
    SpawnUnit(treeLevel1DefID, tx, tz)
    SpawnGrass(tx, tz, 3, 5, minTreeDist, 10)
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
GG.SpawnTree = SpawnTree

end