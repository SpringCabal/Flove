local Mushroom = Unit:New {
    script = "mushroom.lua",
	
    RadarDistance = 0,
    SightDistance = 400,
    Upright = 0,
	
	-- Transporting
	releaseHeld = true,
	holdSteady = true,
	transportCapacity = 10,
	transportSize = 10,
	
	--Pathfinding and related
    Acceleration = 0.2,
    BrakeRate = 0.1,
    FootprintX = 2,
    FootprintZ = 2,
    MaxSlope = 15,
    MaxVelocity = 32, -- max velocity is so high because Spring cannot increase max velocity beyond its maximum.
    MaxWaterDepth = 20,
    MovementClass = "Bot2x2",
	TurnInPlace = false,
	TurnInPlaceSpeedLimit = 1.6, 
	turnInPlaceAngleLimit = 90,
    TurnRate = 3000,
	
	customParams = {
		turnaccel = 500,
		mushroom = true,
	},

	--Abilities
    Builder = 0,
    CanAttack = 1,
    CanGuard = 1,
    CanMove = 1,
    CanPatrol = 1,
    CanStop = 1,
    LeaveTracks = 0,
    Reclaimable = 0,
	
	--Hitbox
	collisionVolumeOffsets    =  "0 16 0",
	collisionVolumeScales     =  "64 64 64",
	collisionVolumeType       =  "sphere",

}

local NormalMushroom = Mushroom:New {
	name                = "Normal Mushroom",
	maxDamage			= 3,
	weapons             = {
		{
			def                = "normalmushroomspray",
		},
	},
}
local SmallMushroom = Mushroom:New {
	name                = "Small Mushroom",
	maxDamage			= 5,
}
local BigMushroom = Mushroom:New {
	name                = "Big Mushroom",
	maxDamage			= 7,
}
local MushroomCluster = Mushroom:New {
	name                = "Mushroom Cluster",
	maxDamage			= 3,
}
local PoisonMushroom = Mushroom:New {
	name                = "Poison Mushroom",
	maxDamage			= 5,
}
local BombMushroom = Mushroom:New {
	name                = "Bomb Mushroom",
	maxDamage			= 7,
}

return lowerkeys({
    NormalMushroom       = NormalMushroom,
	BigMushroom          = BigMushroom,
	SmallMushroom        = SmallMushroom,
	MushroomCluster      = MushroomCluster,
	PoisonMushroom       = PoisonMushroom,
	BombMushroom         = BombMushroom,
})
