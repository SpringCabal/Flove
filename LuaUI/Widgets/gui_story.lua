function widget:GetInfo()
  return {
    name      = "Story Display",
    desc      = "",
    author    = "J R R R Tolkien",
    date      = "",
    license   = "",
    layer     = 0,
    enabled   = true,
  }
end

local Chili

local turquoise = "\255\0\240\180"
local red = "\255\255\20\20"
local green = "\255\0\255\0"
local white = "\255\255\255\255"
local blue = "\255\170\170\255"
local grey = "\255\190\190\190"


function widget:Initialize()
    Chili = WG.Chili
	CreateStartWindow()
end

local MissionName = [[OPERATION MUSHROOM]]
local texts = {
[[In times long gone, the trees, flowers and grasses lived peacefully in the secluded mountain groves. 
But war raged in neighbouring kingdoms, displacing hundreds of ordinary mushrooms from their broken homes. ]],

-- Spawn mushrooms for players to defend
[[Repel borders! Push back the oncoming mushrooms, or they'll take the soil and sunlight from our native fauna!

]] .. "\255\0\128\128Hint: Left click on mushrooms to send love.\b",

-- Damage trees and have player heal them up
[[They reject our love and kindness, but the trees do not... 
Mend the wounds inflicted by the wretched fungi and ease their suffering!

]] .. "\255\0\128\128Hint: Left click on trees to heal them.\b",

-- Spawn a wave of enemies
[[Blasted, another wave of enemies approaches. Protect the forest, purify the invaders!]],

-- Make the player upgrade a tree
[[This seems to be the end of them for now. We must bolster our defences for what is to come.

Make our little garden stronger

]] .. "\255\0\128\128Hint: Right click on trees to upgrade them.\b",

-- Waves start coming
[[It's a beautiful sight indeed. But will it be enough for what is to come? Take good care of the fauna. Nurture it and it will grow.

Also, beware of suicide bombing mushrooms.

]] .. "\255\0\128\128Hint: Right click on mushrooms to create flower fields that slows them.\b",

-- Mushroom king appears...?

[[A vast enemy army approaches, defend the forest at all costs!]],

-- Mushroom king is defeated
[[The mushroom king lies defeated. The day is saved!]],
}

local narrations = {
	"sounds/narration_amplified/InTimesLongGone.ogg",
	"sounds/narration_amplified/RepelBorders.ogg",
	"sounds/narration_amplified/TheyRejectOurLoveAndKindness.ogg",
	"sounds/narration_amplified/BlastedAnoterWaveOfEnemies.ogg",
	"sounds/narration_amplified/ThisSeemsToBeTheEndOfThemForNow.ogg",
	"sounds/narration_amplified/ItsABeautifulSightIndeed.ogg",
	"sounds/narration_amplified/AVastEnemyArmyApproaches.ogg",
	"sounds/narration_amplified/TheMushroomKingLiesDefeated.ogg",
}

local currentText = 1
local currentIndex = 1

local closeTexts = {
	[2] = true,
	[3] = true,
	[4] = true,
	[5] = true,
	[6] = true,
	[7] = true,
	[8] = true,
}

function StartGame(difficulty)
	WG.ResetCamera()
	Spring.SendLuaRulesMsg('difficulty|' .. tostring(difficulty))
	CreateGUI()
	startWindow:Hide()
end

function NextClose()
	if not skip_button.hidden then
		skip_button:Hide()
	end
	local _, _, paused = Spring.GetGameSpeed()
	Spring.SendLuaRulesMsg('story')
	-- we should close it
	if closeTexts[currentText] then
		if paused then
			Spring.SendCommands("pause")
		end
		window:Hide()
	else
		if not paused then
			Spring.SendCommands("pause")
		end
	end
	currentIndex = 0
	currentText = currentText + 1
	UpdateText()
	if closeTexts[currentText] then
		next_button:SetCaption(grey .. "Continue")
	else
		next_button:SetCaption(grey .. "Next")
	end
end

function SkipTutorial()
	local _, _, paused = Spring.GetGameSpeed()
	if paused then
		Spring.SendCommands("pause")
	end
	Spring.SendLuaRulesMsg('skip_tutorial')
end

function CreateStartWindow()
	startWindow = Chili.Control:New {
        parent = Chili.Screen0,
        x = 0,
        y = 0,
        bottom = 0,
		right = 0,
        minHeight = 25,
		padding         = {0, 0, 0, 0},
		children = { 
			Chili.Button:New {
				y = 300,
				right = 400,
				height = 80,
				width = 300,
				caption = "Normal",
				OnClick = { function() StartGame(1) end },
			},
			Chili.Button:New {
				y = 400,
				right = 400,
				height = 80,
				width = 300,
				caption = "Hard",
				OnClick = { function() StartGame(2) end },
			},
			Chili.Button:New {
				y = 500,
				right = 400,
				height = 80,
				width = 300,
				caption = "Extreme",
				OnClick = { function() StartGame(3) end },
			},
			Chili.Button:New {
				y = 600,
				right = 400,
				height = 80,
				width = 300,
				caption = "Leave",
				OnClick = { function() Spring.SendCommands("quitforce") end },
			},
			Chili.Image:New {
				x = 0,
				y = 0,
				width = "100%",
				height = "100%",
				file = "Bitmaps/start_screen.png",
			},
		},
    }
end

function CreateGUI()

    window = Chili.Panel:New{
        parent = Chili.Screen0,
        right  = "35%",
        y      = "45%",
        width  = 525,
        minHeight = 25,
        autosize = true,
    }

    mission_name_window = Chili.Panel:New{
        parent = window,
        width = 525,
        autosize = false,
        height = 35,
        padding     = {0,0,0,0},
        itemPadding = {2,2,2,2},
        itemMargin  = {0,0,0,0},
    }   

    mission_name_text = Chili.TextBox:New{
        parent = mission_name_window,
        width = '100%',
        height = 30,
        y = 10,
        x = 10,
        text = white .. "Groves:  " .. green .. (MissionName or "test"),  
        font = {
            size = 16,
        }
    }

    mission_objective = Chili.LayoutPanel:New{
        parent = window,
        width = '100%',
        autosize = true,
        height = 100,
        y = 35,
        padding     = {5,5,5,5},
        itemPadding = {2,2,2,2},
        itemMargin  = {0,0,0,0},
    }   
    
    mission_objective_text = Chili.TextBox:New{
        parent = mission_objective,
        width = '100%',
        height = 1,
        text = "",
        font = {
            outline          = true,
            autoOutlineColor = true,
            outlineWidth     = 3,
            outlineWeight    = 8,
            size             = 15,
        }
    }

    next_button = Chili.Button:New{
        parent = mission_name_window,
        y = 0,
        right = 0,
        height = 33,
        width = 70,
        caption = grey .. "Next",
        onclick = {NextClose},
    }
	
	
    skip_button = Chili.Button:New{
        parent = mission_name_window,
        y = 0,
        right = 80,
        height = 33,
        width = 100,
        caption = grey .. "Skip Tutorial",
        onclick = {SkipTutorial},
	}
end

function widget:Update()
	if not window then
		return
	end
	
	local time = os.clock()
	if time < 5 then
		return
	end
	local story = Spring.GetGameRulesParam("story")
	local shroomEvent = Spring.GetGameRulesParam("shroomEvent") or 0
	if shroomEvent == 1 then
		story = 7
		if currentText < 7 then 
			currentText = 7
		end
		if not skip_button.hidden then
			skip_button:Hide()
		end
	elseif shroomEvent == 2 then
		story = 8
		if not skip_button.hidden then
			skip_button:Hide()
		end
	end
	
	local skip_tutorial = Spring.GetGameRulesParam("skip_tutorial") or 0
	if skip_tutorial == 1 and shroomEvent == 0 then
		if not window.hidden then
			window:Hide()
		end
		return
	end
	
	-- FIXME: minor race condition here; too tired to redesign this right
	if story == 1 and currentText > 2 then
		Spring.Echo("Reset story", story, currentText)
		WG.ResetCamera()
		-- reset everything
		currentText = 1
		currentIndex = 0
		if skip_button.hidden then
			skip_button:Show()
		end
		next_button:SetCaption(grey .. "Next")
	end
	
	if window.hidden and currentText == story then
		window:Show()
		local _, _, paused = Spring.GetGameSpeed()
		if not paused then
			Spring.SendCommands("pause")
		end
	end
	if math.floor(time * 1000) % 2 == 0 and not window.hidden then
		currentIndex = currentIndex + 1
		UpdateText()
	end
end

function UpdateText()
	local text = texts[currentText]
	if not text then
		return
	end
	if currentIndex == 2 then -- too sleepy, but this means the file is being displayed
		local narration = narrations[currentText]
		Spring.PlaySoundFile(narration)
	end
	local textPart = text:sub(1, currentIndex)
	mission_objective_text:SetText(blue .. textPart)
end





