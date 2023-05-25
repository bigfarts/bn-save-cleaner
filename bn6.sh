#!/bin/bash
set -euxo pipefail
cargo build --release --bin fixbn6
target/release/fixbn6 data/*/ROCK_EXE6_GXX.sav ../tango/tango/src/game/bn6/save/g_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_Blues.sav ../tango/tango/src/game/bn6/save/protoman_g_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_HeatMan.sav ../tango/tango/src/game/bn6/save/heatman_g_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_ElecMan.sav ../tango/tango/src/game/bn6/save/elecman_g_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_SlashMan.sav ../tango/tango/src/game/bn6/save/slashman_g_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_KillerMan.sav ../tango/tango/src/game/bn6/save/eraseman_g_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_ChargeMan.sav ../tango/tango/src/game/bn6/save/chargeman_g_jp.raw
target/release/fixbn6 data/*/ROCK_EXE6_RXX.sav ../tango/tango/src/game/bn6/save/f_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_Blues.sav ../tango/tango/src/game/bn6/save/protoman_f_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_AquaMan.sav ../tango/tango/src/game/bn6/save/spoutman_f_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_TomahawkMan.sav ../tango/tango/src/game/bn6/save/tomahawkman_f_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_TenguMan.sav ../tango/tango/src/game/bn6/save/tenguman_f_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_GroundMan.sav ../tango/tango/src/game/bn6/save/groundman_f_jp.raw
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_DustMan.sav ../tango/tango/src/game/bn6/save/dustman_f_jp.raw
target/release/fixbn6 data/*/ROCK_EXE6_GXX.sav ../tango/tango/src/game/bn6/save/g_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_Blues.sav ../tango/tango/src/game/bn6/save/protoman_g_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_HeatMan.sav ../tango/tango/src/game/bn6/save/heatman_g_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_ElecMan.sav ../tango/tango/src/game/bn6/save/elecman_g_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_SlashMan.sav ../tango/tango/src/game/bn6/save/slashman_g_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_KillerMan.sav ../tango/tango/src/game/bn6/save/eraseman_g_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_GXX_ChargeMan.sav ../tango/tango/src/game/bn6/save/chargeman_g_us.raw --us
target/release/fixbn6 data/*/ROCK_EXE6_RXX.sav ../tango/tango/src/game/bn6/save/f_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_Blues.sav ../tango/tango/src/game/bn6/save/protoman_f_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_AquaMan.sav ../tango/tango/src/game/bn6/save/spoutman_f_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_TomahawkMan.sav ../tango/tango/src/game/bn6/save/tomahawkman_f_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_TenguMan.sav ../tango/tango/src/game/bn6/save/tenguman_f_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_GroundMan.sav ../tango/tango/src/game/bn6/save/groundman_f_us.raw --us
target/release/fixbn6 data/*/*/ROCK_EXE6_RXX_DustMan.sav ../tango/tango/src/game/bn6/save/dustman_f_us.raw --us
