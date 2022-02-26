#!/bin/bash
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && (pwd -W 2>/dev/null || pwd))

(cd $SCRIPT_DIR && cargo run --release)
