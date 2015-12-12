local SpireBase = Unit:New {
    footprintX			    = 2,
	footprintZ			    = 2,
	mass				    = 10e5,
    pushResistant           = true,
    blocking                = false,
    canMove                 = false,
    canGuard                = false,
    canPatrol               = false,
    canRepeat               = false,
    stealth				    = true,
	turnRate			    = 0,
	upright				    = true,
    sightDistance		    = 0,
    yardMap                 = "oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo oooooooo",

    name                = "Spire base",
    script              = "spire.lua",

	 customParams        = {
		modelradius        = [[30]],
		midposoffset       = [[0 50 0]],

		terrainblock_x     = 64,
		terrainblock_z     = 64,
    },

    footprintX			    = 6,
	footprintZ			    = 6,
	-- Hitbox
	collisionVolumeOffsets = "0 0 0",
	collisionVolumeScales  = "70 76 70",
	collisionVolumeType    = "cylY",

}

local Spire = SpireBase:New {
	name                = "Spire",
	maxDamage			= 3,
}

return lowerkeys({
    Spire = Spire,
})
