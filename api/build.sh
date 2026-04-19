#!/bin/bash
set -e

NO_CACHE_ARG=""
if [ "$1" == "--no-cache" ]; then
    NO_CACHE_ARG="--no-cache"
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

export DOCKER_BUILDKIT=1
export $(grep -v '^#' "$SCRIPT_DIR/.env" | xargs)

echo "cache is:"
echo "$NO_CACHE_ARG"

docker build \
  $NO_CACHE_ARG \
  -f "$SCRIPT_DIR/Dockerfile" \
  --build-arg CACHE_BUST=$(date +%s) \
  -t seqworks-api \
  "$PROJECT_ROOT"