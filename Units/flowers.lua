local Flower = Unit:New {
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

    name                = "Flower",
    script              = "tree.lua",

	 customParams        = {
		modelradius        = [[10]],
		midposoffset       = [[0 0 0]],

		terrainblock_x     = 0,
		terrainblock_z     = 0,
    },

    footprintX			    = 1,
	footprintZ			    = 1,
	-- Hitbox
	collisionVolumeOffsets = "0 0 0",
	collisionVolumeScales  = "0 0 0",
	collisionVolumeType    = "cylY",

}

local flower1 = Flower:New {
	name                = "Flower1",
}

local flower2 = Flower:New {
	name                = "Flower2",
}

local flower3 = Flower:New {
	name                = "Flower3",
}

local flower4 = Flower:New {
	name                = "Flower4",
}

local flower5 = Flower:New {
	name                = "Flower5",
}

local flower6 = Flower:New {
	name                = "Flower6",
}

local grass1 = Flower:New {
	name                = "Grass1",
}

local grass2 = Flower:New {
	name                = "Grass2",
}

local grass3 = Flower:New {
	name                = "Grass3",
}

local grass4 = Flower:New {
	name                = "Grass4",
}

local grass5 = Flower:New {
	name                = "Grass5",
}

return lowerkeys({
	flower1 = flower1,
	flower2 = flower2,
	flower3 = flower3,
	flower4 = flower4,
	flower5 = flower5,
	flower6 = flower6,
	grass1 = grass1,
	grass2 = grass2,
	grass3 = grass3,
	grass4 = grass4,
	grass5 = grass5,
})
