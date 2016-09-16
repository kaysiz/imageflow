#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
IMAGE_NAME="$(basename "${SCRIPT_DIR}")"

"${SCRIPT_DIR}/../test.sh" "${IMAGE_NAME}"