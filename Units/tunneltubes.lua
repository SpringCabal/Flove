local TunnelTube = Unit:New {
    footprintX			    = 0,
	footprintZ			    = 0,
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
    yardMap                 = "o",

    name                = "TunnelTube",
    script              = "flower.lua",

	 customParams        = {
		modelradius        = [[10]],
		midposoffset       = [[0 0 0]],

		shrubs 			   = true,
    },

    footprintX			    = 1,
	footprintZ			    = 1,
	-- Hitbox
	collisionVolumeOffsets = "0 0 0",
	collisionVolumeScales  = "0 0 0",
	collisionVolumeType    = "cylY",

}

local tunneltube1 = TunnelTube:New {
	name                = "TunnelTube1",
}

local tunneltube2 = TunnelTube:New {
	name                = "TunnelTube2",
}

local tunneltube3 = TunnelTube:New {
	name                = "TunnelTube3",
}

return lowerkeys({
	tunneltube1 = tunneltube1,
	tunneltube2 = tunneltube2,
	tunneltube3 = tunneltube3,
})
