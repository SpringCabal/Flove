function gadget:GetInfo()
   return {
      name      = "Send Love",
      desc      = "Send love mechanics (healing friendly trees)",
      author    = "gajop",
      date      = "December 2015", 
      license   = "GNU GPL, v2 or later",
      layer     = 0,
      enabled   = true
   }
end

if not gadgetHandler:IsSyncedCode() then
	return
end

local weaponName = "zap"
local zapWDID = WeaponDefNames[weaponName] and WeaponDefNames[weaponName].id or nil

local watchedProjectiles = {}
local tempUnits = {}

function gadget:Initialize()
    if zapWDID or (not TESTING_MODE) then
        Script.SetWatchWeapon(zapWDID, true)    
    else
        Spring.Echo("Warning: Could not find Zap Weapon")
    end
end

function gadget:Shutdown()
    Script.SetWatchWeapon(zapWDID, false)
end

function gadget:ProjectileCreated(proID, proOwnerID, weaponDefID)
    if weaponDefID == zapWDID then 
        watchedProjectiles[proID] = true
    end
end

function gadget:ProjectileDestroyed(proID)
	if watchedProjectiles[proID]  then 
		local px,py,pz = Spring.GetProjectilePosition(proID)
		Spring.PlaySoundFile("sounds/fairy dust.wav", 5, px,py,pz)
		watchedProjectiles[proID] = nil
	end
end

function gadget:GameFrame(frame)
	for i = #tempUnits, 1, -1 do
		local unit = tempUnits[i]
		if unit.frame <= frame then
			Spring.DestroyUnit(unit.unitID)
			table.remove(tempUnits, i)
		end
	end
end

local sighs = { "sounds/fairy dust sigh.wav", "sounds/fairy dust sigh2.wav", "sounds/fairy dust sigh3.wav", "sounds/fairy dust sigh4.wav"}
local squeaks = { "sounds/mushroomsqueak.wav", "sounds/mushroomsqueak2.wav", "sounds/mushroomsqueak3.wav", "sounds/mushroomsqueak4.wav", "sounds/mushroomsqueak5.wav" }
function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)

	if weaponDefID == zapWDID then
		local x,y,z = Spring.GetUnitPosition(unitID);

		local unitDef = UnitDefs[unitDefID]
		if not unitDef.customParams.mushroom then
			Spring.SpawnCEG("love_heals", x, y, z, 0, 0, 0, 0)
			if unitDef.customParams.tree then
				local hp, maxhp = Spring.GetUnitHealth(unitID)
				if hp ~= maxhp then
					local indx = math.random(1, #sighs)
					local sigh = sighs[indx]
					Spring.PlaySoundFile(sigh, 10, x, y, z)
				end
			end
			return -damage, 1.0
		end
		
		local indx = math.random(1, #squeaks)
		local squeak = squeaks[indx]
		Spring.PlaySoundFile(squeak, 3, x, y, z)
		Spring.SpawnCEG("love_hurts", x, y, z, 0, 0, 0, 0)
	end
	
    return damage, 1.0
end
