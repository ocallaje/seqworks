import { useState, type FormEvent } from 'react'
import { Navigate, useLocation } from 'react-router-dom'
import { Eye, EyeOff, Loader2 } from 'lucide-react'
import { useAuth } from '@/lib/auth'
import { config } from '@/config'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/controls'
import { Logo, LogoMark } from '@/components/logo'

export function LoginPage() {
  const { session, loginLocal, loginOidc } = useAuth()
  const location = useLocation()
  const returnTo = (location.state as { from?: string } | null)?.from ?? '/'
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [pending, setPending] = useState<'local' | 'oidc' | null>(null)

  if (session) return <Navigate to={returnTo} replace />

  const onLocal = async (e: FormEvent) => {
    e.preventDefault()
    if (!username.trim() || !password) {
      setError('Enter the service account username and password.')
      return
    }
    setPending('local')
    setError(null)
    try {
      await loginLocal(username.trim(), password)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Sign-in failed.')
      setPending(null)
    }
  }

  const onOidc = async () => {
    setPending('oidc')
    setError(null)
    try {
      await loginOidc(returnTo)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Could not reach GeneLink sign-in.')
      setPending(null)
    }
  }

  return (
    <div className="grid min-h-dvh lg:grid-cols-[1fr_minmax(0,560px)]">
      <div className="relative hidden overflow-hidden lg:block">
        <img src="/images/flowcell.png" alt="" className="absolute inset-0 size-full object-cover" />
        <div className="absolute inset-0 bg-gradient-to-t from-black/75 via-black/20 to-transparent" />
        <div className="absolute inset-x-0 bottom-0 flex flex-col gap-3 p-12">
          <p className="font-mono text-xs tracking-wide text-white/70 uppercase">Seqworks Orchestrator</p>
          <p className="max-w-md text-2xl leading-snug font-semibold text-balance text-white">
            Configure, launch and monitor RNA-seq pipelines
          </p>
        </div>
      </div>

      <main className="flex flex-col justify-center px-6 py-12 sm:px-12">
        <div className="mx-auto flex w-full max-w-sm flex-col gap-8">
          <Logo />

          <div className="flex flex-col gap-1.5">
            <h1 className="text-2xl font-semibold tracking-tight text-foreground">Sign in</h1>
            <p className="text-sm leading-relaxed text-muted-foreground">
              Use your GeneLink account, or a local service account.
            </p>
          </div>

          <Button size="lg" onClick={onOidc} disabled={pending !== null} className="w-full">
            {pending === 'oidc' ? <Loader2 className="animate-spin" /> : <LogoMark className="size-5 bg-primary-foreground/15" />}
            Sign in with GeneLink
          </Button>

          <div className="flex items-center gap-3 text-xs text-muted-foreground">
            <span className="h-px flex-1 bg-border" />
            or use a service account
            <span className="h-px flex-1 bg-border" />
          </div>

          <form onSubmit={onLocal} className="flex flex-col gap-4" noValidate>
            <div className="flex flex-col gap-2">
              <label htmlFor="username" className="text-sm font-medium text-foreground">
                Username
              </label>
              <Input
                id="username"
                autoComplete="username"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                placeholder="svc-pipeline"
                className="h-10"
              />
            </div>
            <div className="flex flex-col gap-2">
              <label htmlFor="password" className="text-sm font-medium text-foreground">
                Password
              </label>
              <div className="relative">
                <Input
                  id="password"
                  type={showPassword ? 'text' : 'password'}
                  autoComplete="current-password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  className="h-10 pr-10"
                />
                <button
                  type="button"
                  onClick={() => setShowPassword((s) => !s)}
                  className="absolute top-1/2 right-1 -translate-y-1/2 rounded-md p-2 text-muted-foreground hover:text-foreground"
                  aria-label={showPassword ? 'Hide password' : 'Show password'}
                >
                  {showPassword ? <EyeOff className="size-4" /> : <Eye className="size-4" />}
                </button>
              </div>
            </div>

            {error ? (
              <p className="rounded-md border border-danger/30 bg-danger/10 px-3 py-2 text-sm text-danger" role="alert">
                {error}
              </p>
            ) : null}

            <Button type="submit" variant="outline" size="lg" disabled={pending !== null} className="w-full">
              {pending === 'local' ? <Loader2 className="animate-spin" /> : null}
              Sign in with service account
            </Button>
          </form>

          {config.demoMode ? (
            <p className="rounded-md bg-warning/15 px-3 py-2 text-xs leading-relaxed text-foreground">
              Demo mode: no orchestrator is configured, so any credentials will sign you in with sample data.
            </p>
          ) : null}
        </div>
      </main>
    </div>
  )
}
