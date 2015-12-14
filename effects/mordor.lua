function deepcopy(t)
    if type(t) ~= 'table' then return t end
    local mt = getmetatable(t)
    local res = {}
    for k,v in pairs(t) do
        if type(v) == 'table' then
            v = deepcopy(v)
        end
        res[k] = v
    end
    setmetatable(res,mt)
    return res
end

local mordor = {
	pillar = {
		air                = true,
		class              = [[explspike]],
		count              = 7,
		ground             = true,
		water              = true,
		properties = {
			alpha              = 1,
			alphadecay         = 0.1,
			alwaysvisible      = true,
			color              = [[1, 0.5, 0.1]],
			dir                = [[0,10.0]],
			length             = 2.5,
			width              = 2,
		},
	},
	circle = {
		air                = true,
		class              = [[CSimpleParticleSystem]],
		count              = 1,
		ground             = true,
		water              = true, 
		underwater         = true,
		properties = {
			airdrag            = 0.87,
			alwaysvisible      = true,
			colormap           = [[1 0.5 0 0.05	0 0 0 0.0]], -- same as groundflash colors
			directional        = false,
			emitrot            = 90,
			emitrotspread      = 5,
			emitvector         = [[0, 0, 0]],
			gravity            = [[0, 0, 0]],
			numparticles       = 1,
			particlelife       = [[20]], -- same as groundflash ttl
			particlelifespread = 0,
			particlesize       = 6, -- groundflash flashsize 25 = 1, so if flashsize is 200, particlesize here would be 8
			particlesizespread = 1,
			particlespeed      = [[8]],
			particlespeedspread = 6,
			pos                = [[0, 1, 0]],
			sizegrowth         = 4, -- same as groundflash circlegrowth
			sizemod            = 0.8,
			texture            = [[explosionwave]],
		},
	}
}
mordor.pillar.properties.color = [[0.015 0.79 0.094]]
mordor.pillar.properties.width = 2
mordor.circle.properties.sizemod = 0.8
mordor.circle.properties.colormap = [[0.015 0.79 0.094 0.05 0 0 0 0.0]]

local mordor2 = deepcopy(mordor)
mordor2.pillar.properties.color = [[0.015 0.79 0.094]]
mordor2.pillar.properties.width = 4
mordor2.circle.properties.sizemod = 0.9
mordor2.circle.properties.colormap = [[0.35 0.76 0.054 0.05 0 0 0 0.0]]

local mordor3 = deepcopy(mordor)
mordor3.pillar.properties.width = 4
mordor3.circle.properties.sizemod = 1
mordor3.circle.properties.colormap = [[0.65 0.74 0.024 0.05 0 0 0 0.0]]

local mordor4 = deepcopy(mordor)
mordor4.pillar.properties.width = 4
mordor4.circle.properties.sizemod = 1.025
mordor4.circle.properties.colormap = [[1 0.737 0.019 0.05 0 0 0 0.0]]

local mordor5 = deepcopy(mordor)
mordor5.pillar.properties.width = 4
mordor5.circle.properties.sizemod = 1.05
mordor5.circle.properties.colormap = [[1 0.6 0.03 0.05 0 0 0 0.0]]

local mordor6 = deepcopy(mordor)
mordor6.pillar.properties.width = 4
mordor6.circle.properties.sizemod = 1.075
mordor6.circle.properties.colormap = [[1 0.4 0.05 0.05 0 0 0 0.0]]

local mordor7 = deepcopy(mordor)
mordor7.pillar.properties.width = 4
mordor7.circle.properties.sizemod = 1.1
mordor7.circle.properties.colormap = [[1 0.3 0.07 0.05 0 0 0 0.0]]

local mordor8 = deepcopy(mordor)
mordor8.pillar.properties.width = 4
mordor8.circle.properties.sizemod = 1.125
mordor8.circle.properties.colormap = [[1 0.188 0.098 0.05 0 0 0 0.0]]

return { 
   ["mordor1"] = mordor,
   ["mordor2"] = mordor2,
   ["mordor3"] = mordor3,
   ["mordor4"] = mordor4,
   ["mordor5"] = mordor5,
   ["mordor6"] = mordor6,
   ["mordor7"] = mordor7,
   ["mordor8"] = mordor8,
   ["mordor_flowershot"] = {
    pillar = {
      air                = true,
      class              = [[explspike]],
      count              = 4,
      ground             = true,
      water              = true,
      properties = {
        alpha              = 1,
        alphadecay         = 0.1,
        alwaysvisible      = true,
        dir                = [[0,10.0]],
        length             = 15,
        width              = 15,
      },
    },
    circle = {
      air                = true,
      class              = [[CSimpleParticleSystem]],
      count              = 1,
      ground             = true,
      water              = true, 
	  underwater         = true,
      properties = {
        airdrag            = 0.87,
        alwaysvisible      = true,
        colormap           = [[0 0.95 1 0.02  0 0.95 1 0.001  0 0 0 0]], 
        directional        = false,
        emitrot            = 90,
        emitrotspread      = 5,
        emitvector         = [[0, 0, 0]],
        gravity            = [[0, 0, 0]],
        numparticles       = 1,
        particlelife       = [[30]], 
        particlelifespread = 0,
        particlesize       = 12, 
        particlesizespread = 1,
        particlespeed      = [[8]],
        particlespeedspread = 6,
        pos                = [[0, 1, 0]],
        sizegrowth         = 1.5, 
        sizemod            = 1,
        texture            = [[explosionwave]],
      },
    }
  },
  
  
   ["mordor_zap"] = {
    pillar = {
      air                = true,
      class              = [[explspike]],
      count              = 4,
      ground             = true,
      water              = true,
      properties = {
        alpha              = 1,
        alphadecay         = 0.1,
        alwaysvisible      = true,
        dir                = [[0,10.0]],
        length             = 15,
        width              = 15,
      },
    },
    circle = {
      air                = true,
      class              = [[CSimpleParticleSystem]],
      count              = 1,
      ground             = true,
      water              = true, 
	  underwater         = true,
      properties = {
        airdrag            = 0.87,
        alwaysvisible      = true,
        colormap           = [[0.5 0 0 0.05  0 0 0 0]], 
        directional        = false,
        emitrot            = 90,
        emitrotspread      = 5,
        emitvector         = [[0, 0, 0]],
        gravity            = [[0, 0, 0]],
        numparticles       = 1,
        particlelife       = [[30]], 
        particlelifespread = 0,
        particlesize       = 12, 
        particlesizespread = 1,
        particlespeed      = [[8]],
        particlespeedspread = 6,
        pos                = [[0, 1, 0]],
        sizegrowth         = 1.5, 
        sizemod            = 1,
        texture            = [[explosionwave]],
      },
    }
  },

  
}
