#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
DEST_REPOSITORY="quay.io/image-mirror/"${SRC_REPOSITORY%/*}

skopeo sync --all --preserve-digests --src docker --dest docker --authfile auth.json $SRC_REPOSITORY $DEST_REPOSITORY
