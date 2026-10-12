import useSWR from 'swr'
import { config } from '@/config'
import { toRun, type ArgoWorkflow } from './argo'
import { listDemoWorkflows, retryDemoRun, stopDemoRun, submitDemoRun } from './demo'
import { useAuth } from './auth'

export const RUNS_KEY = 'argo/workflows'

export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
  ) {
    super(message)
  }
}

async function request<T>(url: string, token: string, init: RequestInit = {}): Promise<T> {
  const res = await fetch(url, {
    ...init,
    headers: {
      Accept: 'application/json',
      ...(init.body ? { 'Content-Type': 'application/json' } : {}),
      Authorization: `Bearer ${token}`,
      ...init.headers,
    },
  })
  if (!res.ok) {
    let detail = res.statusText
    try {
      const body = (await res.json()) as { message?: string; detail?: string }
      detail = body.message ?? body.detail ?? detail
    } catch {
      // response had no JSON body
    }
    throw new ApiError(detail || `Request failed (${res.status})`, res.status)
  }
  return (res.status === 204 ? undefined : await res.json()) as T
}

async function listWorkflows(token: string): Promise<ArgoWorkflow[]> {
  if (config.demoMode) return listDemoWorkflows()
  const url = `${config.argoUrl}/api/v1/workflows/${encodeURIComponent(config.argoNamespace)}?listOptions.limit=200`
  const data = await request<{ items: ArgoWorkflow[] | null }>(url, token)
  return (data.items ?? []).sort((a, b) => b.metadata.creationTimestamp.localeCompare(a.metadata.creationTimestamp))
}

export function useWorkflows() {
  const { session } = useAuth()
  const token = session?.accessToken
  return useSWR(token ? [RUNS_KEY, token] : null, ([, t]) => listWorkflows(t), {
    refreshInterval: 60_000,
    revalidateOnFocus: true,
  })
}

export function useRuns() {
  const { data, error, isLoading, mutate } = useWorkflows()
  return { runs: data?.map(toRun) ?? [], error: error as Error | undefined, isLoading, mutate }
}

export interface SubmitRunInput {
  pipelineId: string
  runName: string
  parameters: Record<string, unknown>
}

export async function submitRun(token: string, user: string, input: SubmitRunInput): Promise<string> {
  const parameters = Object.entries(input.parameters).map(([name, value]) => ({
    name,
    value: Array.isArray(value) ? value.join(',') : String(value),
  }))
  if (config.demoMode) {
    await new Promise((r) => setTimeout(r, 600))
    return submitDemoRun(input.pipelineId, input.runName, parameters, user)
  }
  const data = await request<{ workflowName: string }>(
    `${config.apiUrl}/pipelines/${encodeURIComponent(input.pipelineId)}/runs`,
    token,
    { method: 'POST', body: JSON.stringify({ name: input.runName, parameters: input.parameters }) },
  )
  return data.workflowName
}

export async function stopRun(token: string, name: string) {
  if (config.demoMode) return stopDemoRun(name)
  await request(`${config.argoUrl}/api/v1/workflows/${config.argoNamespace}/${encodeURIComponent(name)}/stop`, token, {
    method: 'PUT',
    body: JSON.stringify({}),
  })
}

export async function retryRun(token: string, name: string) {
  if (config.demoMode) return retryDemoRun(name)
  await request(`${config.argoUrl}/api/v1/workflows/${config.argoNamespace}/${encodeURIComponent(name)}/retry`, token, {
    method: 'PUT',
    body: JSON.stringify({ restartSuccessful: false }),
  })
}

export function argoUiLink(name: string) {
  return config.argoUrl ? `${config.argoUrl}/workflows/${config.argoNamespace}/${name}` : undefined
}
