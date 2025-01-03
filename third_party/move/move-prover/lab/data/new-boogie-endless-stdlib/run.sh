#!/bin/bash

ENDLESS_STD="../../../../../../endless-move/framework/endless-stdlib/sources"

# Check if the first argument is either "new" or "current"
if [[ "$1" != "new" && "$1" != "current" ]]; then
    echo "Invalid argument. The first argument must be 'new' or 'current'."
    exit 1
fi

# Benchmark per function (with `-f``). `-a` is for including the endless-natives.
cargo run --release -p prover-lab -- bench -a -f -c $1_boogie_1.toml $ENDLESS_STD/*.move $ENDLESS_STD/cryptography/*.move $ENDLESS_STD/data_structures/*.move

# Benchmark per module (without `-f`). `-a` is for including the endless-natives.
cargo run --release -p prover-lab -- bench -a -c $1_boogie_1.toml $ENDLESS_STD/*.move $ENDLESS_STD/cryptography/*.move $ENDLESS_STD/data_structures/*.move

# Benchmark per function (with `-f``). `-a` is for including the endless-natives.
cargo run --release -p prover-lab -- bench -a -f -c $1_boogie_2.toml $ENDLESS_STD/*.move $ENDLESS_STD/cryptography/*.move $ENDLESS_STD/data_structures/*.move

# Benchmark per module (without `-f`). `-a` is for including the endless-natives.
cargo run --release -p prover-lab -- bench -a -c $1_boogie_2.toml $ENDLESS_STD/*.move $ENDLESS_STD/cryptography/*.move $ENDLESS_STD/data_structures/*.move

# Benchmark per function (with `-f``). `-a` is for including the endless-natives.
cargo run --release -p prover-lab -- bench -a -f -c $1_boogie_3.toml $ENDLESS_STD/*.move $ENDLESS_STD/cryptography/*.move $ENDLESS_STD/data_structures/*.move

# Benchmark per module (without `-f`). `-a` is for including the endless-natives.
cargo run --release -p prover-lab -- bench -a -c $1_boogie_3.toml $ENDLESS_STD/*.move $ENDLESS_STD/cryptography/*.move $ENDLESS_STD/data_structures/*.move
