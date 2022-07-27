#!/bin/sh

set -eu

TEST_OUTPUT_DIR="$(pwd)/test-data/output"

# clean up the test output directory
[ -d "${TEST_OUTPUT_DIR}" ] && sudo rm -r "${TEST_OUTPUT_DIR}"/*.pdf
