local BaseEffect = Unit:New {
    customParams        = {
        effect = true,
        -- invulnerable means that most instances are invulnerable through normal damage and effects (could still be manually destroyed)
        invulnerable = 1,
    },
	script              = "trigger.lua",
	objectName			= "trigger.dae",
    category            = "EFFECT",
    footprintX			= 1,
	footprintZ			= 1,
	mass				= 50,
    maxDamage           = 10000,
    collisionVolumeScales   = '0 0 0',
    collisionVolumeType     = 'cylY',
    pushResistant       = true,
    blocking            = false,
    canMove             = false, --effects cannot be moved (even by gravity)
    canGuard            = false,
    canPatrol           = false,
    canRepeat           = false,
    stealth				= true,
	turnRate			= 0,
	upright				= true,
    sightDistance		= 0,
--     canCloak            = true,
--     initCloaked         = true,
--     decloakOnFire       = false,
--     minCloakDistance    = 0,
}

local MushroomSpawn = BaseEffect:New {
    name                = "MushroomSpawn",
}

local Base = BaseEffect:New {
    name                = "Base",
}
local Expansion1 = BaseEffect:New {
    name                = "Expansion1",
}
local Expansion2 = BaseEffect:New {
    name                = "Expansion2",
}
local Expansion3 = BaseEffect:New {
    name                = "Expansion3",
}

return lowerkeys({
    MushroomSpawn       = MushroomSpawn,
	Base				= Base,
	Expansion1			= Expansion1,
	Expansion2			= Expansion2,
	Expansion3			= Expansion3,
})
