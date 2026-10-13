#!/bin/bash

get_env() {
  local key="$1"
  grep -E "^${key}=" ../.env | cut -d'=' -f2- || true
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
# 3️⃣  Build the front‑end image (root Dockerfile)
# ------------------------------------------------------------------
# Build the React app
echo "Building React app..."
npm run build

FRONTEND_TAG="${HARBOR_HOST}/genelink/seqworks-react-fe:${VERSION}"
echo "🚀  Building Front-end image: ${FRONTEND_TAG}"
docker build $NO_CACHE_FLAG -t "$FRONTEND_TAG" .



# ------------------------------------------------------------------
# 4️⃣  Log in to Harbor (once for both images)
# ------------------------------------------------------------------
echo "🔐  Logging in to Harbor: $HARBOR_HOST"
printf '%s' "$ROBOT_PASSWORD" | docker login "$HARBOR_HOST" \
  -u "$HARBOR_ROBOT_USER" --password-stdin

# ------------------------------------------------------------------
# 5️⃣  Push image
# ------------------------------------------------------------------

echo "📤  Pushing Front‑end image to Harbor"
docker push "$FRONTEND_TAG"

echo "✅  Deployment complete!"