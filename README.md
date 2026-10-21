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
