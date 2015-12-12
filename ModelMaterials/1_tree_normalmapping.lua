-- $Id$
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
local GetGameFrame=Spring.GetGameFrame
local GetUnitHealth=Spring.GetUnitHealth
local modulo=math.fmod
local glUniform=gl.Uniform
local sine =math.sin
local maximum=math.max

local frameLocID
local function DrawUnit(unitID, material)
	if frameLocID == nil then
		frameLocID = gl.GetUniformLocation(material.shader, "frameLoc")
	end
	local frame = 0.001 * sine(modulo(unitID, 10) + GetGameFrame() / (modulo(unitID, 7) + 6))
	glUniform(frameLocID, frame)
--   health,maxhealth=GetUnitHealth(unitID)
--   glUniform(material.healthLoc, 2*maximum(0, (-2*health)/(maxhealth)+1) )
  
  --Spring.Echo('Drawing tree in 1_tree_normalmapping.lua!')
  --inverse of health, 0 if health is 100%-50%, goes to 1 by 0 health


  --// engine should still draw it (we just set the uniforms for the shader)
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
}


--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

return materials, unitMaterials

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
