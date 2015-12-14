--------------------------------------------------------------------------------

local weaponDef = {
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
  customparams = {
	damagetype		= "beam",  
  },  
  damage = {
    default            = 30,
  },
  collideFriendly = false,
  collideNeutral = false,
}
--------------------------------------------------------------------------------

return lowerkeys({["shroombarf"] = weaponDef})

--------------------------------------------------------------------------------
