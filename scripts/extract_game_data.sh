#!/bin/sh

CUR_DIR=$(pwd)
TARGET="$1"

mkdir -p "$TARGET"/keyvalues
cd aoe2dat/code || exit
mkdir build
cd build || exit
cmake ..
cmake --build .

./aoe2dat "$HOME"/.steam/steam/steamapps/common/AoE2DE/resources/_common/dat/empires2_x2_p1.dat
cd "$CUR_DIR" || exit
mv aoe2dat/code/build/full.json "$TARGET"
mv aoe2dat/code/build/units_buildings_techs.json "$TARGET"
cp "$HOME"/.steam/steam/steamapps/common/AoE2DE/widgetui/civTechTrees.json "$TARGET"