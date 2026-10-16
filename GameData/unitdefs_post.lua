local function RecursiveReplaceStrings(t, name, replacedMap)
	if (replacedMap[t]) then
		return  -- avoid recursion / repetition
	end
	replacedMap[t] = true
	for k, v in pairs(t) do
		if (type(v) == 'string') then
			t[k] = v:gsub("<NAME>", name)
		end
		if (type(v) == 'table') then
			RecursiveReplaceStrings(v, name, replacedMap)
		end
	end 
end

local function ReplaceStrings(t, name)
	local replacedMap = {}
	RecursiveReplaceStrings(t, name, replacedMap)
end

-- Process ALL the units!
for name, ud in pairs(UnitDefs) do
	-- Replace all occurences of <NAME> with the respective values
	ReplaceStrings(ud, ud.unitname or name)
	-- Ensure non-zero power to avoid division by zero in engine experience calculations
	if not ud.power or ud.power <= 0 then
		ud.power = ud.maxdamage or 100
	end
end

-- Clean non-engine fields from WeaponDefs
if WeaponDefs then
	for _, wd in pairs(WeaponDefs) do
		wd.new = nil
		wd.clone = nil
		wd.append = nil
		wd.id = nil
		wd.smart = nil
	end
end