#!/bin/bash
set -euxo pipefail
cargo build --release --bin fixbn6
target/release/fixbn6 data/ロックマンエグゼ６電脳獣グレイガ/ROCK_EXE6_GXX.sav ../tango/tango/src/game/bn6/save/g_jp.raw
target/release/fixbn6 data/ロックマンエグゼ６電脳獣グレイガ/ROCK_EXE6_GXX.sav ../tango/tango/src/game/bn6/save/g_us.raw --us
target/release/fixbn6 data/ロックマンエグゼ６電脳獣ファルザー/ROCK_EXE6_RXX.sav ../tango/tango/src/game/bn6/save/f_jp.raw
target/release/fixbn6 data/ロックマンエグゼ６電脳獣ファルザー/ROCK_EXE6_RXX.sav ../tango/tango/src/game/bn6/save/f_us.raw --us
