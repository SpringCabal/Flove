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

local AI_TESTING_MODE = true

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
	--	[smallMushroomDefID] = 3,
	},
}

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local function SpawnUnit(unitDefID, x, z, noRotate)
    x = x+50*(math.random()-1)
    z = z+50*(math.random()-1)
	local unitID = Spring.CreateUnit(unitDefID, x, 0, z, 0, 0, false, false)
	if not noRotate then
		Spring.SetUnitRotation(unitID, 0, math.random()*2*math.pi, 0)
	end
	local sx, sy, sz = Spring.GetUnitPosition(spireID)
	--Spring.GiveOrderToUnit(unitID, CMD.MOVE, {sx, sy, sz}, 0 )
end

local function CleanUnits()
	for _, unitID in ipairs(Spring.GetAllUnits()) do
		local unitDefID = Spring.GetUnitDefID(unitID)
		local unitDef = UnitDefs[unitDefID]
		if unitDef.customParams.mushroom then
			Spring.DestroyUnit(unitID, false, false)
		else
			CheckForSpire(unitID, unitDefID)
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
	if frame % 33 * 10 == 0 then
		SpawnWave()
	end
    if frame%15==0 then
        CheckForIdleMushrooms()
    end
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
			for i = 1, count do
				SpawnUnit(unitDefID, x, z)
			end
		end
	end
end

function CheckForSpire(unitID, unitDefID)
	if unitDefID == spireDefID then
		spireID = unitID
	end
end

function gadget:UnitCreated(unitID, unitDefID)
	CheckForSpire(unitID, unitDefID)
    
    if UnitDefs[unitDefID].customParams.mushroom and (Spring.GetUnitRulesParam(unitID, "aiEnabled")==1 or AI_TESTING_MODE) then
        RegisterMushroom(unitID)
    end
end

function gadget:UnitDestroyed(unitID, unitDefID)
    if UnitDefs[unitDefID].customParams.mushroom then
        DeregisterMushroom(unitID)
    end
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local aiMushrooms = {} 

function RegisterMushroom(uID)
    Spring.Echo(uID)
    aiMushrooms[uID] = true
end

function DeregisterMushroom(uID)
    aiMushrooms[uID] = nil
end

function SelectEnemy(uID)
    local nID = Spring.GetUnitNearestEnemy(uID, 5120, true)
    local x,y,z = Spring.GetUnitPosition(uID)
    if math.random()<0.5 and nID~=spireID then
        return nID
    else
        -- sample a random enemy with probability proportional to 1 / square distance from self
        local tID = Spring.GetUnitTeam(uID)
        local units = Spring.GetAllUnits(tID)
        local weights = {}
        local totalWeight = 0
        for _,eID in pairs(units) do
            local eTeamID = Spring.GetUnitTeam(eID)
            if not Spring.AreTeamsAllied(eTeamID, tID) and eID~=spireID then
                local ex,ey,ez = Spring.GetUnitPosition(eID)
                local sqrDist = (x-ex)*(x-ex) + (y-ey)*(y-ey) + (z-ez)*(z-ez) 
                weights[eID] = 1/(sqrDist)
                totalWeight = totalWeight + weights[eID]
            end
        end
        if totalWeight<=0 then
            return nil
        end
        local p = math.random()*totalWeight
        local q = 0
        for eID,w in pairs(weights) do
            q = q + w
            if q>p then 
                return eID
            end
        end        
    end
    return nil
end

function CheckForIdleMushrooms()
    for uID,_ in pairs(aiMushrooms) do
        if  Spring.GetCommandQueue(uID,-1,false)==0 then
            local eID = SelectEnemy(uID)
            if eID then
                local x,y,z = Spring.GetUnitPosition(eID)
                local cx,cy,cz = Spring.GetUnitPosition(uID)
                StandUpAndFightLikeAMan(uID,x,y,z)
            else
                GoForAShortWalk(uID)
            end
        end        
    end    
end

function GoForAShortWalk(uID)
    local x,y,z = Spring.GetUnitPosition(uID)
    local theta = math.random(1,360) / 360 * (2*math.pi)
    local dx, dz = 256*math.sin(theta), 256*math.cos(theta)
    local nx, ny, nz = x+dx, Spring.GetGroundHeight(x+dx,z+dz), z+dz
    Spring.GiveOrderToUnit(uID, CMD.MOVE, {nx,ny,nz}, {})    
end

function StandUpAndFightLikeAMan(uID,x,y,z)
    Spring.GiveOrderToUnit(uID, CMD.FIGHT, {x,y,z}, {})
end