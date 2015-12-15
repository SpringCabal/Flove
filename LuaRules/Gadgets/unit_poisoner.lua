function gadget:GetInfo()
   return {
      name      = "Poisoner",
      desc      = "Kills Flowers",
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

local weaponName = "poison"
local unitName = "poisonmushroom"
local WDID = WeaponDefNames[weaponName] and WeaponDefNames[weaponName].id or nil
local poisonUDID = UnitDefNames[unitName].id;
local watchedProjectiles = {}
local watchedUnits = {}

function gadget:Initialize()
   Script.SetWatchWeapon(WDID,true)    
end

function gadget:UnitCreated(unitID, unitDefID)
	if(unitDefID == poisonUDID) then
		watchedUnits[unitID] = Spring.GetGameFrame();
	end
end

function gadget:Shutdown()
    Script.SetWatchWeapon(WDID,false)    
end

function gadget:GameFrame(f)
	for unitID, frame in pairs(watchedUnits) do
		if f % 30 == 0 then
			local x,y,z = Spring.GetUnitPosition(unitID);
			local params = {
				pos = {x, y, z},
				['end'] = {x,y,z},
				maxRange = 2,
				speed = {0,0,0},
				owner  = unitID,
				ttl = 1,
				gravity = 0,
			}
			Spring.SpawnProjectile(WDID, params)
		end
	end
end

function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)
	if(weaponDefID == WDID) then
		if(UnitDefs[unitDefID].customParams.flower) then
			local x,y,z = Spring.GetUnitPosition(unitID);
			Spring.SpawnCEG("barftrail", x, y, z, 0, 0, 0, 0);
			Spring.DestroyUnit(unitID);
		end
		return 0,0;
	end
	
	return damage, paralyzer;
end
