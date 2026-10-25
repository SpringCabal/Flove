#!/usr/bin/env bash
# Use the framework's existing staging, build, launch and strict log helpers.
set -euo pipefail
fixtures=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
project_root=${SPRINGCABAL_DEV_ROOT:-$(cd "$fixtures/../../../.." && pwd)}
cd "$project_root"
source verification/lib/common.sh
exec {lock}>>"${VF_LOCK_FILE:-/tmp/springcabal-verify.lock}"
flock "$lock"
for variant in lua rust; do
    if [[ $variant == rust ]]; then
        game="$GAMES_DIR/Flove-wasm.sdd"
        vf_build "$game/wasm-src" animation-trace
    else
        game="$GAMES_DIR/Flove.sdd"
    fi
    vf_workspace "flove-firing-$variant" stable
    staged=$(vf_stage_game "$game")
    # Keep the script dispatcher and model offsets, remove normal game logic.
    find "$staged/LuaRules/Gadgets" -maxdepth 1 -type f -name '*.lua' \
        ! -name unit_script.lua ! -name unit_centeroffset.lua -delete
    if [[ $variant == rust ]]; then
        vf_install_modules "$staged" flove_rules_wasm:LuaRules/wasm/flove-rules.wasm \
            flove_ui_wasm:LuaUI/wasm/flove-ui.wasm
    fi
    cp "$fixtures/mushroom-firing.lua" "$staged/LuaRules/Gadgets/zz_firing_probe.lua"
    vf_link_map "$MAPS_DIR/LD34-map.sdd" 'The Grove 0.3.sdd'
    launch=lua
    [[ $variant != rust ]] || launch=wasm
    vf_start_file "$(vf_launch_script flove "$launch")"
    vf_start_set FixedRNGSeed 1
    vf_modoption gallery anim
    vf_speed 20
    vf_config
    vf_graphics flove
    vf_run_visual 45
    vf_save_logs
    vf_require_clean_exit
    vf_forbid_runtime_failures
    vf_require 'FLOVE MUSHROOM FIRING PASSED'
    printf '%s: all six mushroom weapons fired and hit their targets\n' "$variant"
done
