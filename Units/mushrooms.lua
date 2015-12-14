local Mushroom = Unit:New {	
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
    MaxVelocity = 62, -- max velocity is so high because Spring cannot increase max velocity beyond its maximum.
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
	
	idleAutoHeal = 0,
	script		 	= "UnitBase.lua",
	customparams = {  
		basepiece	= "Trunk",
		moveanim	= "headers/bipedal_mushroom_movement.lua", 
		mountanimdefaultto	 = "headers/weaponmounts/shroommouth.lua",
		--deathanim   = "headers/death/bipedfallover.lua",
		centeraim	= "Trunk",
		
	},

}

local NormalMushroom = Mushroom:New {
	name                = "Normal Mushroom",
	maxDamage			= 600,
	MaxVelocity 		= 90,
	FootprintX = 3,
    FootprintZ = 3,
	weapons = {
		[1]  = {
		name               = "normalmushroomspray",
		onlyTargetCategory = "TREE",
		mainDir				= "0 0 1",
		maxAngleDif			= 210,	
		},
	},
}
local SmallMushroom = Mushroom:New {
	name                = "Small Mushroom",
	maxDamage			= 600,
	MaxVelocity 		= 280,
	Acceleration		= 0.8,
	TurnInPlace 		= true,
	weapons = {
		[1]  = {
		name               = "smallmushroomspray",
		onlyTargetCategory = "TREE",
		mainDir				= "0 0 1",
		maxAngleDif			= 210,	
		},
	},
}
local BigMushroom = Mushroom:New {
	name                = "Big Mushroom",
	maxDamage			= 2000,
	MaxVelocity 		= 20,
	FootprintX = 4,
    FootprintZ = 4,
	weapons = {
		[1]  = {
		name               = "bigmushroomspray",
		onlyTargetCategory = "TREE",
		mainDir				= "0 0 1",
		maxAngleDif			= 210,	
		},
	},
}
local MushroomCluster = Mushroom:New {
	name                = "Mushroom Cluster",
	maxDamage			= 1000,
	MaxVelocity 		= 50,
	FootprintX = 5,
    FootprintZ = 5,
	customparams = {  
		deathanim   = "headers/death/clustershroom.lua",	
	},
	weapons = {
		[1]  = {
		name               = "mushroomclusterspray",
		onlyTargetCategory = "TREE",
		mainDir				= "0 0 1",
		maxAngleDif			= 210,	
		},
	},
}
local PoisonMushroom = Mushroom:New {
	name                = "Poison Mushroom",
	maxDamage			= 1500,
	FootprintX = 3,
    FootprintZ = 3,

	weapons = {
		[1]  = {
		name               = "poisonmushroomspray",
		onlyTargetCategory = "TREE",
		mainDir				= "0 0 1",
		maxAngleDif			= 210,	
		},
	},
}
local BombMushroom = Mushroom:New {
	name                = "Bomb Mushroom",
	maxDamage			= 700,
	MaxVelocity 		= 110,
	customparams = {  
		deathanim   = "headers/death/bombshroom.lua",	
	},
	
	kamikaze               = true,
    kamikazeDistance       = 80,
    kamikazeUseLOS         = true,
	selfDestructAs         = [[BOMB_MUSHROOM_DEATH]],
	selfDestructCountdown  = 0,
	weaponDefs = {
		BOMB_MUSHROOM_DEATH = {
			areaOfEffect       = 384,
			craterBoost        = 1,
			craterMult         = 3.5,
			edgeEffectiveness  = 0.4,
-- 			explosionGenerator = "custom:ROACHPLOSION",
			explosionSpeed     = 10000,
			impulseBoost       = 0,
			impulseFactor      = 0.3,
			name               = "Explosion",
 			soundHit           = "sounds/mushroomsqueaksplosion-bomb.wav",
			damage = {
				default          = 800,
			},
		}
	}
}

return lowerkeys({
    NormalMushroom       = NormalMushroom,
	BigMushroom          = BigMushroom,
	SmallMushroom        = SmallMushroom,
	MushroomCluster      = MushroomCluster,
	PoisonMushroom       = PoisonMushroom,
	BombMushroom         = BombMushroom,
})
