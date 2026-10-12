import type { Field, FieldValue } from '@/pipelines/types'
import { ChipGroup, Input, Segmented, Select, Switch } from '@/components/ui/controls'
import { cn } from '@/lib/utils'

interface FieldInputProps {
  field: Field
  value: FieldValue
  error?: string
  onChange: (value: FieldValue) => void
}

export function FieldInput({ field, value, error, onChange }: FieldInputProps) {
  const id = `field-${field.key}`
  const labelId = `${id}-label`
  const helpId = field.help ? `${id}-help` : undefined
  const errorId = error ? `${id}-error` : undefined
  const describedBy = [helpId, errorId].filter(Boolean).join(' ') || undefined

  if (field.type === 'toggle') {
    return <Switch checked={Boolean(value)} onChange={onChange} label={field.label} description={field.help} />
  }

  const isWide = field.type === 'text' || field.type === 'multi' || field.type === 'segmented'

  return (
    <div className={cn('flex flex-col gap-2', isWide && 'md:col-span-2')}>
      <div className="flex items-baseline justify-between gap-2">
        <label id={labelId} htmlFor={id} className="text-sm font-medium text-foreground">
          {field.label}
          {field.required ? (
            <span className="text-danger" aria-hidden>
              {' *'}
            </span>
          ) : null}
        </label>
        <span className="font-mono text-xs text-muted-foreground">--{field.key}</span>
      </div>

      {field.type === 'text' ? (
        <Input
          id={id}
          value={String(value ?? '')}
          placeholder={field.placeholder}
          onChange={(e) => onChange(e.target.value)}
          aria-invalid={Boolean(error)}
          aria-describedby={describedBy}
          aria-required={field.required}
          spellCheck={false}
          autoComplete="off"
          className={cn(field.mono && 'font-mono text-[13px]')}
        />
      ) : null}

      {field.type === 'number' ? (
        <div className="relative">
          <Input
            id={id}
            type="number"
            inputMode="decimal"
            value={String(value)}
            min={field.min}
            max={field.max}
            step={field.step}
            onChange={(e) => onChange(e.target.value === '' ? '' : Number(e.target.value))}
            aria-invalid={Boolean(error)}
            aria-describedby={describedBy}
            className={cn('font-mono tabular-nums', field.unit && 'pr-14')}
          />
          {field.unit ? (
            <span className="pointer-events-none absolute top-1/2 right-3 -translate-y-1/2 text-xs text-muted-foreground">
              {field.unit}
            </span>
          ) : null}
        </div>
      ) : null}

      {field.type === 'select' ? (
        <Select
          id={id}
          value={String(value)}
          options={field.options}
          onChange={(e) => onChange(e.target.value)}
          aria-describedby={describedBy}
        />
      ) : null}

      {field.type === 'segmented' ? (
        <Segmented
          name={id}
          labelledBy={labelId}
          value={String(value)}
          options={field.options}
          onChange={onChange}
        />
      ) : null}

      {field.type === 'multi' ? (
        <ChipGroup labelledBy={labelId} value={value as string[]} options={field.options} onChange={onChange} />
      ) : null}

      {field.help ? (
        <p id={helpId} className="text-sm leading-relaxed text-muted-foreground">
          {field.help}
        </p>
      ) : null}
      {error ? (
        <p id={errorId} className="text-sm text-danger">
          {error}
        </p>
      ) : null}
    </div>
  )
}
