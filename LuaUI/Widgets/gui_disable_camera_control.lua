--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function widget:GetInfo()
	return {
		name      = "Disable camera control",
		desc      = "Disables camera zooming and panning",
		author    = "gajop",
		date      = "WIP",
		license   = "GPLv2",
		version   = "0.1",
		layer     = -1000,
		enabled   = true,  --  loaded by default?
		handler   = true,
		api       = true,
		hidden    = true,
	}
end

local CONTROL_CAMERA = true

function widget:Initialize()
--     local devMode = (tonumber(Spring.GetModOptions().play_mode) or 0) == 0
    if not CONTROL_CAMERA then
        widgetHandler:RemoveWidget(widget)
        return
    end
	
	s = Spring.GetCameraState()
	for k, v in pairs(s) do
		print(k .. " = " .. v .. ",")
	end
	ResetCamera()
end

function ResetCamera()	
    s = {
		dist = 2384.8950195312,
		px = 4742.3002929688,
		py = 1380.3073730469,
		pz = 5785.755859375,
		rz = 0,
		dx = 0,
		dy = -0.72166359424591,
		dz = -0.69224387407303,
		fov = 45,
		ry = 0,
		mode = 2,
		rx = 2.3769989013672,
		name = "spring",
    }
    Spring.SetCameraState(s, 0)
end

function widget:Shutdown()
end

function widget:MouseWheel(up,value)
    -- uncomment this to disable zoom/panning
    --return true
end

WG.ResetCamera = ResetCamera