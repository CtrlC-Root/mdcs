#!/bin/bash

set -x

# verify we're running in a virtual env
if [ -z "$VIRTUAL_ENV" ]; then
    echo "\$VIRTUAL_ENV not set"
    exit 1
fi

# detect script location
TOP="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# detect documentation and build directories
DOCS="${TOP}/docs"
BUILD="${DOCS}/build"

# ensure build directory exists
mkdir -p "${BUILD}"

# generate HTML documentation
sphinx-build -M html "${TOP}" "${BUILD}"
