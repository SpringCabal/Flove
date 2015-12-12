--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
--

function gadget:GetInfo()
  return {
    name      = "wisp_animation",
    desc      = "Wisp animation shader gadget",
    author    = "gajop, trepan, jK", -- based on trepan/jk's perticipation gadget
    date      = "2015 December",
    license   = "GNU GPL, v2 or later",
    layer     = 10,
    enabled   = true
  }
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

-- Unsynced side only
if (gadgetHandler:IsSyncedCode()) then
  return false
end

-- Require shaders
if (not gl.CreateShader) or (not gl.PointParameter) then
  return false
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local enabled = true

local shader
local shaderWindLoc
local shaderTimeLoc
local shaderCamPosLoc
local shaderCamDirLoc
local shaderStageLoc
local shaderNeedLocs = true

local rainList
local particleList
local useDrawList = false

local DENSITY     = 10
local SCALE       = 100
local SPEED       = 0.1
local TEXTURE     = 'LuaRules/Gadgets/snowflake.tga'
local WIND_SCALE  = 1.2

assert(type(DENSITY) == "number")
assert(type(SCALE) == "number")
assert(type(SPEED) == "number")
assert(type(WIND_SCALE) == "number")
assert(type(TEXTURE) == "string")

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
local trees ={}

function gadget:Initialize()
  if (not ReloadResources()) then
	  Spring.Echo("failed to reload resources")
    gadgetHandler:RemoveGadget()
    return
  end
Spring.Echo("loaded resources")
  enabled = true

  gadgetHandler:AddChatAction("rain", ChatAction,
    ':  [0|1]  [speed val]  [scale val]  [density val]  [texture name]'
  )
end


function gadget:Shutdown()
  FreeResources()
  gadgetHandler:RemoveChatAction("rain")
end


function ReloadResources()
  FreeResources()
  if ((not CreateParticleList()) or
      (not CreateRainList())     or
      (not CreateShader()))    then
    gadgetHandler:RemoveGadget()
    return false
  end
  return true
end


function FreeResources()
  gl.DeleteList(rainList)
  gl.DeleteList(particleList)
  if (gl.DeleteShader) then
    gl.DeleteShader(shader)
  end
  shader = nil
  rainList = nil
  particleList = nil
end


function ChatAction(cmd, line, words, player)
  --Spring.Echo(cmd,' : ',line ' : ',words, ':',player)
  if (player ~= 0) then
    Spring.Echo('Only the host can control the weather')
    return true
  end
  if (#words == 0) then
    enabled = not enabled
    Spring.Echo("rain is " .. (enabled and 'enabled' or 'disabled'))
    return
  end
  if (words[1] == '0') then
    enabled = false
    Spring.Echo("rain is disabled")
  elseif (words[1] == '1') then
    enabled = true
    Spring.Echo("rain is enabled")
  elseif (words[1] == 'scale') then
    local value = tonumber(words[2])
    if (value and (value > 0)) then
      SCALE = value
      ReloadResources()
    end
  elseif (words[1] == 'speed') then
    local value = tonumber(words[2])
    if (value) then
      SPEED = value
      ReloadResources()
    end
  elseif (words[1] == 'density') then
    local value = tonumber(words[2])
    if (value and (value > 0) and (value <= 1000000)) then
      DENSITY = value
      ReloadResources()
    end
  elseif (words[1] == 'texture') then
    if (type(words[2]) == 'string') then
      TEXTURE = words[2]
      ReloadResources()
    end
  end
  return true
end


function DrawParticles(stage)
    gl.BeginEnd(GL.POINTS, function()
		for _, unitID in ipairs(Spring.GetAllUnits()) do
			local ux, uy, uz = Spring.GetUnitPosition(unitID)
-- 			Spring.Echo(unitID)
			for i = 1, 1000 do
				local wispStage = Spring.GetUnitRulesParam(unitID, "wispStage" .. tostring(i))
				local wispOffset = Spring.GetUnitRulesParam(unitID, "wispOffset" .. tostring(i))
				if wispStage == nil then
					break
				end
				if wispStage == stage then
					local x = ux/4096
					local y = Spring.GetGroundHeight(ux,uz)/4096
					local z = uz/4096
					local w =  0.05 + wispOffset
					gl.Vertex(x, y, z, w)
				end
			end
		end
    end)
end

function CreateParticleList()
   particleList = gl.CreateList(function()
	DrawParticles()
  end)

  if (particleList == nil) then
    return false
  end
  return true
end

function DrawRain(stage)
	gl.Color(0, 0, 1, 1)

    gl.PointSprite(true, true)
    gl.PointSize(1.0)
    gl.PointParameter(0, 0, .001, 0, 1e9, 1)

    gl.DepthTest(true)
    --gl.Blending(GL.SRC_ALPHA, GL.ONE)
    gl.Texture(TEXTURE)

	if useDrawList then
		gl.CallList(particleList)
	else
		DrawParticles(stage)
	end

    gl.Texture(false)
    gl.Blending(GL.SRC_ALPHA, GL.ONE_MINUS_SRC_ALPHA)
    gl.DepthTest(false)

    gl.PointParameter(1, 0, 0, 0, 1e9, 1)
    gl.PointSize(10.0)
    gl.PointSprite(false, false)
end

function CreateRainList()
  rainList = gl.CreateList(function()
    DrawRain()
  end)

  if (rainList == nil) then
    return false
  end
  return true
end


function CreateShader()
  shaderNeedLocs = true

  shader = gl.CreateShader({
    uniform = {
      time   = 0,
      scale  = SCALE,
      speed  = SPEED,
      camPos = { 0, 0, 0 },
    },
    vertex = [[
      uniform float time;
      uniform float scale;
      uniform float speed;
      uniform vec3 camDir;
      uniform vec3 camPos;
      uniform vec3 wind;
	  uniform float stage;
	  varying float glimmer;

      void main(void)
      {
        const float boxSize = 4096.;
        float hBoxSize = boxSize * 0.25;

        vec3 scalePos = vec3(gl_Vertex) * boxSize;
        vec3 eye = camPos;

        vec3 pos = scalePos ;
        pos.x += sin(time*gl_Vertex.w + scalePos.y+gl_Vertex.w*20) * 240. * gl_Vertex.w;
        pos.z += cos(time*(1-gl_Vertex.w) + scalePos.x+gl_Vertex.w*20) * 240. * gl_Vertex.w;
		pos.y += (1+sin(time*(gl_Vertex.w*0.1) + scalePos.y+gl_Vertex.w*20)) * 300. * gl_Vertex.w + 300;
		float origPosY = pos.y;
		glimmer = max(0, sin(time*(gl_Vertex.w*5))/2);

        vec4 eyePos = gl_ModelViewMatrix * vec4(pos, 1.0);

        gl_PointSize = (0.1 + gl_Vertex.w) * scale * hBoxSize / length(eyePos.xyz) * stage;
		if (stage == 1) {
			gl_FrontColor.rgb = vec3(1, 0.7, 0);
		} else if (stage == 2) {
			gl_FrontColor.rgb = vec3(0.4, 0.4, 1);
		} else if (stage == 3) {
			gl_FrontColor.rgb = vec3(1, 1, 1);
		}
        gl_FrontColor.a   = gl_Color.a * (1. - (camPos.y - origPosY) / boxSize);
        gl_Position = gl_ProjectionMatrix * eyePos;
      }
    ]],
    fragment = [[
      uniform sampler2D tex0;
	  varying float glimmer;

      void main(void)
      {
        gl_FragColor = gl_Color * texture2D(tex0, gl_TexCoord[0].st);
        gl_FragColor.a = gl_FragColor.a-glimmer;
      }
    ]],
    uniformInt = {
      tex0 = 0
    },
  })

  if (shader == nil) then
    print(gl.GetShaderLog())
    return false
  end
  return true
end


--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function GetShaderLocations()
  shaderTimeLoc   = gl.GetUniformLocation(shader, 'time')
  shaderCamPosLoc = gl.GetUniformLocation(shader, 'camPos')
  shaderCamDirLoc = gl.GetUniformLocation(shader, 'camDir')
  shaderWindLoc   = gl.GetUniformLocation(shader, 'wind')
  shaderStageLoc   = gl.GetUniformLocation(shader, 'stage')
end


--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local inWaterReflectionPass = false
local function spGetCameraDirection()
	local x,y,z = Spring.GetCameraDirection()
	if inWaterReflectionPass then
		return x, -y, z
	end
	return x, y, z
end

local function smoothstep(min,max,v)
	if (v<=min) then return 0.0; end
	if (v>=max) then return 1.0; end
	local t = (v - min) / (max - min);
	t = math.min(1.0, math.max(0.0, t ));
	return t * t * (3.0 - 2.0 * t);
end

local function blend(x,y,a)
	return x * (1-a) + y * a
end


--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local next_upd = 0
local oldWindX, oldWindY, oldWindZ = 0,0,0
local windX, windY, windZ = 0,0,0
local oldWindOffset = 0

function DrawEffects(stage)
	if (not enabled) then
    return
  end

  --local GL_DEPTH_WRITEMASK = 2930
  --Spring.Echo("GL_DEPTHWRITE", gl.GetNumber(GL_DEPTH_WRITEMASK, 1))

  gl.UseShader(shader)

  if (shaderNeedLocs) then
    GetShaderLocations()
    shaderNeedLocs = false
  end

  local gameFrame = Spring.GetGameFrame()

  if (next_upd <= gameFrame) then
    oldWindX = windX
    oldWindY = windY
    oldWindZ = windZ
    wx, wy, wz = Spring.GetWind()
    windX = windX + wx * WIND_SCALE
    windY = windY + wy * WIND_SCALE
    windZ = windZ + wz * WIND_SCALE
    next_upd = gameFrame + 210
    oldWindOffset = 0
  end

  local timeOffset = Spring.GetFrameTimeOffset() / 30
  gl.Uniform(shaderTimeLoc,   Spring.GetGameSeconds() + timeOffset)
  gl.Uniform(shaderCamPosLoc, Spring.GetCameraPosition())
  gl.Uniform(shaderCamDirLoc, spGetCameraDirection())
  gl.Uniform(shaderStageLoc, stage)

  windOffset = 1 - (next_upd - (gameFrame + timeOffset * 30)) / 210
  if (windOffset < oldWindOffset) then
    windOffset = oldWindOffset
  end
  oldWindOffset = windOffset
  windOffset = smoothstep(0, 1, windOffset)
  gl.Uniform(shaderWindLoc, blend(oldWindX, windX, windOffset), blend(oldWindY, windY, windOffset), blend(oldWindZ, windZ, windOffset))

  gl.MatrixMode(GL.PROJECTION); gl.PushMatrix(); gl.LoadMatrix("camprj")
  gl.MatrixMode(GL.MODELVIEW);  gl.PushMatrix(); gl.LoadMatrix("camera")

  if useDrawList then
	gl.CallList(rainList)
  else
	DrawRain(stage)
  end

  gl.MatrixMode(GL.PROJECTION); gl.PopMatrix()
  gl.MatrixMode(GL.MODELVIEW);  gl.PopMatrix()

  gl.UseShader(0)
end

function gadget:DrawScreenEffects() --World()
	DrawEffects(1)
	DrawEffects(2)
	DrawEffects(3)
end


function gadget:DrawWorldReflection()
	local camY = select(2, Spring.GetCameraPosition())
	if (camY < 350) and (camY > 0) then
		inWaterReflectionPass = true
			gadget:DrawScreenEffects()
		inWaterReflectionPass = false
	end
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
