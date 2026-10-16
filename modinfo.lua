local modinfo = {
	name = "Flove WASM",
	shortname = "FloveW",
	game = "Flove WASM",
	shortgame = "FloveW",
	version = "$VERSION",
	description = "Flove",
	modtype = "1",
	-- Single-player only: without this the engine binds a UDP socket and warns
	-- that nobody else can reach it. Nothing here is ever hosted or joined.
	onlyLocal = true,
	depend = {
		"cursors.sdz",
	}
}

return modinfo
