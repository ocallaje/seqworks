export interface Option {
  value: string
  label: string
}

interface BaseField {
  key: string
  label: string
  help?: string
  required?: boolean
}

export type Field =
  | (BaseField & { type: 'text'; default?: string; placeholder?: string; mono?: boolean })
  | (BaseField & { type: 'number'; default: number; min?: number; max?: number; step?: number; unit?: string })
  | (BaseField & { type: 'select'; default: string; options: Option[] })
  | (BaseField & { type: 'segmented'; default: string; options: Option[] })
  | (BaseField & { type: 'toggle'; default: boolean })
  | (BaseField & { type: 'multi'; default: string[]; options: Option[] })

export type FieldValue = string | number | boolean | string[]

export interface Section {
  title: string
  description?: string
  fields: Field[]
}

export interface PipelineDefinition {
  id: string
  name: string
  workflow: string
  version: string
  description: string
  steps: string[]
  sections: Section[]
}

export function defaultValues(pipeline: PipelineDefinition): Record<string, FieldValue> {
  const values: Record<string, FieldValue> = {}
  for (const section of pipeline.sections) {
    for (const field of section.fields) {
      values[field.key] = field.type === 'text' ? (field.default ?? '') : field.default
    }
  }
  return values
}

export function validate(pipeline: PipelineDefinition, values: Record<string, FieldValue>): Record<string, string> {
  const errors: Record<string, string> = {}
  for (const section of pipeline.sections) {
    for (const field of section.fields) {
      const value = values[field.key]
      if (field.type === 'text' && field.required && !String(value ?? '').trim()) {
        errors[field.key] = `${field.label} is required`
      }
      if (field.type === 'number') {
        const n = Number(value)
        if (Number.isNaN(n)) errors[field.key] = 'Enter a number'
        else if (field.min !== undefined && n < field.min) errors[field.key] = `Must be at least ${field.min}`
        else if (field.max !== undefined && n > field.max) errors[field.key] = `Must be at most ${field.max}`
      }
      if (field.type === 'multi' && field.required && (value as string[]).length === 0) {
        errors[field.key] = 'Select at least one option'
      }
    }
  }
  return errors
}
