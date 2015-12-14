local SporeSprayBase = Weapon:New{
  areaOfEffect       = 2,
  beamWeapon         = true,
  craterMult         = 0,
  duration 			 = 1,
  fireStarter        = 290,
  id                 = 138,
  impulseFactor      = 0,
  largeBeamLaser     = true,
  lineOfSight        = true,
  minIntensity       = 1,
  name               = "shroombarf",
  noSelfDamage       = true,
  range              = 600,
  reloadtime         = 3,
  renderType         = 0,
  rgbColor           = "1 1 0.4",
  rgbColor2          = "0.9 0.9 0.4",  
  texture1           = "flare",
  texture2           = "flare",
  thickness          = 7,
  --corethickness      = 2,
  tolerance          = 500,
  turret             = true,
  weaponTimer        = 2,
  weaponVelocity     = 1200,
  soundstart         = "sporedispersal.wav",
	customparams = {
		effect1			= "greymuzzle",
		damagetype		= "explosive",	
		--explosion		= "sparklered",
		animation		= "headers/weapons/shroombarf.lua",
	},	
  damage = {
    default            = 30,
  },
  collideFriendly = false,
  collideNeutral = false,
}

local NormalMushroomSpray = SporeSprayBase:New {
	name                    = "NormalMushroomSpray",
	damage = {
		default            = 30,
	},
}
local SmallMushroomSpray = SporeSprayBase:New {
	name                    = "SmallMushroomSpray",
	damage = {
		default            = 30,
	},
}
local BigMushroomSpray = SporeSprayBase:New {
	name                    = "BigMushroomSpray",
	reloadtime         		= 8,
	thickness          		= 20,
	beamTime 				= 3,
	damage = {
		default            = 300,
	},
}
local MushroomClusterSpray = SporeSprayBase:New {
	name                    = "MushroomClusterSpray",
	damage = {
		default            = 45,
	},
}
local PoisonMushroomSpray = SporeSprayBase:New {
	name                    = "PoisonMushroomSpray",
	damage = {
		default            = 60,
	},
}

return lowerkeys{
	NormalMushroomSpray = NormalMushroomSpray,
	SmallMushroomSpray = SmallMushroomSpray,
	BigMushroomSpray = BigMushroomSpray,
	MushroomClusterSpray = MushroomClusterSpray,
	PoisonMushroomSpray = PoisonMushroomSpray,
}