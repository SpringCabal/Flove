# Flove WASM

The Rust/WASM port of the game.

Use the [SpringCabal development repository](https://github.com/gajop/springcabal-dev) for engine setup, launching, builds and verification.

## Scoped SDK port

This worktree uses `spring-sdk` (named `spring` in Cargo) and the shared
`spring-addons` handler. Helpers take the current `&Spring`; queued commands,
Rml events and render-to-texture callbacks receive a fresh reference. Unit
scripts use the shared CUS host and ask `UnitCtx` for engine access during each
operation, so sleeping tasks do not retain a callin reference.

This remains a Wasm game. Native game binaries are not built by this port.
The outline shader, tree deformation and mouse-over highlight are retained.
Grass respects the engine's configured `GrassDetail`, including zero to disable
it. The game no longer overrides this with 2000 at startup.

From the development repository root, run
`spring-linux/games/Flove-wasm.sdd/verification/mushroom-firing.sh` to check
all six shooting mushroom types in both Lua and Rust. It stages private game
copies and checks actual projectiles and damage; bomb mushrooms are kamikazes.

Verification on the SDK test engine:

- Linux and Wine: all 84 existing integration assertions pass (66 rules,
  10 UI input, 8 Lua scenario), with no engine errors or warnings.
- Lua/Rust animation comparison: 95 skeleton entries, 19,760 poses and five
  events agree within the existing tolerance (largest pose difference 0.000008).
- Strict Clippy passes with all features, including the integration and
  animation features. The six shared Rust unit tests pass.
- All nine visual scenario captures complete. The SDK port exposed a bug in
  its counted graphics arrays: scalar/vec2/vec3 uniforms were rejected. That
  is fixed in the SDK, and outlines are visible again. Compared with the
  pre-port code on the same renderer, image differences are 0–2.08%.
- After removing the forced grass detail, the nine visual goldens were reviewed
  and refreshed using the current GPU renderer in the development repository.

Run verification through the parent development repository's `just verify
flove` commands, selecting an independently built engine via
`SPRING_ENGINE_DIR`. Local engine builds here are only test builds.

Wisp drawing gathers the existing tree rules parameters once per draw frame
and reuses stage vertex buffers across the three stage and reflection passes.
Creation and destruction invalidate that frame's snapshot. Mushroom AI reads
command counts directly and reuses weighted-target scratch storage; its target
ordering, random draws and attack behavior are unchanged.

Release builds use whole-program LTO and one code generation unit. The
workspace profile applies to both installed modules and verification builds.

## Optional Rust jobs

The default `rust_jobs=off` keeps the serial path. Set the game modoption
`rust_jobs=on` to calculate mushroom target distances and weights in same-tick jobs.
`rust_jobs=check` also compares worker output with inline computation; it is
for verification, not timing. RNG, engine calls and result application keep
their original order. A dedicated worker module has an 8 MiB fixed memory.

For measurements, use the springcabal-dev performance runner with
`--rust-jobs off|on|check --worker-threads 1|4`; compare jobs at the same
engine pool width.

The Lua/Rust startup uses the same frame-zero markers and frame-three groves
and tunnels, including placement jitter, ownership and tree wisp RNG order.
Growth shuffles only live trees in both versions. Decorations use terrain
height, and weighted growth calculations retain the engine's float32 ordering.
The visual upgrade scenario waits for three real projectile completions and
checks promotion at the chosen tree; its deadline remains bounded.
