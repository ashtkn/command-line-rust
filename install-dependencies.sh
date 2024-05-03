#!/usr/bin/env bash

if [ "$(uname)" == 'Darwin' ]; then
  brew update
  brew install fortune
fi
