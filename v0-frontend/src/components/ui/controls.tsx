import { forwardRef, useId, type InputHTMLAttributes, type SelectHTMLAttributes } from 'react'
import { Check, ChevronDown } from 'lucide-react'
import { cn } from '@/lib/utils'
import type { Option } from '@/pipelines/types'

const fieldBase =
  'w-full rounded-md border border-input bg-card px-3 text-sm text-foreground shadow-xs transition-colors placeholder:text-muted-foreground/70 focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/20 focus-visible:outline-none aria-invalid:border-danger aria-invalid:ring-danger/20 disabled:opacity-50'

export const Input = forwardRef<HTMLInputElement, InputHTMLAttributes<HTMLInputElement>>(function Input(
  { className, ...props },
  ref,
) {
  return <input ref={ref} className={cn(fieldBase, 'h-9', className)} {...props} />
})

export const Select = forwardRef<HTMLSelectElement, SelectHTMLAttributes<HTMLSelectElement> & { options: Option[] }>(
  function Select({ className, options, ...props }, ref) {
    return (
      <div className="relative">
        <select ref={ref} className={cn(fieldBase, 'h-9 cursor-pointer appearance-none pr-9', className)} {...props}>
          {options.map((o) => (
            <option key={o.value} value={o.value}>
              {o.label}
            </option>
          ))}
        </select>
        <ChevronDown className="pointer-events-none absolute top-1/2 right-3 size-4 -translate-y-1/2 text-muted-foreground" aria-hidden />
      </div>
    )
  },
)

interface SegmentedProps {
  value: string
  options: Option[]
  onChange: (value: string) => void
  labelledBy: string
  name: string
}

export function Segmented({ value, options, onChange, labelledBy, name }: SegmentedProps) {
  return (
    <div role="radiogroup" aria-labelledby={labelledBy} className="flex flex-wrap gap-1 rounded-lg border bg-muted/60 p-1">
      {options.map((o) => {
        const selected = o.value === value
        return (
          <label
            key={o.value}
            className={cn(
              'relative flex h-8 flex-1 cursor-pointer items-center justify-center rounded-md px-3 text-sm font-medium whitespace-nowrap transition-colors has-[:focus-visible]:outline-2 has-[:focus-visible]:outline-ring',
              selected ? 'bg-card text-foreground shadow-sm ring-1 ring-border dark:bg-input dark:ring-input' : 'text-muted-foreground hover:text-foreground',
            )}
          >
            <input
              type="radio"
              name={name}
              value={o.value}
              checked={selected}
              onChange={() => onChange(o.value)}
              className="sr-only"
            />
            {o.label}
          </label>
        )
      })}
    </div>
  )
}

interface SwitchProps {
  checked: boolean
  onChange: (checked: boolean) => void
  label: string
  description?: string
}

export function Switch({ checked, onChange, label, description }: SwitchProps) {
  const id = useId()
  return (
    <div className="flex items-start justify-between gap-6 rounded-lg border bg-card px-4 py-3">
      <div className="flex flex-col gap-0.5">
        <label htmlFor={id} className="cursor-pointer text-sm font-medium text-foreground">
          {label}
        </label>
        {description ? <p className="text-sm leading-relaxed text-muted-foreground">{description}</p> : null}
      </div>
      <button
        id={id}
        type="button"
        role="switch"
        aria-checked={checked}
        onClick={() => onChange(!checked)}
        className={cn(
          'relative mt-0.5 inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full transition-colors',
          checked ? 'bg-primary' : 'bg-input',
        )}
      >
        <span
          className={cn(
            'inline-block size-4 rounded-full bg-card shadow-sm transition-transform',
            checked ? 'translate-x-4.5' : 'translate-x-0.5',
          )}
        />
      </button>
    </div>
  )
}

interface ChipGroupProps {
  value: string[]
  options: Option[]
  onChange: (value: string[]) => void
  labelledBy: string
}

export function ChipGroup({ value, options, onChange, labelledBy }: ChipGroupProps) {
  return (
    <div role="group" aria-labelledby={labelledBy} className="flex flex-wrap gap-2">
      {options.map((o) => {
        const selected = value.includes(o.value)
        return (
          <label
            key={o.value}
            className={cn(
              'inline-flex h-8 cursor-pointer items-center gap-1.5 rounded-full border px-3 text-sm font-medium transition-colors has-[:focus-visible]:outline-2 has-[:focus-visible]:outline-ring',
              selected
                ? 'border-primary/40 bg-primary/10 text-primary'
                : 'border-input bg-card text-muted-foreground hover:text-foreground',
            )}
          >
            <input
              type="checkbox"
              className="sr-only"
              checked={selected}
              onChange={() => onChange(selected ? value.filter((v) => v !== o.value) : [...value, o.value])}
            />
            {selected ? <Check className="size-3.5" aria-hidden /> : null}
            {o.label}
          </label>
        )
      })}
    </div>
  )
}
