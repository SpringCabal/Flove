function widget:GetInfo()
	return {
		name 	= "Spire UI",
		desc	= "Sends spire actions from LuaUI to LuaRules",
		author	= "gajop, Google Frog", -- based on GoogleFrog's shotgun UI
		date	= "12 December 2015",
		license	= "GNU GPL, v2 or later",
		layer	= 20, -- Placed after other click stealing widgets (eg structure placer)
		enabled = true
	}
end

-------------------------------------------------------------------
-------------------------------------------------------------------
local mouseControl1 = false
local mouseControl3 = false

local function WeaponControl()
	local mx, my, lmb, mmb, rmb = Spring.GetMouseState()
	local traceType, pos = Spring.TraceScreenRay(mx, my, true)
    if not pos then return false end
	local x, y, z = pos[1], pos[2], pos[3]
	if x<2048 or z<2048 or x>8192 or z>8192 then	 
		return false
	end
	if pos then
		if lmb and mouseControl1 then
			Spring.SendLuaRulesMsg('zap|' .. x .. '|' .. y .. '|' .. z )
			return true
		elseif rmb and mouseControl3 then
			Spring.SendLuaRulesMsg('field_of_flowers|' .. x .. '|' .. y .. '|' .. z )
			return true
		end
	end
end

function widget:MousePress(mx, my, button)
	local alt, ctrl, meta, shift = Spring.GetModKeyState()
	if not Spring.IsAboveMiniMap(mx, my) then
		local traceType, pos = Spring.TraceScreenRay(mx, my, true)
        if not pos then return false end
        local x, y, z = pos[1], pos[2], pos[3]

		if x<2048 or z<2048 or x>8192 or z>8192 then	 
            return false
        end

		if pos then
			if button == 1 then
				Spring.SendLuaRulesMsg('zap|' .. x .. '|' .. y .. '|' .. z )
				mouseControl1 = true
				return true
			elseif button == 3 then
				Spring.SendLuaRulesMsg('field_of_flowers|' .. x .. '|' .. y .. '|' .. z )
				mouseControl3 = true
				return true
			end
		end
	end	
end

function widget:GameFrame(n)
	if mouseControl1 or mouseControl3 then
		WeaponControl()
	end
	
	--UpdateCamera()
end

function widget:Initialize()
	if Game.gameName == "Scened LD34" then
		widgetHandler:RemoveWidget()
		return
	end
end

function widget:MouseRelease(mx, my, button)
	if button == 1 then
		mouseControl1 = false
	elseif button == 3 then
		mouseControl3 = false
	end
end