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

local AI_TESTING_MODE = false

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

-- SYNCED ONLY
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

local treesTakeNoDamage = false

local firstSpawnFrame = nil

local waveConfig = {
	[1] = {
		units = {
			[normalMushroomDefID] = 5,
		},
		time = 5,
	},
	[2] = {
		units = {
			[normalMushroomDefID] = 5,
			[smallMushroomDefID] = 3,
		},
		time = 35,
	},
	[3] = {
		units = {
			[bigMushroomDefID] = 3,
			[smallMushroomDefID] = 3,
		},
		time = 60,
	},
	[4] = {
		units = {
			[normalMushroomDefID] = 10,
		},
		time = 90,
	},
	[5] = {
		units = {
			[mushroomclusterDefID] = 1,
		},
		time = 120,
	},
	[6] = {
		units = {
			[bombmushroomDefID] = 1,
			[normalMushroomDefID] = 3,
		},
		time = 150,
	},
	[7] = {
		units = {
			[poisonMushroomDefID] = 1,
			[normalMushroomDefID] = 3,
		},
		time = 190,
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
-- Event Handling
--------------------------------------------------------------------------------

function gadget:Initialize()
	CleanUnits()
	startSpawnFrame = 100 + Spring.GetGameFrame()
end

function gadget:GameFrame(frame)
	local gameFrame = frame - startSpawnFrame
	if gameFrame < 0 then
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
	for id, config in pairs(waveConfig) do
		if not config.spawned and config.time*33 <= gameFrame then
			config.spawned = true
			Spring.Log("spawn", LOG.NOTICE, "Spawning wave " .. tostring(id))
			SpawnWave(config.units)
		end
	end
	
    if gameFrame%15==0 then
        CheckForIdleMushrooms()
    end
end


function SpawnWave(spawnUnits)
    if AI_TESTING_MODE then return end
-- 	local config = waveConfig[currentWave]
-- 	if config == nil then
-- 		return
-- 	end
	
	
	for _, spawnPointID in pairs(spawnPoints) do
		local x, _, z = Spring.GetUnitPosition(spawnPointID)
		for unitDefID, count in pairs(spawnUnits) do
			for i = 1, count do
				SpawnUnit(unitDefID, x, z)
			end
		end
	end
end


--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Mushroom AI
--------------------------------------------------------------------------------

local aiMushrooms = {} 

function CheckForSpire(unitID, unitDefID)
	if unitDefID == spireDefID then
		spireID = unitID
	end
end

function gadget:UnitCreated(unitID, unitDefID)
	CheckForSpire(unitID, unitDefID)
    
    if UnitDefs[unitDefID].customParams.mushroom and not (Spring.GetUnitRulesParam(unitID, "aiDisabled")==1) then
        RegisterMushroom(unitID)
    end
end

function gadget:UnitDestroyed(unitID, unitDefID)
    if UnitDefs[unitDefID].customParams.mushroom then
        DeregisterMushroom(unitID)
    end
end

function RegisterMushroom(uID)
    aiMushrooms[uID] = true
end

function DeregisterMushroom(uID)
    aiMushrooms[uID] = nil
end

function SelectEnemy(uID)
    local nID = Spring.GetUnitNearestEnemy(uID, 10240, true)
    local x,y,z = Spring.GetUnitPosition(uID)
    if math.random()<0.5 and nID~=spireID then
        return nID
    else    
        -- knuth shuffle, because Spring.GetAllUnits order is predictable
        local units = Spring.GetAllUnits(tID)
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
        
        -- sample a random enemy with probability proportional to 1 / square distance from self
        local tID = Spring.GetUnitTeam(uID)
        local weights = {}
        local totalWeight = 0
        for i=1,#units do
            local eID = units[perm[i]]
            local eTeamID = Spring.GetUnitTeam(eID)
            local eDID = Spring.GetUnitDefID(eID)
            if UnitDefs[eDID].customParams.tree and not Spring.AreTeamsAllied(tID, eTeamID) then
                local ex,ey,ez = Spring.GetUnitPosition(eID)
                local sqrDist = (x-ex)*(x-ex) + (y-ey)*(y-ey) + (z-ez)*(z-ez) 
                weights[eID] = (sqrDist>10*10) and 1/(sqrDist) or 0
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
        if  #Spring.GetUnitCommands(uID,2)==0 then
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
    Spring.GiveOrderToUnit(uID, CMD.FIGHT, {nx,ny,nz}, {})    
end

function StandUpAndFightLikeAMan(uID,x,y,z)
    Spring.GiveOrderToUnit(uID, CMD.FIGHT, {x,y,z}, {})
end


--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Trees Special Modes
--------------------------------------------------------------------------------

function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)
    if not UnitDefs[unitDefID].customParams.tree then
        return damage,0
    end
    
    if treesTakeNoDamage then
        return 0,0
    end    
    
    return damage,0    
end

function DamageTrees(damageTreeP)
    -- set all trees to have health approximately p of their full health (plus a bit of randomness)
    local units = Spring.GetAllUnits()
    for _,uID in ipairs(units) do
        local unitDefID = Spring.GetUnitDefID(uID)
        if UnitDefs[unitDefID].customParams.tree then
            local h,mh = Spring.GetUnitHealth(uID)
            local deviation = 0.2
            local newH = math.max(0.05*mh, math.min(0.95*mh, mh*damageTreeP + deviation*mh*2*(math.random()-1) ) )
            Spring.SetUnitHealth(uID, newH)        
        end        
    end
end
