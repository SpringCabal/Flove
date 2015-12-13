local SporeSprayBase = Weapon:New{
	name                    = "Spore Spray Base",
	areaOfEffect            = 128,
	avoidFeature            = false,
	avoidFriendly           = false,
	avoidNeutral            = false,
	avoidGround             = false,

	collideFriendly         = false,

	beamTime                = 5,
	beamTtl                 = 3,
	coreThickness           = 0.2,
	craterBoost             = 0,
	craterMult              = 0,
	impulseFactor			= 0,

	damage                  = {
		default = 2,
	},
	explosionGenerator      = [[custom:genericshellexplosion-small-firey-modified]],
	interceptedByShieldType = 1,
	largeBeamLaser          = true,
	laserFlareSize          = 0.1,
	minIntensity            = 1,
	noSelfDamage            = true,
	range                   = 100,
	reloadtime              = 10,
	rgbColor                = "0.6 0.1 0.1",
	scrollSpeed             = 10,
	soundTrigger            = true,
	sweepfire               = false,
	thickness               = 20,
	tileLength              = 10000,
	tolerance               = 5000,
	turret                  = true,
	weaponType              = "BeamLaser",
	weaponVelocity          = 100,
	pulseSpeed				= 0.1,
-- 	soundstart  			= "firemono.wav",
}

local NormalMushroomSpray = SporeSprayBase:New {
	name                    = "NormalMushroomSpray",
}

return lowerkeys{
	NormalMushroomSpray = NormalMushroomSpray,
}