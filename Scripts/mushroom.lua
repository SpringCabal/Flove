local head = piece "head"

function script.Killed(recentDamage, maxHealth)
	--Explode(head,SFX.FIRE);
		
	local x,y,z = Spring.GetUnitPiecePosDir(unitID,head);
	Spring.SpawnCEG("redpop", x, y, z, 0, 2, 0, 10,10);
	
	return 0
end

function script.QueryWeapon(num)
	return head
end

function script.AimFromWeapon(num)
	return head
end

function script.FireWeapon(weaponID)
	return head
end

function script.AimWeapon(num, heading, pitch)
	return true
end
