local Stalk = piece('Stalk');
local Eye = piece('Eye');

local Leaves={};
local Branch={};

local function ShowBranch(n)
	local x,y,z = Spring.GetUnitPiecePosDir(unitID,Branch[n]);
	Spring.SpawnCEG("dirtfling", x, y, z, 0, 0, 0, 0);
	Show(Branch[n]);
	
	Sleep(500)
	
	x,y,z = Spring.GetUnitPiecePosDir(unitID,Leaves[n]);
	Spring.SpawnCEG("dirtfling", x, y, z, 0, 0, 0, 0);
	Show(Leaves[n]);
end

local function GlowEye()
	SetSignalMask(1);
	while true do
		x,y,z = Spring.GetUnitPiecePosDir(unitID,Eye);
		Spring.SpawnCEG("mordor", x, y, z, 0, 0, 0, 0);
		Sleep(300);
	end
end

function script.Create()
	for i=1,9 do
		Leaves[i] = piece('Leaves'..i);
		Branch[i] = piece('Branch'..i);
		Hide(Leaves[i]);
		Hide(Branch[i]);
	end
	
	StartThread(GlowEye);
	
	Sleep(1000);
	
	for i=1,9 do
		ShowBranch(i);
		Sleep(1000);
	end
end

function script.Killed(recentDamage, maxHealth)
	return 0;
end
