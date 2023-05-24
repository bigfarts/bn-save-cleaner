#!/bin/bash
set -euxo pipefail
cargo build --release --bin fixexe45
target/release/fixexe45 data/*/ROCK_EXE4_5.sav ../tango/tango/src/game/exe45/save/any.raw
