-- $Id$
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
local GetGameFrame = Spring.GetGameFrame
local GetUnitHealth = Spring.GetUnitHealth
local modulo = math.fmod
local glUniform = gl.Uniform
local sine = math.sin
local maximum = math.max

local function DrawUnit(unitID, material)
	if not material or not material.shader then
		return false
	end
	local shader = material.shader
	local frameLocID = gl.GetUniformLocation(shader, "frameLoc")
	local healthFactorID = gl.GetUniformLocation(shader, "healthFactor")
	local grownPercentageID = gl.GetUniformLocation(shader, "grownPercentage")
	local heightID = gl.GetUniformLocation(shader, "height")
	local colorFactorID = gl.GetUniformLocation(shader, "colorFactor")

	local unitDefID = Spring.GetUnitDefID(unitID)
	local unitDef = unitDefID and UnitDefs[unitDefID]
	if not unitDef then
		return false
	end

	local factor = 0.0001
	if unitDef.customParams and unitDef.customParams.tree then
		factor = 0.002
	elseif unitDef.customParams and unitDef.customParams.shrubs then
		factor = 0.008
	end
	local gameFrame = GetGameFrame()
	local frame = factor * sine(modulo(unitID, 10) + gameFrame / (modulo(unitID, 7) + 6))
	
	local hp, maxhp = GetUnitHealth(unitID)
	local isDead = Spring.GetUnitIsDead(unitID)
	local healthFactor = 0.0
	if hp and maxhp and maxhp > 0 and not isDead and hp > 0 then
		healthFactor = math.max(0.0, math.min(1.0, hp / maxhp))
	end

	local createdFrame = Spring.GetUnitRulesParam(unitID, "createdFrame") or 0
	local growTime = 33 * 0.5
	local duration = gameFrame - createdFrame
	local grownPercentage = math.min(1, duration / growTime)
	if not unitDef.customParams or not unitDef.customParams.tree then
		grownPercentage = 1
	end

	local height = Spring.GetUnitHeight(unitID) or 40

	local colorFactor = 0
	if unitDef.customParams and unitDef.customParams.tree then
		local progress = Spring.GetUnitRulesParam(unitID, "upgradeProgress") or 0
		if progress > 0 and not isDead then
			colorFactor = (sine(gameFrame / 3 * (progress - 0.5)) / math.pi + 1) / 2.5
		end
	end

	if frameLocID then glUniform(frameLocID, frame) end
	if healthFactorID then glUniform(healthFactorID, healthFactor) end
	if grownPercentageID then glUniform(grownPercentageID, grownPercentage) end
	if heightID then glUniform(heightID, height) end
	if colorFactorID then glUniform(colorFactorID, colorFactor) end

	return false
end

local materials = {
   tree = {
      shader    = include("ModelMaterials/Shaders/treeshader.lua"),
      force     = true, --// always use the shader even when normalmapping is disabled
      usecamera = false,
      culling   = GL.BACK,
      texunits  = {
        [0] = '%%UNITDEFID:0',
        [1] = '%%UNITDEFID:1',
        [2] = '$shadow',
        [3] = '$specular',
        [4] = '$reflection',
      },
      DrawUnit = DrawUnit
   }
}

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
-- affected unitdefs

local unitMaterials = {
   treelevel1 = "tree",
   treelevel2 = "tree",
   treelevel3 = "tree",
   spire = "tree",
}

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

return materials, unitMaterials

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
