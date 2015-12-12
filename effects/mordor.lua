return { 
 ["mordor"] = {
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
  },
  
   ["mordor_flowershot"] = {
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
        length             = 10.5,
        width              = 10,
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
        colormap           = [[0 0.95 1 1   0 0.95 1 1   0 0.95 1 0.9   0 0.95 1 0.5   0 0.95 1 0.3   1 0.5 0 0.05   0.25 0.1 0 0    0 0 0 0]], 
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
