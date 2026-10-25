-- Both runtimes use the same isolated engine scenario. Bomb mushrooms are
-- kamikazes rather than shooters, so the six projectile weapons are tested.
function gadget:GetInfo() return {name='Mushroom firing regression',layer=100000,enabled=true} end
if not gadgetHandler:IsSyncedCode() then return false end
local names={'normalmushroom','smallmushroom','bigmushroom','mushroomcluster','poisonmushroom','kingshroom'}
local actor,target,shots,hits,current= nil,nil,0,0,0
function gadget:Initialize()
 for _,name in ipairs(names) do
  for _,slot in ipairs(UnitDefs[UnitDefNames[name].id].weapons) do Script.SetWatchWeapon(slot.weaponDef,true) end
 end
end
function gadget:ProjectileCreated(_,owner) if owner==actor then shots=shots+1 end end
function gadget:UnitDamaged(unit,_,_,damage,_,_,_,attacker)
 if unit==target and attacker==actor then hits=hits+1 end
end
local function state(frame)
 local ready,reloaded,reload=Spring.GetUnitWeaponState(actor,1)
 local px,py,pz=Spring.GetUnitWeaponVectors(actor,1)
 local valid=Spring.GetUnitWeaponTestTarget(actor,1,target)
 local line=Spring.GetUnitWeaponHaveFreeLineOfFire(actor,1,target)
 Spring.Echo(string.format('[MushroomFire] frame=%d %s shots=%d hits=%d aim=%s reload=%s target=%s line=%s muzzle=(%.2f,%.2f,%.2f) ground=%.2f',frame,names[current],shots,hits,tostring(ready),tostring(reloaded),tostring(valid),tostring(line),px,py,pz,Spring.GetGroundHeight(px,pz)))
end
function gadget:GameFrame(frame)
 if frame<1 then return end
 local phase=(frame-1)%360
 if phase==0 then
  if actor then Spring.DestroyUnit(actor,false,true);Spring.DestroyUnit(target,false,true) end
  current=current+1
  if current>#names then Spring.Echo('FLOVE MUSHROOM FIRING PASSED');Spring.SendCommands('quitforce');return end
  shots,hits=0,0
  actor=assert(Spring.CreateUnit(names[current],5250,Spring.GetGroundHeight(5250,4500),4500,0,1))
  target=assert(Spring.CreateUnit('treelevel3',5250,Spring.GetGroundHeight(5250,4750),4750,0,0))
  Spring.SetUnitMaxHealth(target,100000);Spring.SetUnitHealth(target,100000)
  Spring.SetUnitNeutral(target,false);Spring.SetUnitAlwaysVisible(target,true)
  Spring.GiveOrderToUnit(actor,CMD.MOVE_STATE,{0},{})
  Spring.GiveOrderToUnit(actor,CMD.FIRE_STATE,{2},{})
  Spring.GiveOrderToUnit(actor,CMD.ATTACK,{target},{})
  local def=WeaponDefs[UnitDefs[Spring.GetUnitDefID(actor)].weapons[1].weaponDef]
  Spring.Echo('[MushroomFire] CREATE '..names[current]..' weapon='..def.type..' range='..def.range)
 elseif phase==359 then
  state(frame)
  assert(shots>0 and hits>0, names[current]..' never fired and hit its target')
 end
end
