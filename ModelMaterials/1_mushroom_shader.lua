-- $Id$
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
local glUniform = gl.Uniform

local function DrawUnit(unitID, material)
	if not material or not material.shader then
		return false
	end
	local shader = material.shader
	local healthFactorID = gl.GetUniformLocation(shader, "healthFactor")

	local hp, maxhp = Spring.GetUnitHealth(unitID)
	local isDead = Spring.GetUnitIsDead(unitID)
	local healthFactor = 0.0
	if hp and maxhp and maxhp > 0 and not isDead and hp > 0 then
		healthFactor = math.max(0.0, math.min(1.0, hp / maxhp))
	end

	if healthFactorID then
		glUniform(healthFactorID, healthFactor)
	end

	return false
end

local materials = {
   mushroom = {
      shader    = include("ModelMaterials/Shaders/mushroomshader.lua"),
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
    normalmushroom       = "mushroom",
	bigmushroom          = "mushroom",
	smallmushroom        = "mushroom",
	mushroomcluster      = "mushroom",
	poisonmushroom       = "mushroom",
	bombmushroom         = "mushroom",
	kingshroom           = "mushroom",
}

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

return materials, unitMaterials

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
