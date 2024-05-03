#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
cd ${SCRIPT_DIR}

# Linux version `cal` requires installation on GitHub Actions runnners
# Run this script on macOS machine and then commit the generated files

if [ "$(uname)" == 'Darwin' ]; then
  ./mk-outs.sh
fi
