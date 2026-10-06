#!/usr/bin/env bash
# Deployment script for seqworks API + front‑end
# Usage: ./deploy.sh [--no-cache]

set -euo pipefail

# ------------------------------------------------------------------
# 1️⃣  Load environment variables from .env
# ------------------------------------------------------------------
if [[ ! -f ".env" ]]; then
  echo "❌  .env file not found! Create it with the required variables."
  exit 1
fi

# Helper to extract a value from .env
get_env() {
  local key="$1"
  grep -E "^${key}=" .env | cut -d'=' -f2- || true
}

ROBOT_PASSWORD=$(get_env "HARBOR_ROBOT_PWD")
HARBOR_HOST=$(get_env "HARBOR_HOST")
HARBOR_ROBOT_USER=$(get_env "HARBOR_ROBOT_USER")
VERSION=$(get_env "VERSION")

# Validate required settings
[[ -z $ROBOT_PASSWORD ]] && { echo "❌  HARBOR_ROBOT_PWD missing in .env"; exit 1; }
[[ -z $HARBOR_HOST ]]      && { echo "❌  HARBOR_HOST missing in .env"; exit 1; }
[[ -z $HARBOR_ROBOT_USER ]]&& { echo "❌  HARBOR_ROBOT_USER missing in .env"; exit 1; }
[[ -z $VERSION ]]          && { echo "❌  VERSION missing in .env"; exit 1; }

# ------------------------------------------------------------------
# 2️⃣  Build the API image
# ------------------------------------------------------------------
API_TAG="${HARBOR_HOST}/genelink/seqworks-api:${VERSION}"
echo "🚀  Building API image: ${API_TAG}"
NO_CACHE_FLAG=""
[[ "${1:-}" == "--no-cache" ]] && NO_CACHE_FLAG="--no-cache"
docker build $NO_CACHE_FLAG -t "$API_TAG" -f ./api/dockerfile .

# ------------------------------------------------------------------
# 3️⃣  Build the front‑end image (root Dockerfile)
# ------------------------------------------------------------------
FRONTEND_TAG="${HARBOR_HOST}/genelink/seqworks-frontend:${VERSION}"
echo "🚀  Building Front-end image: ${FRONTEND_TAG}"
docker build $NO_CACHE_FLAG -t "$FRONTEND_TAG" .

# ------------------------------------------------------------------
# 4️⃣  Log in to Harbor (once for both images)
# ------------------------------------------------------------------
echo "🔐  Logging in to Harbor: $HARBOR_HOST"
printf '%s' "$ROBOT_PASSWORD" | docker login "$HARBOR_HOST" \
  -u "$HARBOR_ROBOT_USER" --password-stdin

# ------------------------------------------------------------------
# 5️⃣  Push images
# ------------------------------------------------------------------
echo "📤  Pushing API image to Harbor"
docker push "$API_TAG"

echo "📤  Pushing Front‑end image to Harbor"
docker push "$FRONTEND_TAG"

echo "✅  Deployment complete!"