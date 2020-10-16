#!/bin/bash

set -euo pipefail

# Prepare a new build of jtd-codegen
cargo build

build_python_image() {
    docker build --quiet \
        --build-arg MAIN_CLASS=$(jq -r .metadata.integration.python.MAIN_CLASS $1) \
        --build-arg CODEGEN_DIR=$(jq -r .metadata.integration.python.CODEGEN_DIR $1) \
        integration_tests/target/python
}

integration_test_image() {
    echo "$2: $1"
    jtd-fuzz --num-values 1000 $2 | docker run -i $3 | jtd-validate $2
}

for schema in $(dirname $0)/schemas/*; do
    schema_name=$(basename $schema .jtd.json)

    # Directories where we will output code to
    python_dir=integration_tests/target/python/codegen/$schema_name

    # Prepare output directories for jtd-codegen
    mkdir -p $python_dir

    # Generate code for this schema
    ./target/debug/jtd-codegen \
        --python-out $python_dir \
        -- $schema

    # Run integration tests
    integration_test_image python $schema $(build_python_image $schema)
done
