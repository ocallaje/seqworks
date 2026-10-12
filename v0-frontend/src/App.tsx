import type { ReactNode } from 'react'
import { BrowserRouter, Navigate, Route, Routes, useLocation } from 'react-router-dom'
import { Loader2 } from 'lucide-react'
import { AuthProvider, useAuth } from '@/lib/auth'
import { AppShell } from '@/components/layout/app-shell'
import { LoginPage } from '@/pages/login'
import { AuthCallbackPage } from '@/pages/auth-callback'
import { DashboardPage } from '@/pages/dashboard'
import { PipelinePage } from '@/pages/pipeline'

function RequireAuth({ children }: { children: ReactNode }) {
  const { session, isLoading } = useAuth()
  const location = useLocation()
  if (isLoading) {
    return (
      <div className="flex min-h-dvh items-center justify-center">
        <Loader2 className="size-6 animate-spin text-primary" aria-label="Loading session" />
      </div>
    )
  }
  if (!session) return <Navigate to="/login" replace state={{ from: location.pathname + location.search }} />
  return children
}

export function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <Routes>
          <Route path="/login" element={<LoginPage />} />
          <Route path="/auth/callback" element={<AuthCallbackPage />} />
          <Route
            element={
              <RequireAuth>
                <AppShell />
              </RequireAuth>
            }
          >
            <Route index element={<DashboardPage />} />
            <Route path="pipelines/:pipelineId" element={<PipelinePage />} />
          </Route>
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </AuthProvider>
    </BrowserRouter>
  )
}
