#!/bin/bash
set -e
cargo build --release
cargo objcopy --release -- -O binary --binary-architecture=i386:x86 rgraph.com
dosbox ./rgraph.com