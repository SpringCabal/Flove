local SporeSprayBase = Weapon:New{
  areaOfEffect       = 2,
  beamWeapon         = true,
  craterMult         = 0,
  cegTag             = [[barftrail]],
  duration 			 = 0,
  explosionGenerator = [[custom:barfsplurge]],
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
  thickness          = 0,
  --corethickness      = 2,
  tolerance          = 500,
  turret             = true,
  weaponTimer        = 2,
  weaponVelocity     = 1200,
  soundstart         = "sporedispersal.wav",
	customparams = {
		effect1			= "greymuzzle",
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
		default            = 10,
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

local KingMushroomSpray = SporeSprayBase:New {
	name                    = "KingMushroomSpray",
	reloadtime         		= 8,
	thickness          		= 20,
	beamTime 				= 3,
	customparams = {
		animation		= "headers/weapons/kingshroombarf.lua",
	},	
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

local poison = SporeSprayBase:New {
	name                    = "poison",
	damage = {
		default            = 1,
	},
	explosionGenerator = [[custom:barfgas]],
	areaOfEffect       = 320,
	soundstart         = "",
	burnBlow = true,
	weaponType = "Cannon",
}

return lowerkeys{
	poison = poison,
	NormalMushroomSpray = NormalMushroomSpray,
	SmallMushroomSpray = SmallMushroomSpray,
	BigMushroomSpray = BigMushroomSpray,
	KingMushroomSpray = KingMushroomSpray,
	MushroomClusterSpray = MushroomClusterSpray,
	PoisonMushroomSpray = PoisonMushroomSpray,
}
