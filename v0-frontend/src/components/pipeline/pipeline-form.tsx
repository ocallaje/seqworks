import { useMemo, useState, type FormEvent } from 'react'
import { useNavigate } from 'react-router-dom'
import { useSWRConfig } from 'swr'
import { Loader2, Play, RotateCcw } from 'lucide-react'
import { defaultValues, validate, type FieldValue, type PipelineDefinition } from '@/pipelines/types'
import { gencodeOptionsByGenome } from '@/pipelines/registry'
import { RUNS_KEY, submitRun } from '@/lib/api'
import { useAuth } from '@/lib/auth'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/controls'
import { FieldInput } from './field-input'

function suggestName(pipelineId: string) {
  const d = new Date()
  const stamp = `${d.getFullYear()}${String(d.getMonth() + 1).padStart(2, '0')}${String(d.getDate()).padStart(2, '0')}`
  return `${pipelineId}-${stamp}-${Math.random().toString(36).slice(2, 6)}`
}

const NAME_PATTERN = /^[a-z0-9]([-a-z0-9]*[a-z0-9])?$/

export function PipelineForm({ pipeline }: { pipeline: PipelineDefinition }) {
  const { session } = useAuth()
  const navigate = useNavigate()
  const { mutate } = useSWRConfig()
  const [values, setValues] = useState<Record<string, FieldValue>>(() => defaultValues(pipeline))
  const [runName, setRunName] = useState(() => suggestName(pipeline.id))
  const [errors, setErrors] = useState<Record<string, string>>({})
  const [submitError, setSubmitError] = useState<string | null>(null)
  const [submitting, setSubmitting] = useState(false)

  const changed = useMemo(() => {
    const defaults = defaultValues(pipeline)
    return Object.keys(values).filter((k) => JSON.stringify(values[k]) !== JSON.stringify(defaults[k])).length
  }, [values, pipeline])

  const setValue = (key: string, value: FieldValue) => {
    setValues((v) => ({ ...v, [key]: value }))
    if (errors[key]) setErrors(({ [key]: _removed, ...rest }) => rest)
  }

  const onSubmit = async (e: FormEvent) => {
    e.preventDefault()
    const nextErrors = validate(pipeline, values)
    if (!NAME_PATTERN.test(runName) || runName.length > 63) {
      nextErrors.__name = 'Lowercase letters, numbers and hyphens only (max 63 characters).'
    }
    setErrors(nextErrors)
    if (Object.keys(nextErrors).length > 0) {
      const first = Object.keys(nextErrors)[0]
      document.getElementById(first === '__name' ? 'run-name' : `field-${first}`)?.focus()
      return
    }
    if (!session) return
    setSubmitting(true)
    setSubmitError(null)
    try {
      const name = await submitRun(session.accessToken, session.user.username, {
        pipelineId: pipeline.id,
        runName,
        parameters: values,
      })
      await mutate((key) => Array.isArray(key) && key[0] === RUNS_KEY)
      navigate(`/?run=${encodeURIComponent(name)}`)
    } catch (err) {
      setSubmitError(err instanceof Error ? err.message : 'Submission failed')
      setSubmitting(false)
    }
  }

  const reset = () => {
    setValues(defaultValues(pipeline))
    setErrors({})
  }

  return (
    <form onSubmit={onSubmit} noValidate className="flex flex-col gap-8 xl:flex-row xl:items-start">
      <div className="flex min-w-0 flex-1 flex-col gap-6">
        <section className="flex flex-col gap-2 rounded-lg border bg-card p-5">
          <label htmlFor="run-name" className="text-sm font-medium text-foreground">
            Run name
          </label>
          <Input
            id="run-name"
            value={runName}
            onChange={(e) => setRunName(e.target.value.toLowerCase())}
            className="font-mono text-[13px]"
            aria-invalid={Boolean(errors.__name)}
            aria-describedby="run-name-help"
            spellCheck={false}
            autoComplete="off"
          />
          <p id="run-name-help" className={errors.__name ? 'text-sm text-danger' : 'text-sm text-muted-foreground'}>
            {errors.__name ?? 'Used as the Argo workflow name.'}
          </p>
        </section>

        {pipeline.sections.map((section) => (
          <section key={section.title} className="flex flex-col gap-5 rounded-lg border bg-card p-5">
            <div className="flex flex-col gap-1">
              <h2 className="text-base font-semibold text-foreground">{section.title}</h2>
              {section.description ? <p className="text-sm text-muted-foreground">{section.description}</p> : null}
            </div>
            <div className="grid gap-5 md:grid-cols-2">

              {section.fields.map((field) => {
                const fieldToRender: Field =
                  field.key === 'gencode' && field.type === 'select'
                    ? {
                        ...field,
                        options:
                          gencodeOptionsByGenome[String(values.genome)] ?? [],
                      }
                    : field

                return (
                  <div
                    key={field.key}
                    className={field.type === 'toggle' ? 'md:col-span-2' : 'contents'}
                  >
                    <FieldInput
                      field={fieldToRender}
                      value={values[field.key]}
                      error={errors[field.key]}
                      onChange={(v) => {
                        setValue(field.key, v)

                        if (field.key === 'genome') {
                          const options =
                            gencodeOptionsByGenome[String(v)] ?? []

                          const currentGencode = values.gencode

                          if (
                            !options.some(
                              (option) => option.value === currentGencode,
                            )
                          ) {
                            setValue('gencode', options[0]?.value ?? '')
                          }
                        }
                      }}
                    />
                  </div>
                )
              })}


            </div>
          </section>
        ))}
      </div>

      <aside className="flex flex-col gap-4 xl:sticky xl:top-8 xl:w-80 xl:shrink-0">
        <div className="flex flex-col gap-4 rounded-lg border bg-card p-5">
          <div className="flex flex-col gap-1">
            <h2 className="text-base font-semibold text-foreground">Launch</h2>
            <p className="text-sm text-muted-foreground">
              {changed === 0 ? 'Using pipeline defaults.' : `${changed} parameter${changed === 1 ? '' : 's'} changed from defaults.`}
            </p>
          </div>

          <ol className="flex flex-col gap-2" aria-label="Workflow steps">
            {pipeline.steps.map((step, i) => (
              <li key={step} className="flex items-center gap-3 text-sm text-muted-foreground">
                <span className="flex size-5 items-center justify-center rounded-full border font-mono text-[10px] text-muted-foreground">
                  {i + 1}
                </span>
                {step}
              </li>
            ))}
          </ol>

          <pre className="max-h-56 overflow-auto rounded-md break-all whitespace-pre-wrap bg-muted/60 p-3 font-mono text-xs leading-relaxed text-foreground">
            {Object.entries(values)
              .map(([k, v]) => `${k}: ${Array.isArray(v) ? `[${v.join(', ')}]` : v === '' ? '""' : v}`)
              .join('\n')}
          </pre>

          {submitError ? (
            <p className="rounded-md border border-danger/30 bg-danger/10 px-3 py-2 text-sm text-danger" role="alert">
              {submitError}
            </p>
          ) : null}
          {Object.keys(errors).length > 0 ? (
            <p className="text-sm text-danger" role="alert">
              Fix {Object.keys(errors).length} field{Object.keys(errors).length === 1 ? '' : 's'} before launching.
            </p>
          ) : null}

          <div className="flex flex-col gap-2">
            <Button type="submit" size="lg" disabled={submitting}>
              {submitting ? <Loader2 className="animate-spin" /> : <Play />}
              {submitting ? 'Submitting…' : 'Launch run'}
            </Button>
            <Button variant="ghost" onClick={reset} disabled={submitting || changed === 0}>
              <RotateCcw />
              Reset to defaults
            </Button>
          </div>
        </div>
      </aside>
    </form>
  )
}
