#!/bin/bash

export SCRIPT_DIR=$(dirname $0)
export HOST="chip@chip.local"
export REMOTE_SRC_PATH="/home/chip/chip_led_src"

echo "Running cargo build..."
echo "Copying source to host ${HOST}"
scp -r ./ ${HOST}:${REMOTE_SRC_PATH}
echo "Building on host ${HOST}: ${REMOTE_EXE_PATH}"
export BUILD_COMMAND="cargo build --manifest-path=${REMOTE_SRC_PATH}/Cargo.toml --target=armv7-unknown-linux-gnueabihf"
ssh ${HOST} \""${BUILD_COMMAND}"\"
