local cap = piece "Cap"

local smallMushroomDefID   = UnitDefNames["smallmushroom"].id
function script.Killed(recentDamage, maxHealth)
	--Explode(cap,SFX.FIRE);
		
	local x,y,z = Spring.GetUnitPiecePosDir(unitID,cap);
	Spring.SpawnCEG("redpop", x, y, z, 0, 2, 0, 10,10);
	
	-- spawn small mushrooms on death
	local x, y, z = Spring.GetUnitPosition(unitID)
	local teamID = Spring.GetUnitTeam(unitID)
	local radius = 300
	local height = 70
	for i = 1, 10 do
		Spring.CreateUnit(smallMushroomDefID, x + math.random() * radius - radius/2, y + height, z + math.random() * radius - radius/2, 0, teamID)
	end
	
	return 0
end

function script.QueryWeapon(num)
	return cap
end

function script.AimFromWeapon(num)
	return cap
end

function script.FireWeapon(weaponID)
	return cap
end

function script.AimWeapon(num, heading, pitch)
	return true
end
