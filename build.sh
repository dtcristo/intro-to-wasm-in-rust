#!/bin/sh
set -e

cd 1-add && ./build.sh && cd ..
cd 2-add-and-log && ./build.sh && cd ..
cd 3-add-rust && ./build.sh && cd ..
cd 4-hello-world && ./build.sh && cd ..
cd 5-memory && ./build.sh && cd ..
cd 6-dom && ./build.sh && cd ..
