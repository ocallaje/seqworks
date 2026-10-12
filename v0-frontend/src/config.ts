export interface AppConfig {
  apiUrl: string
  argoUrl: string
  argoNamespace: string
  wsUrl: string
  oidcAuthority: string
  oidcClientId: string
  oidcScope: string
  demoMode: boolean
}

type RawConfig = Partial<Record<keyof AppConfig, string | boolean>>

declare global {
  interface Window {
    __APP_CONFIG__?: RawConfig
  }
}

const runtime: RawConfig = typeof window !== 'undefined' ? (window.__APP_CONFIG__ ?? {}) : {}
const env = import.meta.env

function pick(key: keyof AppConfig, envKey: string, fallback = ''): string {
  const value = runtime[key]
  if (typeof value === 'string' && value.length > 0) return value
  const fromEnv = env[envKey]
  return typeof fromEnv === 'string' && fromEnv.length > 0 ? fromEnv : fallback
}

function toBool(value: string | boolean | undefined): boolean | undefined {
  if (typeof value === 'boolean') return value
  if (value === 'true' || value === '1') return true
  if (value === 'false' || value === '0') return false
  return undefined
}

const apiUrl = pick('apiUrl', 'VITE_API_URL').replace(/\/$/, '')

export const config: AppConfig = {
  apiUrl,
  argoUrl: pick('argoUrl', 'VITE_ARGO_URL', apiUrl ? `${apiUrl}/argo` : '').replace(/\/$/, ''),
  argoNamespace: pick('argoNamespace', 'VITE_ARGO_NAMESPACE', 'argo'),
  wsUrl: pick('wsUrl', 'VITE_WS_URL', apiUrl ? `${apiUrl.replace(/^http/, 'ws')}/ws/workflows` : ''),
  oidcAuthority: pick('oidcAuthority', 'VITE_OIDC_AUTHORITY'),
  oidcClientId: pick('oidcClientId', 'VITE_OIDC_CLIENT_ID', 'genelink-pipelines'),
  oidcScope: pick('oidcScope', 'VITE_OIDC_SCOPE', 'openid profile email'),
  demoMode: toBool(runtime.demoMode) ?? toBool(env.VITE_DEMO_MODE) ?? !apiUrl,
}
