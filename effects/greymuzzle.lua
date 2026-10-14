-- Mushroom firing effect, matching the definition in the WASM archive.
return {
    greymuzzle = {
        particles = {
            air = true,
            class = [[CSimpleParticleSystem]],
            count = 1,
            ground = true,
            properties = {
                airdrag = 0.8,
                alwaysvisible = true,
                colormap = [[0.6 0.9 0.1 0.8  0.3 0.6 0.0 0.0]],
                directional = true,
                emitrot = 0,
                emitrotspread = 30,
                emitvector = [[0, 0, 1]],
                gravity = [[0, 0, 0]],
                numparticles = 4,
                particlelife = 10,
                particlelifespread = 5,
                particlesize = 4,
                particlesizespread = 2,
                particlespeed = 5,
                particlespeedspread = 3,
                pos = [[0, 0, 0]],
                sizegrowth = 1,
                sizemod = 1.0,
                texture = [[flare]],
            },
        },
    },
}
