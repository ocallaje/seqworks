#!/bin/sh
# Writes runtime configuration so a single image can be promoted across environments.
set -eu

json_escape() {
  printf '%s' "$1" | sed -e 's/\\/\\\\/g' -e 's/"/\\"/g'
}

cat > /usr/share/nginx/html/config.js <<EOF
window.__APP_CONFIG__ = {
  apiUrl: "$(json_escape "${API_URL:-}")",
  argoUrl: "$(json_escape "${ARGO_URL:-}")",
  argoNamespace: "$(json_escape "${ARGO_NAMESPACE:-argo}")",
  wsUrl: "$(json_escape "${WS_URL:-}")",
  oidcAuthority: "$(json_escape "${OIDC_AUTHORITY:-}")",
  oidcClientId: "$(json_escape "${OIDC_CLIENT_ID:-genelink-pipelines}")",
  oidcScope: "$(json_escape "${OIDC_SCOPE:-openid profile email}")",
  demoMode: "$(json_escape "${DEMO_MODE:-}")"
}
EOF

echo "genelink: wrote runtime config (api=${API_URL:-<unset>}, oidc=${OIDC_AUTHORITY:-<unset>})"
