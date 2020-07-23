#!/usr/bin/env bash

# Abrasion distribution startup script.
# To whom may read this: Abrasion-based programs do not require any CWD
# shenanigans, and you don't really have to use this script at all! It's here
# so that logging-related env vars are set to some sane defaults.

# Typical cursedness to get the path to the directory containing this script.
# We refer to this directory as the 'distribution root'. It should contain the
# engine/ and assets/ directories.
DST="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# Show a backtrace on panic.
export RUST_BACKTRACE=1
# Enable info logging.
export RUST_LOG=info
# For debug logs, uncomment the following line.
#export RUST_LOG=debug

# Now, just run abrasion!
exec "$DST/engine/engine"
