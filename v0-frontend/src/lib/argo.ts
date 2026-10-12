export type WorkflowPhase = 'Pending' | 'Running' | 'Succeeded' | 'Failed' | 'Error'
export type NodePhase = WorkflowPhase | 'Skipped' | 'Omitted'

export interface ArgoNode {
  id: string
  name: string
  displayName: string
  type: 'Pod' | 'Steps' | 'DAG' | 'StepGroup' | 'TaskGroup' | 'Retry' | 'Skipped' | 'Suspend'
  templateName?: string
  phase?: NodePhase
  startedAt?: string
  finishedAt?: string
  message?: string
}

export interface ArgoWorkflow {
  metadata: {
    name: string
    namespace: string
    uid: string
    creationTimestamp: string
    labels?: Record<string, string>
    annotations?: Record<string, string>
  }
  spec?: {
    arguments?: { parameters?: { name: string; value?: string }[] }
  }
  status?: {
    phase?: WorkflowPhase
    startedAt?: string
    finishedAt?: string | null
    progress?: string
    message?: string
    nodes?: Record<string, ArgoNode>
  }
}

export interface WorkflowEvent {
  type: 'ADDED' | 'MODIFIED' | 'DELETED'
  workflow: ArgoWorkflow
}

export const LABEL_PIPELINE = 'genelink.io/pipeline'
export const LABEL_SUBMITTED_BY = 'genelink.io/submitted-by'

export interface RunStep {
  id: string
  name: string
  phase: NodePhase
  startedAt?: string
  finishedAt?: string
}

export interface Run {
  name: string
  uid: string
  namespace: string
  pipelineId: string
  phase: WorkflowPhase
  createdAt: string
  startedAt?: string
  finishedAt?: string
  progress: { done: number; total: number }
  submittedBy: string
  message?: string
  parameters: { name: string; value: string }[]
  steps: RunStep[]
}

export function toRun(wf: ArgoWorkflow): Run {
  const nodes = Object.values(wf.status?.nodes ?? {})
  const steps: RunStep[] = nodes
    .filter((n) => n.type === 'Pod')
    .sort((a, b) => (a.startedAt ?? '9').localeCompare(b.startedAt ?? '9'))
    .map((n) => ({
      id: n.id,
      name: n.displayName || n.templateName || n.name,
      phase: n.phase ?? 'Pending',
      startedAt: n.startedAt,
      finishedAt: n.finishedAt,
    }))

  const [done, total] = (wf.status?.progress ?? '0/0').split('/').map((v) => Number.parseInt(v, 10) || 0)

  return {
    name: wf.metadata.name,
    uid: wf.metadata.uid,
    namespace: wf.metadata.namespace,
    pipelineId: wf.metadata.labels?.[LABEL_PIPELINE] ?? 'unknown',
    phase: wf.status?.phase ?? 'Pending',
    createdAt: wf.metadata.creationTimestamp,
    startedAt: wf.status?.startedAt,
    finishedAt: wf.status?.finishedAt ?? undefined,
    progress: { done, total: total || steps.length },
    submittedBy: wf.metadata.labels?.[LABEL_SUBMITTED_BY] ?? 'unknown',
    message: wf.status?.message,
    parameters: (wf.spec?.arguments?.parameters ?? []).map((p) => ({ name: p.name, value: p.value ?? '' })),
    steps,
  }
}

export function applyEvent(list: ArgoWorkflow[], event: WorkflowEvent): ArgoWorkflow[] {
  const uid = event.workflow.metadata.uid
  if (event.type === 'DELETED') return list.filter((w) => w.metadata.uid !== uid)
  const index = list.findIndex((w) => w.metadata.uid === uid)
  if (index === -1) return [event.workflow, ...list]
  const next = list.slice()
  next[index] = event.workflow
  return next
}
