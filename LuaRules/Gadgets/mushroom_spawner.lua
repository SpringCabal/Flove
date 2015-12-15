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
local kingshroomDefID      = UnitDefNames["kingshroom"].id

local treeLevel2DefID      = UnitDefNames["treelevel2"].id

local tunneltube1DefID  = UnitDefNames["tunneltube1"].id
local tunneltube2DefID  = UnitDefNames["tunneltube2"].id
local tunneltube3DefID  = UnitDefNames["tunneltube3"].id
local tunnelTubes = { tunneltube1DefID, tunneltube2DefID, tunneltube3DefID }

local spireDefID = UnitDefNames["spire"].id
local spireID = nil

local currentWave = 0
local startSpawnFrame
local spawnPoints = nil

-- story stuff
local treesTakeNoDamage = false
local storyMushrooms = {}
local storyWaveSpawnTime

local firstSpawnFrame = nil

local waveConfig = {
	[1] = { -- first wave is spawned as part of the story
		units = {
			[normalMushroomDefID] = 3,
		},
		time = 0,
	},
	[2] = {
		units = {
			[normalMushroomDefID] = 3,
			[smallMushroomDefID] = 3,
		},
		time = 5,
	},
	[3] = {
		units = {
			[bigMushroomDefID] = 1,
			[smallMushroomDefID] = 3,
		},
		time = 50,
	},
	[4] = {
		units = {
			[normalMushroomDefID] = 6,
		},
		time = 100,
	},
	[5] = {
		units = {
			[mushroomclusterDefID] = 1,
		},
		time = 150,
	},
	[6] = {
		units = {
			[normalMushroomDefID] = 3,
			[bigMushroomDefID] = 1,
		},
		time = 200,
	},
	[7] = {
		units = {
			[poisonMushroomDefID] = 1,
			[normalMushroomDefID] = 10,
		},
		time = 250,
	},
	[8] = {
		units = {
			[normalMushroomDefID] = 6,
			[smallMushroomDefID] = 2,
		},
		time = 300,
	},
	[9] = {
		units = {
			[smallMushroomDefID] = 10,
			[bigMushroomDefID] = 2,
		},
		time = 350,
	},
	[10] = {
		units = {
			[normalMushroomDefID] = 3,
			[smallMushroomDefID] = 4,
		},
		time = 400,
	},
	[11] = {
		units = {
			[bombmushroomDefID] = 1,
		},
		time = 450,
	},
	[12] = {
		units = {
			[mushroomclusterDefID] = 3,
			[poisonMushroomDefID] = 1,
		},
		time = 500,
	},
	[13] = {
		units = {
			[normalMushroomDefID] = 5,
			[smallMushroomDefID] = 15,
			[bigMushroomDefID] = 3,
		},
		time = 550,
	},
	[14] = {
		units = {
			[bombmushroomDefID] = 3,
			[bigMushroomDefID] = 3,
			[normalMushroomDefID] = 3,
			[mushroomclusterDefID] = 2,
		},
		time = 600,
		spawnKing = true,
	},
	[15] = {
		units = {
			[smallMushroomDefID] = 10,
			[normalMushroomDefID] = 7,
			[bigMushroomDefID] = 2,
		},
		time = 650,
		repeatWave = true, -- we'll repeat this one
		repeatTime = 50,
	},
}

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local function SpawnUnit(unitDefID, x, z, noRotate)
    x = x+20*(2*math.random()-1)
    z = z+20*(2*math.random()-1)
	local unitID = Spring.CreateUnit(unitDefID, x, 0, z, 0, 0, false, false)
	if not noRotate then
		Spring.SetUnitRotation(unitID, 0, math.random()*2*math.pi, 0)
	end
	return unitID
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- Event Handling
--------------------------------------------------------------------------------

function gadget:Initialize()
	Spring.SetGameRulesParam("story", 1)
	Spring.SetGameRulesParam("skip_tutorial", 0)
	Spring.SetGameRulesParam("shroomEvent", 0)
	for _, unitID in ipairs(Spring.GetAllUnits()) do
		local unitDefID = Spring.GetUnitDefID(unitID)
		self:UnitCreated(unitID, unitDefID)
	end
end

local _loadFrame
function gadget:GameFrame(frame)
	if _loadFrame == nil then
		_loadFrame = frame + 3
	end
	if _loadFrame == frame and spawnPoints == nil then
		spawnPoints = {}
		for _, unitID in ipairs(Spring.GetAllUnits()) do
			local unitDefID = Spring.GetUnitDefID(unitID)
			if unitDefID == mushroomSpawnDefID then
				table.insert(spawnPoints, unitID)
				local x, _, z = Spring.GetUnitPosition(unitID)
				local TUBE_RADIUS = 800
				for i = 1, 12 do
					local indx = math.random(1, 3)
					local tunnelTubeDef = tunnelTubes[indx]
					local ux = x + math.random()*TUBE_RADIUS - TUBE_RADIUS/2
					local uz = z + math.random()*TUBE_RADIUS - TUBE_RADIUS/2
					SpawnUnit(tunnelTubeDef, ux, uz)
				end
			end
		end
	end
	
	if frame%15==0 then
        CheckForIdleMushrooms()
    end
	
	local story = Spring.GetGameRulesParam("story")
	if story ~= 0 then
		if CheckTreeHP() then
			return
		end
		if storyWaveSpawnTime ~= nil and frame - storyWaveSpawnTime > 30 then
			if CheckMushrooms() then
				return
			end
		end
		if story ~= 5 then
			Spring.SetGameRulesParam("mana", 0)
		end
		if CheckUpgradedTree() then
			return
		end
		
		return
	elseif startSpawnFrame == nil then
		startSpawnFrame = 100 + Spring.GetGameFrame()
	end
	local gameFrame = frame - startSpawnFrame
	if gameFrame < 0 then
		return
	end
	SpawnNextWave()
end

function SpawnNextWave(force)
	local frame = Spring.GetGameFrame()
	local gameFrame = frame - (startSpawnFrame or 0)
	for i = 1, 100 do
		local config = waveConfig[i]
		if config and (not config.spawned or (config.repeatWave and (config.lastFrame == nil or gameFrame - config.lastFrame > config.repeatTime*33))) and (config.time*33 <= gameFrame or force) then
			config.spawned = true
			config.lastFrame = gameFrame
			--Spring.Log("spawn", LOG.NOTICE, "Spawning wave " .. tostring(i))
			SpawnWave(config.units, config.spawnKing)
			break
		end
	end
end

function SpawnWave(spawnUnits, spawnKing)
    if AI_TESTING_MODE then return end
-- 	local config = waveConfig[currentWave]
-- 	if config == nil then
-- 		return
-- 	end
	
	for indx, spawnPointID in pairs(spawnPoints) do
		local x, _, z = Spring.GetUnitPosition(spawnPointID)
		if spawnKing and indx == 2 then
			SpawnUnit(kingshroomDefID, x, z)
		end
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

	if unitDefID == kingshroomDefID then
		Spring.SetGameRulesParam("shroomEvent", 1)
	end
    if UnitDefs[unitDefID].customParams.mushroom and not (Spring.GetUnitRulesParam(unitID, "aiDisabled")==1) then
		aiMushrooms[unitID] = true
    end
end

function gadget:UnitDestroyed(unitID, unitDefID)
	local stage = Spring.GetGameRulesParam("story")
	if stage == 2 then
		storyMushrooms[unitID] = nil
		local c = 0
		for _, _ in pairs(storyMushrooms) do 
			c = c + 1
		end
		if c == 0 then
			StoryStage(3)
		end
	end
	
	if unitDefID == kingshroomDefID then
		Spring.SetGameRulesParam("shroomEvent", 2)
	end
	
	aiMushrooms[unitID] = nil
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
            if (UnitDefs[eDID].customParams.tree or spireDefID == eDID) and not Spring.AreTeamsAllied(tID, eTeamID) then
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

local SMALL_MUSHROOM_ORDER_CHANGE = 30
function CheckForIdleMushrooms()
	local frame = Spring.GetGameFrame()
    for uID, _ in pairs(aiMushrooms) do
		local unitDefID = Spring.GetUnitDefID(uID)
		if unitDefID == smallMushroomDefID then
			local lastOrderFrame = Spring.GetUnitRulesParam(uID, "lastOrderFrame") or 0
			if frame - lastOrderFrame >= SMALL_MUSHROOM_ORDER_CHANGE then
				lastOrderFrame = frame
				Spring.SetUnitRulesParam(uID, "lastOrderFrame", lastOrderFrame)
				
				local eID = SelectEnemy(uID)
				if eID then
					local x,y,z = Spring.GetUnitPosition(eID)
					local d = 500
					local dx = x + math.random()*d - d/2
					local dz = z + math.random()*d - d/2
					local dy = Spring.GetGroundHeight(dx, dz)
					Spring.GiveOrderToUnit(uID, CMD.MOVE, {dx, dy, dz}, {}) 
				end
			end
		elseif Spring.GetUnitCommands(uID,2) ~= nil and #Spring.GetUnitCommands(uID,2)==0 then
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

function CheckTreeHP()
	local stage = Spring.GetGameRulesParam("story")
	if stage ~= 3 then
		return
	end

	local units = Spring.GetAllUnits()
	local fullHP = true
    for _, uID in ipairs(units) do
        local unitDefID = Spring.GetUnitDefID(uID)
        if UnitDefs[unitDefID].customParams.tree then
            local h,mh = Spring.GetUnitHealth(uID)
			if mh * 0.8 > h then
				fullHP = false
			end
		end
	end
	if fullHP then
		StoryStage(4)
		return true
	end
end

function CheckMushrooms()
	local stage = Spring.GetGameRulesParam("story")
	if stage ~= 4 then
		return
	end
	local noshroomsC = 0
	local units = Spring.GetAllUnits()
	for _, uID in ipairs(units) do
        local unitDefID = Spring.GetUnitDefID(uID)
        if UnitDefs[unitDefID].customParams.mushroom then
            noshroomsC = noshroomsC + 1
		end
	end
	-- there always seems to be a few left...
	if noshroomsC <= 2 then
		StoryStage(5)
		return true
	end
end

function CheckUpgradedTree()
	local stage = Spring.GetGameRulesParam("story")
	if stage ~= 5 then
		return
	end
	local units = Spring.GetAllUnits()
	for _, uID in ipairs(units) do
        local unitDefID = Spring.GetUnitDefID(uID)
        if unitDefID == treeLevel2DefID then
			StoryStage(6)
            return true
		end
	end
end

----------------------
--- Story handling
----------------------

local function explode(div,str)
	if (div=='') then return 
		false 
	end
	local pos,arr = 0,{}
	-- for each divider found
	for st,sp in function() return string.find(str,div,pos,true) end do
		table.insert(arr,string.sub(str,pos,st-1)) -- Attach chars left of current divider
		pos = sp + 1 -- Jump past current divider
	end
	table.insert(arr,string.sub(str,pos)) -- Attach chars right of last divider
	return arr
end

function StoryStage(stage)
	Spring.Echo("Stage: " .. tostring(stage))
	if stage == 2 then
		Spring.SetGameRulesParam("story", 2)
		-- FIXME: something is broken with this call
-- 		local sx, sy, sz = Spring.GetUnitPosition(spireID) 
		-- FIXME: hardcoding...
		local sx, sy, sz = 4688, 1370.5444335938, 5552
		for i = 1, 5 do
			local unitID = SpawnUnit(normalMushroomDefID, sx - 400 + math.random() * 800, sz + 600 + math.random() * 100)
			storyMushrooms[unitID] = true
		end
		treesTakeNoDamage = true
	elseif stage == 3 then
		treesTakeNoDamage = false
		DamageTrees(0.5)
		Spring.SetGameRulesParam("story", 3)
	elseif stage == 4 then
		Spring.SetGameRulesParam("story", 4)
		SpawnNextWave(true)
		storyWaveSpawnTime = Spring.GetGameFrame()
	elseif stage == 5 then
 		Spring.SetGameRulesParam("story", 5)
		GG.SpawnTree()
		GG.SpawnTree()
		GG.SpawnTree()
		Spring.SetGameRulesParam("mana", 100)
	elseif stage == 6 then-- story ends here
		Spring.SetGameRulesParam("story", 6)
	elseif stage == 7 then
		Spring.SetGameRulesParam("story", 0)
		Spring.SetGameRulesParam("mana", 0)
	end
end

function HandleLuaMessage(msg)
	local msg_table = explode('|', msg)
	if msg_table[1] == 'story' then
		local shroomEvent = Spring.GetGameRulesParam("shroomEvent") or 0
		if shroomEvent == 2 then
			Spring.SetGameRulesParam("gameOver", 1)
			Spring.SetGameRulesParam("won", 1)
			return
		end
		local stage = Spring.GetGameRulesParam("story")
		stage = stage + 1
		if stage ~= 3 and stage ~= 4 and stage ~= 5 and stage ~= 6 then
			StoryStage(stage)
		end
	elseif msg_table[1] == 'skip_tutorial' then
		Spring.SetGameRulesParam("skip_tutorial", 1)
		StoryStage(7)
		GG.SpawnTree()
		GG.SpawnTree()
		GG.SpawnTree()
		Spring.SetGameRulesParam("mana", 30)
		waveConfig[1].spawned = true -- make the first wave spawned (its used in the tutorial)
	elseif msg_table[1] == 'difficulty' then
		local difficulty = tonumber(msg_table[2])
		Spring.SetGameRulesParam("difficulty", difficulty)
		
		if difficulty >= 2 then
			table.insert(waveConfig, {
				units = {
					[normalMushroomDefID] = 3,
					[smallMushroomDefID] = 3,
				},
				time = 75,
				repeatWave = true, -- we'll repeat this one
				repeatTime = 100,
			})
			table.insert(waveConfig, {
				units = {
					[mushroomclusterDefID] = 1,
				},
				time = 90,
				repeatWave = true, -- we'll repeat this one
				repeatTime = 100,
			})
		end
		if difficulty >= 3 then
			table.insert(waveConfig, {
				units = {
					[poisonMushroomDefID] = 1,
					[bigMushroomDefID] = 2,
				},
				time = 230,
				repeatWave = true, -- we'll repeat this one
				repeatTime = 60,
			})
			table.insert(waveConfig, {
				units = {
					[bigMushroomDefID] = 3,
				},
				time = 166,
				repeatWave = true, -- we'll repeat this one
				repeatTime = 66,
			})
		end
	end
end


function gadget:RecvLuaMsg(msg)
	HandleLuaMessage(msg)
end
