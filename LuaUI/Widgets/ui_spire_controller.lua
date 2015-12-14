function widget:GetInfo()
	return {
		name 	= "Spire UI, also prevents use of mouse for anything else",
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

local function getMouseCoordinate(mx,my)
	local traceType, pos = Spring.TraceScreenRay(mx, my, true)
    if not pos then return false end
	local x, y, z = pos[1], pos[2], pos[3]
	if x<2048 or z<2048 or x>8192 or z>8192 then	 
		return false
	end
	return x,y,z
end

local function shootFlowers(mx,my)
	local x,y,z = getMouseCoordinate(mx,my);
	if (x) then
		if(x<2048 or z<2048 or x>8192 or z>8192) then return true end
		Spring.SendLuaRulesMsg('field_of_flowers|' .. x .. '|' .. y .. '|' .. z )
		return true
	else
		return false
	end
end

local function WeaponControl()
	local mx, my, lmb, mmb, rmb = Spring.GetMouseState()
	
	if lmb and mouseControl1 then
		local x,y,z = getMouseCoordinate(mx,my);
		if (x) then
			Spring.SendLuaRulesMsg('zap|' .. x .. '|' .. y .. '|' .. z )
			return true
		else
			return false
		end
	elseif rmb and mouseControl3 then
		local traceType, unitID = Spring.TraceScreenRay(mx, my)
		
		if (traceType == 'unit') then
			local ud = UnitDefs[Spring.GetUnitDefID(unitID)];
			if ud.customParams.tree then
				Spring.SendLuaRulesMsg('upgrade|' .. unitID )
			else
				shootFlowers(mx,my);
			end
		else
			shootFlowers(mx,my);
		end
	end
end

function widget:MousePress(mx, my, button)
	local alt, ctrl, meta, shift = Spring.GetModKeyState()
	if not Spring.IsAboveMiniMap(mx, my) then
        
		if button == 1 then
			local x,y,z = getMouseCoordinate(mx,my);
			if (x) then
				if(x<2048 or z<2048 or x>8192 or z>8192) then return true end
				Spring.SendLuaRulesMsg('zap|' .. x .. '|' .. y .. '|' .. z )
				mouseControl1 = true
				return true
			else
				return false
			end
		elseif button == 3 then
			local traceType, unitID = Spring.TraceScreenRay(mx, my)
			
			if (traceType == 'unit') then
				local ud = UnitDefs[Spring.GetUnitDefID(unitID)];
				if ud.customParams.tree and not (ud.name == 'treelevel3') then
					Spring.SendLuaRulesMsg('upgrade|' .. unitID )
					mouseControl3 = true
					return true
				else
					shootFlowers(mx,my);
					mouseControl3 = true
					return true
				end
			else
				shootFlowers(mx,my);
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
