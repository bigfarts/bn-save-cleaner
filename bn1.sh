#!/bin/bash
set -euxo pipefail
cargo build --release --bin fixbn1
target/release/fixbn1 data/*/ROCK_EXE1.sav ../tango/tango/src/game/bn1/save/jp.raw
target/release/fixbn1 data/*/ROCK_EXE1.sav ../tango/tango/src/game/bn1/save/us.raw --us
