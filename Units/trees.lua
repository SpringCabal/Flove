local Tree = Unit:New {
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

    name                = "Tree",
    script              = "tree.lua",

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

local TreeLevel1 = Tree:New {
	name                = "Tree Level 1",
	maxDamage			= 3,
}
local TreeLevel2 = Tree:New {
	name                = "Tree Level 2",
	maxDamage			= 5,
}
local TreeLevel3 = Tree:New {
	name                = "Tree Level 3",
	maxDamage			= 7,
}

return lowerkeys({
    TreeLevel1       = TreeLevel1,
	TreeLevel2       = TreeLevel2,
	TreeLevel3       = TreeLevel3,
})