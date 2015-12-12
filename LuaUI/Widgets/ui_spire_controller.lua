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
local lastx, lasty, lastz

-- function widget:GameFrame(n)
-- 	local mx, my = Spring.GetMouseState()
-- 	local _, pos = Spring.TraceScreenRay(mx, my, true)
-- 	if pos then
-- 		local x, y, z = pos[1], pos[2], pos[3]
-- 		if x ~= lastx or y ~= lasty or z ~= lastz then
-- 			lastx, lasty, lastz = x, y, z
-- 			Spring.SendLuaRulesMsg('movegun|' .. x .. '|' .. y .. '|' .. z )
-- 		end
-- 	end
-- end

function widget:MousePress(mx, my, button)
	local alt, ctrl, meta, shift = Spring.GetModKeyState()
	if not Spring.IsAboveMiniMap(mx, my) then
		local traceType, pos = Spring.TraceScreenRay(mx, my, true)
		if pos then
			if button == 1 then
				local x, y, z = pos[1], pos[2], pos[3]
				Spring.SendLuaRulesMsg('zap|' .. x .. '|' .. y .. '|' .. z )
				return true
			else
				local x, y, z = pos[1], pos[2], pos[3]
				Spring.SendLuaRulesMsg('field_of_flowers|' .. x .. '|' .. y .. '|' .. z )
				return true
			end
		end
	end	
end