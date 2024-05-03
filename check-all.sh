#!/usr/bin/env bash

for DIR in [01]*; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo check && cargo fmt --check && cargo clippy)
done

echo "Done."
