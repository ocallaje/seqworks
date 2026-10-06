#!/usr/bin/env bash
# Django Docker Deployment Script
# Usage: ./deploy.sh [--no-cache]

set -euo pipefail   # safer Bash defaults
# ------------------------------------------------------------------
# 1️⃣  Determine environment  ───────────────────────────────────────
# ------------------------------------------------------------------
if [ ! -f ".env" ]; then
  echo ".env file not found! Please create it with your environment variables."
  exit 1
fi

# Grab the robot password
ROBOT_PASSWORD="$(grep -E '^HARBOR_ROBOT_PWD=' .env | cut -d'=' -f2-)" || true
if [[ -z $ROBOT_PASSWORD ]] ; then
  echo "❌  ROBOT_PASSWORD missing in .env – required for staging uploads."
  exit 1
fi

# Grab the harbor info
HARBOR_HOST="$(grep -E '^HARBOR_HOST=' .env | cut -d'=' -f2-)" || true
if [[ -z $HARBOR_HOST ]] ; then
  echo "❌  HARBOR_HOST missing in .env – required for staging uploads."
  exit 1
fi
HARBOR_ROBOT_USER="$(grep -E '^HARBOR_ROBOT_USER=' .env | cut -d'=' -f2-)" || true
if [[ -z $HARBOR_ROBOT_USER ]]; then
  echo "❌  HARBOR_ROBOT_USER missing in .env – required for staging uploads."
  exit 1
fi

VERSION="$(grep -E '^VERSION=' .env | cut -d'=' -f2-)" || true
if [[ -z $VERSION ]]; then
  echo "❌  VERSION missing in .env – required for staging uploads."
  exit 1
fi


echo "🚀  Staging environment – pushing image to Harbor"
# a) Build + tag image
IMAGE_TAG="$HARBOR_HOST/genelink/seqworks-api:$VERSION"
echo "🐳  Building image $IMAGE_TAG"
docker build -t "$IMAGE_TAG" -f ./api/dockerfile .

# b) Login to Harbor (using the robot account)
echo "🔐  Logging in to Harbor ($HARBOR_HOST) as ($HARBOR_ROBOT_USER)"

printf '%s' "$ROBOT_PASSWORD" | \
    docker login "$HARBOR_HOST" \
      -u "$HARBOR_ROBOT_USER" \
      --password-stdin
echo "✅ Logged in!"


# c) Push
echo "📤  Pushing image to Harbor"
docker push "$IMAGE_TAG"

echo "Deployment complete!"