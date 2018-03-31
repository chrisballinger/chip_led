#!/bin/bash

export SCRIPT_DIR=$(dirname $0)
export HOST="chip@chip.local"
export REMOTE_EXE_PATH="/home/chip/chip_led_bin/chip_led"

echo "Running cargo build..."
cargo build --manifest-path=${SCRIPT_DIR}/Cargo.toml --target=armv7-unknown-linux-gnueabihf
echo "Copying to host ${HOST}"
rsync ${SCRIPT_DIR}/target/armv7-unknown-linux-gnueabihf/debug/chip_led ${HOST}:${REMOTE_EXE_PATH}
#echo "Running on host ${HOST}: ${REMOTE_EXE_PATH}"
#ssh -t ${HOST} "sudo ${REMOTE_EXE_PATH}"
