#!/bin/bash
set -euxo pipefail
cargo build --release --bin fixbn5
target/release/fixbn5 data/*/ROCK_EXE5_TOC_Dark.sav ../tango/tango/src/game/bn5/save/dark_colonel_jp.raw
target/release/fixbn5 data/*/ROCK_EXE5_TOC_Light.sav ../tango/tango/src/game/bn5/save/light_colonel_jp.raw
target/release/fixbn5 data/*/ROCK_EXE5_TOB_Dark.sav ../tango/tango/src/game/bn5/save/dark_protoman_jp.raw
target/release/fixbn5 data/*/ROCK_EXE5_TOB_Light.sav ../tango/tango/src/game/bn5/save/light_protoman_jp.raw
target/release/fixbn5 data/*/ROCK_EXE5_TOC_Dark.sav ../tango/tango/src/game/bn5/save/dark_colonel_us.raw --us
target/release/fixbn5 data/*/ROCK_EXE5_TOC_Light.sav ../tango/tango/src/game/bn5/save/light_colonel_us.raw --us
target/release/fixbn5 data/*/ROCK_EXE5_TOB_Dark.sav ../tango/tango/src/game/bn5/save/dark_protoman_us.raw --us
target/release/fixbn5 data/*/ROCK_EXE5_TOB_Light.sav ../tango/tango/src/game/bn5/save/light_protoman_us.raw --us
