function gadget:GetInfo()
   return {
      name      = "Tree Upgrader",
      desc      = "Upgrades affected units",
      author    = "SpringCabal",
      date      = "LD33", 
      license   = "GNU GPL, v2 or later",
      layer     = 0,
      enabled   = true
   }
end

if not gadgetHandler:IsSyncedCode() then
	return
end

local weaponName = "upgradeshot"
local WDID = WeaponDefNames[weaponName] and WeaponDefNames[weaponName].id or nil
local watchedProjectiles = {}

function gadget:Initialize()
	-- watch the FlowerShot weapon (from the spire unit)
    if WDID or (not TESTING_MODE) then
        Script.SetWatchWeapon(WDID,true)    
    else
        Spring.Echo("Warning: Could not find Upgrade Shot Weapon")
    end
end

function gadget:Shutdown()
    Script.SetWatchWeapon(WDID,false)    
end

function gadget:ProjectileCreated(proID, proOwnerID, weaponDefID)
    if weaponDefID==WDID then 
        watchedProjectiles[proID] = true
    end
end

local function DistanceCompare(a, b)
	return a.length < b.length
end

function gadget:ProjectileDestroyed(proID)
    if watchedProjectiles[proID] then  
		local upgraded = false
		-- upgrades
		local px,py,pz = Spring.GetProjectilePosition(proID)
		local units = Spring.GetUnitsInCylinder(px, pz, 100)
		
		local trees = {}
		for i=1, #units do
			local uID = units[i]
			local uDID = Spring.GetUnitDefID(uID)
			local uDef = UnitDefs[uDID]
			if uDef.customParams.tree then
				local x, y, z = Spring.GetUnitPosition(uID)
				local dx, dy, dz = x - px, y - py, z - pz
				local d = dx * dx + dy * dy + dz * dz
				table.insert(trees, {uID = uID, length = d})
			end
		end
		table.sort(trees, DistanceCompare)
		for i = 1, #trees do
			local uID = trees[i].uID
			if GG.AddUpgradeProgress(uID) then
				Spring.PlaySoundFile("sounds/wind+leaves.wav", 10, px, py, pz)
				upgraded = true
				break
			end
		end
    end
	watchedProjectiles[proID] = nil
end
