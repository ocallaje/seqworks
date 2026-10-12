import { useEffect, useRef, useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { Loader2 } from 'lucide-react'
import { useAuth } from '@/lib/auth'

export function AuthCallbackPage() {
  const { completeOidc } = useAuth()
  const navigate = useNavigate()
  const [error, setError] = useState<string | null>(null)
  const handled = useRef(false)

  useEffect(() => {
    // The authorization code can only be exchanged once; guard against StrictMode double effects.
    if (handled.current) return
    handled.current = true
    completeOidc()
      .then((returnTo) => navigate(returnTo, { replace: true }))
      .catch((err: unknown) => setError(err instanceof Error ? err.message : 'Sign-in could not be completed.'))
  }, [completeOidc, navigate])

  return (
    <main className="flex min-h-dvh flex-col items-center justify-center gap-4 px-6 text-center">
      {error ? (
        <>
          <h1 className="text-lg font-semibold text-foreground">GeneLink sign-in failed</h1>
          <p className="max-w-sm text-sm leading-relaxed text-muted-foreground">{error}</p>
          <Link to="/login" className="text-sm font-medium text-primary hover:underline">
            Back to sign in
          </Link>
        </>
      ) : (
        <>
          <Loader2 className="size-6 animate-spin text-primary" aria-hidden />
          <p className="text-sm text-muted-foreground">Completing GeneLink sign-in…</p>
        </>
      )}
    </main>
  )
}
