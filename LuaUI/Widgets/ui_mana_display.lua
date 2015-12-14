-- WIP
function widget:GetInfo()
	return {
		name    = 'Mana Display',
		desc    = '',
		author  = 'Bluestone',
		date    = 'rawr',
		license = 'Horses',
        layer = 0,
		enabled = true,
	}
end

local Chili, Screen0
local textBox
local manaColour = "\255\255\0\99"
local nMana = 0

function resizeUI(vsx,vsy)
    if quitButton ~= nil then
        quitButton:SetPos(vsx*0.52, vsy*(1-0.05-0.01), vsx*0.05, vsy*0.05) 
    end
end

function widget:ViewResize(vsx,vsy)
	resizeUI(vsx,vsy)
end

function widget:Initialize()
	Chili = WG.Chili
    Screen0 = Chili.Screen0
    
    textBox = Chili.TextBox:New{
        parent = Screen0,
        x = "45%",
        bottom = 20,
        width = 200,
        text = "Mana: " .. manaColour .. "0",        
        font = {
            size = 20,
        },
    }
        
    local vsx,vsy = Spring.GetViewGeometry()
    resizeUI(vsx,vsy)
end

function widget:GameFrame()
    local nManaNew = Spring.GetGameRulesParam("mana")
    if nMana ~= nManaNew then
        textBox:SetText("Mana: " .. manaColour .. nManaNew)
        nMana = nManaNew
    end
end
