function gadget:GetInfo()
	local modOptions = Spring.GetModOptions() or {}
	local isTest = (modOptions.test_mode == "1" or modOptions.test == "1")
	return {
		name      = "Flove Test Scenario",
		desc      = "Automated test scenario for Flove (disabled in normal releases)",
		author    = "Antigravity",
		date      = "2026",
		license   = "GPL v2",
		layer     = 1000,
		enabled   = isTest
	}
end

if not gadgetHandler:IsSyncedCode() then
	-- Screenshots, taken on these frames after the test start. Video-capturing
	-- mode makes the engine simulate exactly one frame per drawn frame, with
	-- interpolation pinned at 0, so each capture shows exactly its frame, and
	-- the scripted messages reach the rules on the same frames every run.
	local SHOTS = {
		{frame = 10, name = "1 (initial grove)"},
		{frame = 30, name = "2 (spire zap in flight)"},
		{frame = 60, name = "3 (mushroom elimination)"},
		{frame = 80, name = "4 (tree damage shader)"},
		{frame = 122, name = "5 (RMB golden upgrade projectile in flight)"},
		{frame = 145, name = "5b (tree promoted to Level 2)"},
		{frame = 172, name = "6 (RMB flower shot projectile in flight)"},
		{frame = 195, name = "6b (field of flowers bloomed)"},
		{frame = 225, name = "7 (king shroom boss)"},
	}
	local unpaused = false
	local nextShot = 1
	function gadget:Update()
		if not unpaused then
			Spring.SendCommands("forcestart")
			Spring.SendCommands("pause 0")
			Spring.SendCommands("setminspeed 1.0")
			Spring.SendCommands("setmaxspeed 1.0")
			unpaused = true
		end
		-- The UI pauses the game while a story is open (here the king's), and
		-- nobody presses Continue: resume it. While the Rust suites run, they
		-- handle the stories themselves.
		if Spring.GetGameRulesParam("flove_rust_suite_running") ~= 1 then
			local _, _, paused = Spring.GetGameSpeed()
			if paused then
				Spring.SendCommands("pause 0")
			end
		end
	end

	-- Only drawing runs call this, so headless runs keep real-time pacing.
	function gadget:DrawScreen()
		local shot = SHOTS[nextShot]
		if not shot then
			return
		end
		if not Spring.GetVideoCapturingMode() then
			Spring.SetVideoCapturingMode(true)
		end
		local frame = Spring.GetGameFrame()
		local startF = Spring.GetGameRulesParam("test_start_frame")
		if not startF then
			return
		end
		local f = frame - startF
		if f >= shot.frame then
			nextShot = nextShot + 1
			Spring.Echo("[Flove-Test] Taking screenshot " .. shot.name .. " at gameframe=" .. tostring(frame) .. " rel=" .. tostring(f))
			Spring.SendCommands("screenshot png")
			if not SHOTS[nextShot] then
				Spring.SetVideoCapturingMode(false)
			end
		end
	end
	return
end

-- SYNCED CODE
local startFrame = nil
local testTreeID = nil
local testMushID = nil
local bossID = nil
local passCount = 0
local failCount = 0
-- Every Assert below passes exactly once.
local EXPECTED_CHECKS = 8

local function Assert(cond, desc)
	if not cond then
		failCount = failCount + 1
		Spring.Echo("[Flove-Test] ASSERTION FAILED: " .. tostring(desc))
	else
		passCount = passCount + 1
		Spring.Echo("[Flove-Test] ASSERTION PASSED (" .. passCount .. "): " .. tostring(desc))
	end
end

local function CountUnits(matches, x, z, radius)
	local units = x and Spring.GetUnitsInCylinder(x, z, radius) or Spring.GetAllUnits()
	local count = 0
	for _, uid in ipairs(units) do
		if not Spring.GetUnitIsDead(uid) and matches(UnitDefs[Spring.GetUnitDefID(uid)]) then
			count = count + 1
		end
	end
	return count
end

local function IsMushroom(ud)
	return ud.customParams and ud.customParams.mushroom
end

function gadget:Initialize()
	Spring.Echo("[Flove-Test] Initializing Flove automated test scenario...")
	Spring.SetGameRulesParam("test_mode", 1)
end

function gadget:GameFrame(frame)
	if not startFrame then
		-- The Rust integration suite (headless builds) runs first and ends by
		-- restarting the round; this scenario plays the round after it.
		-- The rules module starts on its first callin, its GameFrame on
		-- frame 0, which runs after this one: its flag is readable from
		-- frame 1.
		if frame < 1 or Spring.GetGameRulesParam("flove_rust_suite_running") == 1 then
			return
		end
		startFrame = frame
		Spring.SetGameRulesParam("test_start_frame", startFrame)
		Spring.Echo("[Flove-Test] Test sequence starting at frame=" .. tostring(startFrame))
		Spring.SendLuaRulesMsg("skip_tutorial")
	end

	local f = frame - startFrame

	-- Step 1 (f = 5): Locate and select a tree and mushroom
	if f == 5 then
		for _, uid in ipairs(Spring.GetAllUnits()) do
			local ud = UnitDefs[Spring.GetUnitDefID(uid)]
			if ud and ud.customParams and ud.customParams.tree then
				local ux, uy, uz = Spring.GetUnitPosition(uid)
				Spring.Echo("[Flove-Test] Tree id=" .. tostring(uid) .. " pos=(" .. math.floor(ux) .. ", " .. math.floor(uy) .. ", " .. math.floor(uz) .. ")")
				if not testTreeID then
					testTreeID = uid
				end
			elseif ud and ud.customParams and ud.customParams.mushroom and not testMushID then
				testMushID = uid
			end
		end
		Assert(testTreeID ~= nil, "Found foreground tree (id=" .. tostring(testTreeID) .. ")")
		-- Mushrooms arrive with the wave schedule, so this early in the run there
		-- is nothing to find; the six "mushroomspawn" points are spawners, not
		-- mushrooms. Spawn one to attack with, the same way step 7 spawns the boss.
		if not testMushID and testTreeID then
			local tx, ty, tz = Spring.GetUnitPosition(testTreeID)
			local mushDef = UnitDefNames.normalmushroom
			if mushDef then
				testMushID = Spring.CreateUnit(mushDef.id, tx + 300, ty, tz + 300, 0, 1)
			end
		end
		Assert(testMushID ~= nil, "Found mushroom attacker (id=" .. tostring(testMushID) .. ")")
		if testMushID and testTreeID then
			local tx, ty, tz = Spring.GetUnitPosition(testTreeID)
			Spring.GiveOrderToUnit(testMushID, CMD.ATTACK, {tx, ty, tz}, {})
			Spring.Echo("[Flove-Test] Ordered mushroom to attack foreground tree id=" .. tostring(testTreeID))
		end
	end

	-- Step 2 (f = 25): Fire Spire Zap at the mushroom attacker
	if f == 25 then
		Spring.Echo("[Flove-Test] Step 2: Firing Spire Zap ability from summit apex...")
		if testMushID and Spring.ValidUnitID(testMushID) then
			local mx, my, mz = Spring.GetUnitPosition(testMushID)
			Spring.SendLuaRulesMsg("zap|" .. math.floor(mx) .. "|" .. math.floor(my) .. "|" .. math.floor(mz))
			Spring.Echo("[Flove-Test] Dispatched Spire Zap attack towards (" .. math.floor(mx) .. ", " .. math.floor(my) .. ", " .. math.floor(mz) .. ")")
		end
	end

	-- The zap is on its way a few frames later.
	if f == 28 then
		local zapDefID = WeaponDefNames.zap.id
		local zaps = 0
		for _, proID in ipairs(Spring.GetProjectilesInRectangle(0, 0, Game.mapSizeX, Game.mapSizeZ)) do
			if Spring.GetProjectileDefID(proID) == zapDefID then
				zaps = zaps + 1
			end
		end
		Assert(zaps > 0, "The spire's zap is in flight (" .. zaps .. " zap projectiles)")
	end

	-- Step 3 (f = 50): Eliminate all mushrooms to clear the foreground completely
	if f == 50 then
		Spring.Echo("[Flove-Test] Step 3: Eliminating all mushroom attackers...")
		for _, uid in ipairs(Spring.GetAllUnits()) do
			local ud = UnitDefs[Spring.GetUnitDefID(uid)]
			if ud and ud.customParams and ud.customParams.mushroom then
				Spring.DestroyUnit(uid, true, false)
			end
		end
	end
	if f == 52 then
		local left = CountUnits(IsMushroom)
		Assert(left == 0, "All mushrooms cleared from the field (" .. left .. " left)")
	end

	-- Step 4 (f = 75): Apply 20% damage to grove trees to test health shader/growth
	if f == 75 then
		Spring.Echo("[Flove-Test] Step 4: Applying 20% health damage to grove trees...")
		local count = 0
		for _, uid in ipairs(Spring.GetAllUnits()) do
			local ud = UnitDefs[Spring.GetUnitDefID(uid)]
			if ud and ud.customParams and ud.customParams.tree then
				local _, maxHp = Spring.GetUnitHealth(uid)
				Spring.SetUnitHealth(uid, maxHp * 0.2)
				count = count + 1
			end
		end
		Assert(count > 0, "Set " .. count .. " grove trees to 20% health for damage shader")
	end

	-- Step 5 (f = 90, 110, 130): Send 3 upgrade shots to fully promote tree to Level 2
	if f == 90 or f == 110 or f == 130 then
		Spring.Echo("[Flove-Test] Step 5 (f=" .. tostring(f) .. "): Dispatching upgrade shot...")
		Spring.SendLuaRulesMsg("add_mana|100")
		local targetTree = nil
		local minD = 999999
		for _, uid in ipairs(Spring.GetAllUnits()) do
			local ud = UnitDefs[Spring.GetUnitDefID(uid)]
			if ud and ud.customParams and ud.customParams.tree then
				local ux, uy, uz = Spring.GetUnitPosition(uid)
				local d = (ux - 4688)^2 + (uz - 5630)^2
				if d < minD then
					minD = d
					targetTree = uid
				end
			end
		end
		if targetTree and Spring.ValidUnitID(targetTree) then
			local ux, uy, uz = Spring.GetUnitPosition(targetTree)
			Spring.SendLuaRulesMsg("upgrade|" .. tostring(targetTree) .. "|" .. math.floor(ux) .. "|" .. math.floor(uy) .. "|" .. math.floor(uz))
		end
	end

	-- Step 6 (f = 160): Cast Field of Flowers in open empty foreground (fires FlowerShot at f=160)
	if f == 160 then
		Spring.Echo("[Flove-Test] Step 6: Granting mana and casting Field of Flowers in empty foreground...")
		Spring.SendLuaRulesMsg("add_mana|100")
		Spring.SendLuaRulesMsg("field_of_flowers|4688|1370|5650")
	end

	-- Three upgrade shots have landed on the tree.
	if f == 145 then
		local level2 = CountUnits(function(ud) return ud.name == "treelevel2" end)
		Assert(level2 > 0, "Upgrade shots promoted a tree to level 2 (" .. level2 .. " level-2 trees)")
	end

	-- The flower shot has landed and its field bloomed.
	if f == 195 then
		local flowers = CountUnits(function(ud) return ud.name:find("^flower%d") ~= nil end, 4688, 5650, 350)
		Assert(flowers >= 20, "The field of flowers bloomed (" .. flowers .. " flowers)")
	end

	-- Step 7 (f = 220): Spawn King Shroom Boss in foreground
	if f == 220 then
		Spring.Echo("[Flove-Test] Step 7: Spawning King Shroom boss in foreground...")
		local kingDefID = UnitDefNames.kingshroom.id
		bossID = Spring.CreateUnit(kingDefID, 4688, 1370, 5620, 0, 1)
		Assert(bossID ~= nil and Spring.ValidUnitID(bossID), "Spawned King Shroom Boss (id=" .. tostring(bossID) .. ")")
	end

	-- Step 8 (f = 250): Verification complete
	if f == 250 then
		Spring.Echo("[Flove-Test] =========================================")
		if failCount == 0 and passCount >= EXPECTED_CHECKS then
			Spring.Echo("[Flove-Test] ALL FLOVE EXTENDED TEST ASSERTIONS PASSED (" .. passCount .. " checks)!")
		elseif failCount == 0 then
			Spring.Echo("[Flove-Test] TEST SUITE FAILED: only " .. passCount .. " of " .. EXPECTED_CHECKS .. " checks ran")
		else
			Spring.Echo("[Flove-Test] TEST SUITE HAD " .. failCount .. " FAILURES (passed: " .. passCount .. ")")
		end
		Spring.Echo("[Flove-Test] =========================================")
		Spring.SendCommands("quitforce")
	end
end
