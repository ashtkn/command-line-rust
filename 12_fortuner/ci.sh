#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
cd ${SCRIPT_DIR}

# Linux version `fortune` is very different from macOS version
# Run this script on macOS machine and then commit the generated files

if [ "$(uname)" == 'Darwin' ]; then
  ./mk-dat.sh
  ./mk-outs.sh
fi
