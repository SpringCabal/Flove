Skeleton Spring game.
Libraries are included as files (not git submodules) so just pull/push as usual.

To use this for a new game clone it in a separate repository. Do *not* add game-specific modifications. This should be generic and usable for future Spring games as well (think of it is "Z" in SpringABC).

This repository contains:
- SpringABC (using gajop's chili instead)
- analytics
- lups
- modelshaders
- S44 style OO defs
- and a bunch of other useful custom widgets and gadgets

Check out the wiki for detailed information.


[![Join the chat at https://gitter.im/SpringCabal/LD34](https://badges.gitter.im/SpringCabal/LD34.svg)](https://gitter.im/SpringCabal/LD34?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Mushroom AI shares an indexed tree/spire target snapshot per tick. Actor and
wisp rosters use stable ID order and rebuild only after creation or destruction.
Tree configuration is cached at creation. Grass spacing uses the correct two
coordinates. `skip_tutorial=1` enters free play at frame four; normal launches
keep the interactive tutorial.
