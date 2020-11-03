#!/bin/bash

set -euo pipefail

TEST_CASE_RUN_TARGETS=${TEST_CASE_RUN_TARGETS:-.*}
TEST_CASE_RUN_SCHEMAS=${TEST_CASE_RUN_SCHEMAS:-.*}

# Prepare a new build of jtd-codegen
cargo build

build_python_image() {
    docker build --quiet \
        --build-arg MAIN_CLASS=$(jq -r .metadata.integration.python.MAIN_CLASS $1) \
        --build-arg CODEGEN_DIR=$(jq -r .metadata.integration.python.CODEGEN_DIR $1) \
        integration_tests/target/python
}

build_typescript_image() {
    docker build --quiet \
        --build-arg MAIN_CLASS=$(jq -r .metadata.integration.typescript.MAIN_CLASS $1) \
        --build-arg CODEGEN_DIR=$(jq -r .metadata.integration.typescript.CODEGEN_DIR $1) \
        integration_tests/target/typescript
}

integration_test_image() {
    if [[ ! $1 =~ $TEST_CASE_RUN_TARGETS ]]; then
        return
    fi

    echo "$2: $1"
    jtd-fuzz --num-values 1000 $2 | docker run -i $3 | jtd-validate $2
}

for schema in $(dirname $0)/schemas/*; do
    if [[ ! $schema =~ $TEST_CASE_RUN_SCHEMAS ]]; then
        continue
    fi

    schema_name=$(basename $schema .jtd.json)

    # Directories where we will output code to
    python_dir=integration_tests/target/python/codegen/$schema_name
    typescript_dir=integration_tests/target/typescript/codegen/$schema_name

    # Prepare output directories for jtd-codegen
    mkdir -p $python_dir $typescript_dir

    # Generate code for this schema
    ./target/debug/jtd-codegen \
        --python-out $python_dir \
        --typescript-out $typescript_dir \
        -- $schema

    # Run integration tests
    integration_test_image python $schema $(build_python_image $schema)
    integration_test_image typescript $schema $(build_typescript_image $schema)
done
