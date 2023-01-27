#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

echo "Container start"
exec "$@"
echo "Container finished"
