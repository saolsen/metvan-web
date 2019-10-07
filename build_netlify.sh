#!/bin/bash

echo "Downloading Clang"
wget --quiet http://releases.llvm.org/9.0.0/clang+llvm-9.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz
tar -xf clang+llvm-9.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz
export PATH="`pwd`/clang+llvm-9.0.0-x86_64-linux-gnu-ubuntu-16.04/bin:${PATH}"
export CC="`pwd`/clang+llvm-9.0.0-x86_64-linux-gnu-ubuntu-16.04/bin/clang"
#export LDFLAGS="-Lclang+llvm-9.0.0-x86_64-linux-gnu-ubuntu-16.04/lib"
#export CPPFLAGS="-Lclang+llvm-9.0.0-x86_64-linux-gnu-ubuntu-16.04/include"
./build_web.sh