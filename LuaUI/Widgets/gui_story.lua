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
    CreateGUI()
end

function NewMissionObjective(objective)
    mission_objective_text:SetText(blue .. objective)
    --Spring.PlaySoundStream('sounds/missions/NewObjective.wav') 
    if mission_objective.hidden then
        mission_objective:Show()
    end
end

local MissionName = [[OPERATION MUSHROOM]]
local MissionObjective = [[
In times long gone, the trees, flowers and grasses lived peacefully in the secluded mountain groves. But war raged in neighbouring kingdoms, displacing hundreds of ordinary mushrooms from their broken homes. 

Repel borders! Push back the oncoming mushrooms, or they'll take the soil and sunlight from our native fauna! Also, beware of suicide bombing mushrooms.
]]

function CreateGUI()

    window = Chili.Panel:New{
        parent = Chili.Screen0,
        right  = 450+50,
        y      = 0,
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
        text = blue .. (MissionObjective or "insert mushroom"),  
        font = {
            outline          = true,
            autoOutlineColor = true,
            outlineWidth     = 3,
            outlineWeight    = 8,
            size             = 15,        
        }
    }
    
    local function ShowHide()
        if mission_objective.hidden then
            showhide_button:SetCaption(grey .. "hide")
            mission_objective:Show()
            window:Invalidate()
        else
            showhide_button:SetCaption(grey .. "show")
            mission_objective:Hide()
            window:Invalidate()
        end
    
    end

    showhide_button = Chili.Button:New{
        parent = mission_name_window,
        y = 0,
        right = 0,
        height = 33,
        width = 70,
        caption = grey .. "hide",
        onclick = {ShowHide},
    }
    
end








