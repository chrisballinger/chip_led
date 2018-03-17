# CHIP LED

Flashing some LEDs on the Next Thing CHIP in Rust.

### Cross Compiling on macOS

Follow these directions to install a cross compiler / linker for armv7:

[https://github.com/chrisballinger/ct-ng-nextthing-chip](https://github.com/chrisballinger/ct-ng-nextthing-chip)

Create `.cargo/config` pointing to your install of cross compiling gcc toolchains:

```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "/Volumes/crosstool-ng/x-tools/armv7-chip-linux-gnueabihf/bin/armv7-chip-linux-gnueabihf-gcc"
[target.arm-unknown-linux-gnueabi]
linker = "/Volumes/crosstool-ng/x-tools/armv6-rpi-linux-gnueabi/bin/armv6-rpi-linux-gnueabi-gcc"
```

Build

```
$ cargo build --target=armv7-unknown-linux-gnueabihf
```

### Running on CHIP

`deploy.sh` modified as needed will compile and run on the CHIP.

```bash
#!/bin/bash

export SCRIPT_DIR=$(dirname $0)
export HOST="chip@chip.local"
export REMOTE_EXE_PATH="/home/chip/chip_led/chip_led"

echo "Running cargo build..."
cargo build --manifest-path=${SCRIPT_DIR}/Cargo.toml --target=armv7-unknown-linux-gnueabihf
echo "Copying to host ${HOST}"
scp ${SCRIPT_DIR}/target/armv7-unknown-linux-gnueabihf/debug/chip_led ${HOST}:${REMOTE_EXE_PATH}
echo "Running on host ${HOST}: ${REMOTE_EXE_PATH}"
ssh ${HOST} \""${REMOTE_EXE_PATH}"\"
```