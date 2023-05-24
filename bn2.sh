#!/bin/bash
set -euxo pipefail
cargo build --release --bin fixbn2
target/release/fixbn2 data/*/ROCK_EXE2_Brother.sav ../tango/tango/src/game/bn2/save/team_any.raw
target/release/fixbn2 data/*/ROCK_EXE2_Custom.sav ../tango/tango/src/game/bn2/save/custom_any.raw
target/release/fixbn2 data/*/ROCK_EXE2_Guts.sav ../tango/tango/src/game/bn2/save/guts_any.raw
target/release/fixbn2 data/*/ROCK_EXE2_Shield.sav ../tango/tango/src/game/bn2/save/shield_any.raw
target/release/fixbn2 data/*/ROCK_EXE2_Normal_Saito.sav ../tango/tango/src/game/bn2/save/hub_any.raw
