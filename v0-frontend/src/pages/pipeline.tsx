import { Link, useParams } from 'react-router-dom'
import { getPipeline } from '@/pipelines/registry'
import { PipelineForm } from '@/components/pipeline/pipeline-form'

export function PipelinePage() {
  const { pipelineId } = useParams()
  const pipeline = getPipeline(pipelineId)

  if (!pipeline) {
    return (
      <div className="flex flex-col items-start gap-3 px-4 py-10 md:px-8">
        <h1 className="text-xl font-semibold text-foreground">Pipeline not found</h1>
        <Link to="/" className="text-sm font-medium text-primary hover:underline">
          Back to dashboard
        </Link>
      </div>
    )
  }

  return (
    <div className="flex flex-col gap-8 px-4 py-6 md:px-8 md:py-8">
      <header className="flex flex-col gap-2">
        <div className="flex flex-wrap items-center gap-2">
          <h1 className="text-2xl font-semibold tracking-tight text-foreground text-balance">{pipeline.name}</h1>
          <span className="rounded-md border bg-muted/50 px-2 py-0.5 font-mono text-xs text-muted-foreground">
            {pipeline.workflow}@{pipeline.version}
          </span>
        </div>
        <p className="max-w-2xl text-sm leading-relaxed text-muted-foreground text-pretty">{pipeline.description}</p>
      </header>
      <PipelineForm key={pipeline.id} pipeline={pipeline} />
    </div>
  )
}
