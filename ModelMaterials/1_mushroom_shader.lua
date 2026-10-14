-- $Id$
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
local glUniform=gl.Uniform

local healthFactorID, uniformShader

local function DrawUnit(unitID, material)
	if material.shader == nil then
		-- The shader failed to compile; without it there are no uniform
		-- locations to set, and returning true here would leave the unit
		-- undrawn.  Let the engine draw it plainly instead of raising a
		-- Lua error on every unit, every frame.
		return false
	end
	if uniformShader ~= material.shader then
		uniformShader = material.shader
		healthFactorID = gl.GetUniformLocation(material.shader, "healthFactor")
	end
	local hp, maxhp = Spring.GetUnitHealth(unitID)
	local healthFactor = hp / maxhp
	glUniform(healthFactorID, healthFactor)

  --// engine should still draw it (we just set the uniforms for the shader)
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
}


--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

return materials, unitMaterials

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
