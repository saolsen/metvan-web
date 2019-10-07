#!/bin/bash

# @TODO: Make this whole script fail if any parts of it fail. I forget how to do that.
# @TODO: Probably don't want export all since we wanna pass structs and floats and stuff.

CFLAGS="-Wall -Os -g -fdiagnostics-absolute-paths"

echo "Compiling"
mkdir -p dist
pushd dist
clang $CFLAGS --target=wasm32 -nostdlib -Wl,--no-entry -Wl,--export-all -Wl,--allow-undefined -Wl,--import-memory -o metvan.wasm ../src/web_platform.c
cp ../src/web_platform_index.html index.html
cp ../src/web_platform.js web_platform.js
popd
echo "Done"