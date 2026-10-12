import { pipelines } from '@/pipelines/registry'
import { LABEL_PIPELINE, LABEL_SUBMITTED_BY, type ArgoNode, type ArgoWorkflow, type NodePhase, type WorkflowEvent, type WorkflowPhase } from './argo'

const NAMESPACE = 'genelink'
const users = ['svc-pipeline', 'a.okafor', 'm.lindqvist', 'r.tanaka', 'j.alvarez']

let counter = 0
function uid() {
  counter += 1
  return `demo-${Date.now().toString(36)}-${counter}`
}

function suffix() {
  return Math.random().toString(36).slice(2, 7)
}

function minutesAgo(m: number) {
  return new Date(Date.now() - m * 60_000).toISOString()
}

function buildWorkflow(opts: {
  pipelineId: string
  name?: string
  phase: WorkflowPhase
  startedMinutesAgo: number
  completedSteps: number
  user: string
  failedAtStep?: number
  parameters?: { name: string; value: string }[]
}): ArgoWorkflow {
  const pipeline = pipelines.find((p) => p.id === opts.pipelineId) ?? pipelines[0]
  const name = opts.name ?? `${pipeline.id}-${suffix()}`
  const startedAt = minutesAgo(opts.startedMinutesAgo)
  const stepDuration = 6
  const nodes: Record<string, ArgoNode> = {}

  pipeline.steps.forEach((step, i) => {
    let phase: NodePhase = 'Pending'
    if (opts.failedAtStep === i) phase = 'Failed'
    else if (opts.failedAtStep !== undefined && i > opts.failedAtStep) phase = 'Omitted'
    else if (i < opts.completedSteps) phase = 'Succeeded'
    else if (i === opts.completedSteps && opts.phase === 'Running') phase = 'Running'

    const id = `${name}-${i}`
    const started = phase !== 'Pending' && phase !== 'Omitted' ? minutesAgo(opts.startedMinutesAgo - i * stepDuration) : undefined
    nodes[id] = {
      id,
      name: `${name}.${step}`,
      displayName: step,
      type: 'Pod',
      templateName: step.toLowerCase().replace(/\s+/g, '-'),
      phase,
      startedAt: started,
      finishedAt:
        phase === 'Succeeded' || phase === 'Failed' ? minutesAgo(opts.startedMinutesAgo - (i + 1) * stepDuration) : undefined,
      message: phase === 'Failed' ? 'Error (exit code 137): OOMKilled' : undefined,
    }
  })

  const done = Object.values(nodes).filter((n) => n.phase === 'Succeeded').length

  return {
    metadata: {
      name,
      namespace: NAMESPACE,
      uid: uid(),
      creationTimestamp: startedAt,
      labels: { [LABEL_PIPELINE]: pipeline.id, [LABEL_SUBMITTED_BY]: opts.user },
    },
    spec: {
      arguments: {
        parameters: opts.parameters ?? [
          { name: 'input', value: `s3://genelink-data/projects/p0${Math.floor(Math.random() * 90 + 10)}/samplesheet.csv` },
          { name: 'genome', value: 'GRCh38' },
          { name: 'max_cpus', value: '16' },
        ],
      },
    },
    status: {
      phase: opts.phase,
      startedAt: opts.phase === 'Pending' ? undefined : startedAt,
      finishedAt:
        opts.phase === 'Succeeded' || opts.phase === 'Failed' || opts.phase === 'Error'
          ? minutesAgo(Math.max(0, opts.startedMinutesAgo - pipeline.steps.length * stepDuration))
          : null,
      progress: `${done}/${pipeline.steps.length}`,
      message: opts.failedAtStep !== undefined ? `child '${name}-${opts.failedAtStep}' failed` : undefined,
      nodes,
    },
  }
}

let store: ArgoWorkflow[] = [
  buildWorkflow({ pipelineId: 'bulk-rnaseq', phase: 'Running', startedMinutesAgo: 22, completedSteps: 3, user: 'a.okafor' }),
  buildWorkflow({ pipelineId: 'scrnaseq', phase: 'Running', startedMinutesAgo: 9, completedSteps: 1, user: 'r.tanaka' }),
  buildWorkflow({ pipelineId: 'differential-expression', phase: 'Running', startedMinutesAgo: 30, completedSteps: 4, user: 'svc-pipeline' }),
  buildWorkflow({ pipelineId: 'fusion-detection', phase: 'Pending', startedMinutesAgo: 1, completedSteps: 0, user: 'm.lindqvist' }),
  buildWorkflow({ pipelineId: 'bulk-rnaseq', phase: 'Succeeded', startedMinutesAgo: 140, completedSteps: 6, user: 'svc-pipeline' }),
  buildWorkflow({ pipelineId: 'read-qc', phase: 'Succeeded', startedMinutesAgo: 210, completedSteps: 4, user: 'j.alvarez' }),
  buildWorkflow({ pipelineId: 'scrnaseq', phase: 'Failed', startedMinutesAgo: 300, completedSteps: 2, failedAtStep: 2, user: 'r.tanaka' }),
  buildWorkflow({ pipelineId: 'differential-expression', phase: 'Succeeded', startedMinutesAgo: 420, completedSteps: 6, user: 'a.okafor' }),
  buildWorkflow({ pipelineId: 'bulk-rnaseq', phase: 'Failed', startedMinutesAgo: 640, completedSteps: 1, failedAtStep: 1, user: 'm.lindqvist' }),
  buildWorkflow({ pipelineId: 'read-qc', phase: 'Succeeded', startedMinutesAgo: 900, completedSteps: 4, user: 'svc-pipeline' }),
  buildWorkflow({ pipelineId: 'fusion-detection', phase: 'Succeeded', startedMinutesAgo: 1300, completedSteps: 6, user: users[2] }),
]

type Listener = (event: WorkflowEvent) => void
const listeners = new Set<Listener>()

function emit(type: WorkflowEvent['type'], workflow: ArgoWorkflow) {
  for (const l of listeners) l({ type, workflow: structuredClone(workflow) })
}

export function listDemoWorkflows(): Promise<ArgoWorkflow[]> {
  return new Promise((resolve) => setTimeout(() => resolve(structuredClone(store)), 350))
}

export function submitDemoRun(pipelineId: string, name: string, parameters: { name: string; value: string }[], user: string) {
  const wf = buildWorkflow({ pipelineId, name, phase: 'Pending', startedMinutesAgo: 0, completedSteps: 0, user, parameters })
  store = [wf, ...store]
  emit('ADDED', wf)
  return wf.metadata.name
}

function nowIso() {
  return new Date().toISOString()
}

function advance(wf: ArgoWorkflow) {
  const status = wf.status
  if (!status?.nodes) return false
  const nodes = Object.values(status.nodes)

  if (status.phase === 'Pending') {
    status.phase = 'Running'
    status.startedAt = nowIso()
    nodes[0].phase = 'Running'
    nodes[0].startedAt = nowIso()
    return true
  }
  if (status.phase !== 'Running') return false

  const runningIndex = nodes.findIndex((n) => n.phase === 'Running')
  if (runningIndex === -1) return false
  nodes[runningIndex].phase = 'Succeeded'
  nodes[runningIndex].finishedAt = nowIso()
  const next = nodes[runningIndex + 1]
  if (next) {
    next.phase = 'Running'
    next.startedAt = nowIso()
  } else {
    status.phase = 'Succeeded'
    status.finishedAt = nowIso()
  }
  status.progress = `${nodes.filter((n) => n.phase === 'Succeeded').length}/${nodes.length}`
  return true
}

export function stopDemoRun(name: string) {
  const wf = store.find((w) => w.metadata.name === name)
  if (!wf?.status?.nodes) return
  for (const node of Object.values(wf.status.nodes)) {
    if (node.phase === 'Running') {
      node.phase = 'Failed'
      node.message = 'Stopped by user'
      node.finishedAt = nowIso()
    } else if (node.phase === 'Pending') node.phase = 'Omitted'
  }
  wf.status.phase = 'Failed'
  wf.status.message = 'Stopped with strategy \'Stop\''
  wf.status.finishedAt = nowIso()
  emit('MODIFIED', wf)
}

export function retryDemoRun(name: string) {
  const wf = store.find((w) => w.metadata.name === name)
  if (!wf?.status?.nodes) return
  for (const node of Object.values(wf.status.nodes)) {
    if (node.phase === 'Failed' || node.phase === 'Omitted') {
      node.phase = 'Pending'
      node.message = undefined
      node.startedAt = undefined
      node.finishedAt = undefined
    }
  }
  const first = Object.values(wf.status.nodes).find((n) => n.phase === 'Pending')
  if (first) {
    first.phase = 'Running'
    first.startedAt = nowIso()
  }
  wf.status.phase = 'Running'
  wf.status.message = undefined
  wf.status.finishedAt = null
  emit('MODIFIED', wf)
}

let timer: ReturnType<typeof setInterval> | undefined

export function subscribeDemo(listener: Listener) {
  listeners.add(listener)
  if (!timer) {
    timer = setInterval(() => {
      const active = store.filter((w) => w.status?.phase === 'Running' || w.status?.phase === 'Pending')
      if (active.length === 0) return
      const wf = active[Math.floor(Math.random() * active.length)]
      if (advance(wf)) emit('MODIFIED', wf)
    }, 3500)
  }
  return () => {
    listeners.delete(listener)
    if (listeners.size === 0 && timer) {
      clearInterval(timer)
      timer = undefined
    }
  }
}
