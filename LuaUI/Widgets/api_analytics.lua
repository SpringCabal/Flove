function widget:GetInfo()
	return {
		name    = "Analytics API",
		desc    = "Placeholder: the analytics include is disabled, so this stays off.",
		author  = "SpringCabal",
		license = "GNU GPL, v2 or later",
		layer   = 0,
		enabled = false,
	}
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

ANALYTICS_DIRNAME = "libs/analytics/luaui/widgets/analytics/"
--VFS.Include(ANALYTICS_DIRNAME .. "init.lua", nil, VFS.DEF_MODE)
