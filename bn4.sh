#!/bin/bash
set -euxo pipefail
cargo build --release --bin fixbn4
target/release/fixbn4 data/ロックマンエグゼ４トーナメントブルームーン/ROCK_EXE4_BM_HP1000_Light.sav ../tango/tango/src/game/bn4/save/light_hp_1000_bm_jp.raw
target/release/fixbn4 data/ロックマンエグゼ４トーナメントブルームーン/ROCK_EXE4_BM_HP999_Light.sav ../tango/tango/src/game/bn4/save/light_hp_999_bm_jp.raw
target/release/fixbn4 data/ロックマンエグゼ４トーナメントブルームーン/ROCK_EXE4_BM_HP997_Dark.sav ../tango/tango/src/game/bn4/save/dark_hp_997_bm_jp.raw
target/release/fixbn4 data/ロックマンエグゼ４トーナメントブルームーン/ROCK_EXE4_BM_HP1000_Light.sav ../tango/tango/src/game/bn4/save/light_hp_1000_bm_us.raw --us
target/release/fixbn4 data/ロックマンエグゼ４トーナメントブルームーン/ROCK_EXE4_BM_HP999_Light.sav ../tango/tango/src/game/bn4/save/light_hp_999_bm_us.raw --us
target/release/fixbn4 data/ロックマンエグゼ４トーナメントブルームーン/ROCK_EXE4_BM_HP997_Dark.sav ../tango/tango/src/game/bn4/save/dark_hp_997_bm_us.raw --us
target/release/fixbn4 data/ロックマンエグゼ４トーナメントレッドサン/ROCK_EXE4_RS_HP1000_Light.sav ../tango/tango/src/game/bn4/save/light_hp_1000_rs_jp.raw
target/release/fixbn4 data/ロックマンエグゼ４トーナメントレッドサン/ROCK_EXE4_RS_HP999_Light.sav ../tango/tango/src/game/bn4/save/light_hp_999_rs_jp.raw
target/release/fixbn4 data/ロックマンエグゼ４トーナメントレッドサン/ROCK_EXE4_RS_HP997_Dark.sav ../tango/tango/src/game/bn4/save/dark_hp_997_rs_jp.raw
target/release/fixbn4 data/ロックマンエグゼ４トーナメントレッドサン/ROCK_EXE4_RS_HP1000_Light.sav ../tango/tango/src/game/bn4/save/light_hp_1000_rs_us.raw --us
target/release/fixbn4 data/ロックマンエグゼ４トーナメントレッドサン/ROCK_EXE4_RS_HP999_Light.sav ../tango/tango/src/game/bn4/save/light_hp_999_rs_us.raw --us
target/release/fixbn4 data/ロックマンエグゼ４トーナメントレッドサン/ROCK_EXE4_RS_HP997_Dark.sav ../tango/tango/src/game/bn4/save/dark_hp_997_rs_us.raw --us
