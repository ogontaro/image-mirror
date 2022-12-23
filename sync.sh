#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

skopeo sync --all --preserve-digests --keep-going --retry-times 3 --scoped --src docker --dest docker --authfile auth.json $SRC_REPOSITORY quay.io/image-mirror
