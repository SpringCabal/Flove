function gadget:GetInfo()
   return {
      name      = "Field Of Flowers",
      desc      = "Makes units slow down within a given region ",
      author    = "rawr",
      date      = "rawr", 
      license   = "GNU GPL, v2 or later",
      layer     = 0,
      enabled   = true
   }
end

if not gadgetHandler:IsSyncedCode() then
	return
end


local duration = 10 * 30 -- in simframes
local radius = 300 -- in elmos
local speedFactor = 0.25 -- multipicative factor

local TESTING_MODE = false
local AUTO_MODE = false
local aMushroomID = UnitDefNames["bigmushroom"].id

local weaponName = "flowershot"
local flowerShotWDID = WeaponDefNames[weaponName] and WeaponDefNames[weaponName].id or nil
local watchedProjectiles = {}
local watchedCenters = {}
local watchedUnits = {}
local coveredUnits = {} -- units that, on this simframe, we slow down
local tempUnits = {}

function gadget:Initialize()
	-- watch the FlowerShot weapon (from the spire unit)
    if flowerShotWDID or (not TESTING_MODE) then
        Script.SetWatchWeapon(flowerShotWDID,true)    
    else
        Spring.Echo("Warning: Could not find Flower Shot Weapon")
    end
end

function gadget:Shutdown()
    Script.SetWatchWeapon(flowerShotWDID,false)    
end

function gadget:ProjectileCreated(proID, proOwnerID, weaponDefID)
    if weaponDefID==flowerShotWDID then 
        watchedProjectiles[proID] = true
    end
end

function gadget:ProjectileDestroyed(proID)
    if watchedProjectiles[proID] then
    
        local px,py,pz = Spring.GetProjectilePosition(proID)
        local gy = Spring.GetGroundHeight(px,pz)
        if math.abs(py-gy)<5 or true then
            -- cause it to happen
     		local frame = Spring.GetGameFrame()

            if TESTING_MODE then Spring.Echo("Adding new center at ", px, pz) end
            table.insert(watchedCenters, {x=px, z=pz, f=Spring.GetGameFrame()})
            
			local units = GG.SpawnFlowers(px, pz, 2, 4, 500)
			local units2 = GG.SpawnGrass(px, pz, 2, 4, 500)
			for _, unitID in pairs(units) do
				table.insert(tempUnits, {unitID = unitID, frame = frame + duration -15 +math.random(30)})
			end
			for _, unitID in pairs(units2) do
				table.insert(tempUnits, {unitID = unitID, frame = frame + duration -15 +math.random(30)})
			end
			local units = Spring.GetUnitsInCylinder(px, pz, 200)
			for i=1,#units do
				local uID = units[i]
				local uDID = Spring.GetUnitDefID(uID)
				local uDef = UnitDefs[uDID]
				if uDef.customParams.tree then
					if GG.AddUpgradeProgress(uID) then
						break
					end
				end
			end
        end
    end
end

function gadget:UnitDestroyed(uID)
    if watchedUnits[uID] then
        watchedUnits[uID] = nil
    end
end

function ChangeSpeed(uID, mult)
    Spring.SetUnitRulesParam(uID, "selfMoveSpeedChange", mult)
    GG.UpdateUnitAttributes(uID)
    if TESTING_MODE then Spring.Echo("Changing speed by factor "..mult.." for unit "..uID) end
end

function gadget:GameFrame(frame)
    coveredUnits = {}
    
    -- list all covered units
    -- watch newly covered units, and slow them
    for i=1,#watchedCenters do
        local c = watchedCenters[i]
        local units = Spring.GetUnitsInCylinder(c.x, c.z, radius)
        for i=1,#units do
            local uID = units[i]
            coveredUnits[uID] = true   
            local uDID = Spring.GetUnitDefID(uID)
            if not watchedUnits[uID] then
                local speed = UnitDefs[uDID].speed
                if TESTING_MODE then Spring.Echo("Now watching "..uID..", with original speed "..speed) end
                watchedUnits[uID] = speed
                ChangeSpeed(uID, speedFactor)
            end
        end        
    end
    
    -- speed up no longer covered units
    for uID,_ in pairs(watchedUnits) do
        if not coveredUnits[uID] then
            ChangeSpeed(uID, 1)
            watchedUnits[uID] = nil
        end    
    end
    
    -- remove no longer covered centers
    local i = 1
    while i<=#watchedCenters do
        local c = watchedCenters[i]
        local inForce = (c.f+duration) > frame
        if not inForce then
            if TESTING_MODE then Spring.Echo("Removing center at ", c.x, c.z) end
            table.remove(watchedCenters,i)
        else
            i = i + 1
        end        
    end
    
    for i = #tempUnits, 1, -1 do
		local unit = tempUnits[i]
		if unit.frame <= frame then
			Spring.DestroyUnit(unit.unitID)
			table.remove(tempUnits, i)
		end
	end
    
    if AUTO_MODE and frame%(duration+3*30)==0 then
        local dummyCenter = {x=4100, z=4100, f=Spring.GetGameFrame()}
        table.insert(watchedCenters, dummyCenter)
        Spring.Echo("Placed dummy center at 6000,6000 for duration "..tostring(duration))
        Spring.CreateUnit(aMushroomID, 4100, Spring.GetGroundHeight(4100,4100), 4100, 0, 0)
        Spring.CreateUnit(aMushroomID, 4800, Spring.GetGroundHeight(4800,4800), 4800, 0, 0)
    end
end
