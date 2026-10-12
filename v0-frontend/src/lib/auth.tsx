import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from 'react'
import { UserManager, WebStorageStateStore, type User } from 'oidc-client-ts'
import { config } from '@/config'

export interface SessionUser {
  username: string
  name: string
  email?: string
  roles: string[]
}

export interface Session {
  method: 'local' | 'oidc'
  accessToken: string
  expiresAt?: number
  user: SessionUser
}

interface AuthContextValue {
  session: Session | null
  isLoading: boolean
  loginLocal: (username: string, password: string) => Promise<void>
  loginOidc: (returnTo?: string) => Promise<void>
  completeOidc: () => Promise<string>
  logout: () => Promise<void>
}

const LOCAL_KEY = 'genelink.session'
const AuthContext = createContext<AuthContextValue | null>(null)

let userManager: UserManager | null = null
function getUserManager() {
  if (!config.oidcAuthority) return null
  if (!userManager) {
    userManager = new UserManager({
      authority: config.oidcAuthority,
      client_id: config.oidcClientId,
      redirect_uri: `${window.location.origin}/auth/callback`,
      post_logout_redirect_uri: `${window.location.origin}/login`,
      response_type: 'code',
      scope: config.oidcScope,
      automaticSilentRenew: true,
      userStore: new WebStorageStateStore({ store: window.sessionStorage }),
    })
  }
  return userManager
}

function sessionFromOidc(user: User): Session {
  const profile = user.profile as User['profile'] & { realm_access?: { roles?: string[] } }
  return {
    method: 'oidc',
    accessToken: user.access_token,
    expiresAt: user.expires_at ? user.expires_at * 1000 : undefined,
    user: {
      username: profile.preferred_username ?? profile.sub,
      name: profile.name ?? profile.preferred_username ?? 'GeneLink user',
      email: profile.email,
      roles: profile.realm_access?.roles ?? [],
    },
  }
}

function readLocalSession(): Session | null {
  try {
    const raw = sessionStorage.getItem(LOCAL_KEY)
    if (!raw) return null
    const session = JSON.parse(raw) as Session
    if (session.expiresAt && session.expiresAt < Date.now()) {
      sessionStorage.removeItem(LOCAL_KEY)
      return null
    }
    return session
  } catch {
    return null
  }
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [session, setSession] = useState<Session | null>(() => readLocalSession())
  const [isLoading, setIsLoading] = useState(() => !readLocalSession() && Boolean(getUserManager()))

  useEffect(() => {
    const mgr = getUserManager()
    if (!mgr) return
    let active = true
    mgr
      .getUser()
      .then((user) => {
        if (active && user && !user.expired) setSession(sessionFromOidc(user))
      })
      .finally(() => active && setIsLoading(false))

    const onLoaded = (user: User) => setSession(sessionFromOidc(user))
    const onExpired = () => setSession((s) => (s?.method === 'oidc' ? null : s))
    mgr.events.addUserLoaded(onLoaded)
    mgr.events.addAccessTokenExpired(onExpired)
    return () => {
      active = false
      mgr.events.removeUserLoaded(onLoaded)
      mgr.events.removeAccessTokenExpired(onExpired)
    }
  }, [])

  const loginLocal = useCallback(async (username: string, password: string) => {
    let next: Session
    if (config.demoMode) {
      await new Promise((r) => setTimeout(r, 400))
      next = {
        method: 'local',
        accessToken: 'demo',
        expiresAt: Date.now() + 8 * 3600_000,
        user: { username, name: username, roles: ['pipeline-operator'] },
      }
    } else {
      const res = await fetch(`${config.apiUrl}/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password }),
      })
      if (res.status === 401) throw new Error('Invalid username or password.')
      if (!res.ok) throw new Error(`Sign-in failed (${res.status}). Please try again.`)
      const data = (await res.json()) as {
        access_token: string
        expires_in?: number
        user?: Partial<SessionUser>
      }
      next = {
        method: 'local',
        accessToken: data.access_token,
        expiresAt: data.expires_in ? Date.now() + data.expires_in * 1000 : undefined,
        user: {
          username: data.user?.username ?? username,
          name: data.user?.name ?? username,
          email: data.user?.email,
          roles: data.user?.roles ?? [],
        },
      }
    }
    sessionStorage.setItem(LOCAL_KEY, JSON.stringify(next))
    setSession(next)
  }, [])

  const loginOidc = useCallback(async (returnTo = '/') => {
    const mgr = getUserManager()
    if (!mgr) {
      if (config.demoMode) {
        const next: Session = {
          method: 'oidc',
          accessToken: 'demo',
          expiresAt: Date.now() + 8 * 3600_000,
          user: { username: 'a.okafor', name: 'Ada Okafor', email: 'a.okafor@genelink.io', roles: ['researcher'] },
        }
        sessionStorage.setItem(LOCAL_KEY, JSON.stringify(next))
        setSession(next)
        return
      }
      throw new Error('GeneLink single sign-on is not configured.')
    }
    await mgr.signinRedirect({ state: { returnTo } })
  }, [])

  const completeOidc = useCallback(async () => {
    const mgr = getUserManager()
    if (!mgr) throw new Error('GeneLink single sign-on is not configured.')
    const user = await mgr.signinRedirectCallback()
    setSession(sessionFromOidc(user))
    const state = user.state as { returnTo?: string } | undefined
    return state?.returnTo ?? '/'
  }, [])

  const logout = useCallback(async () => {
    const current = session
    sessionStorage.removeItem(LOCAL_KEY)
    setSession(null)
    const mgr = getUserManager()
    if (current?.method === 'oidc' && mgr) {
      await mgr.signoutRedirect()
    }
  }, [session])

  const value = useMemo(
    () => ({ session, isLoading, loginLocal, loginOidc, completeOidc, logout }),
    [session, isLoading, loginLocal, loginOidc, completeOidc, logout],
  )

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

export function useAuth() {
  const ctx = useContext(AuthContext)
  if (!ctx) throw new Error('useAuth must be used within AuthProvider')
  return ctx
}
