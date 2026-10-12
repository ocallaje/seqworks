import { cn } from '@/lib/utils'

export function LogoMark({ className }: { className?: string }) {
  return (
    <span
      className={cn('flex size-8 items-center justify-center rounded-md bg-primary text-primary-foreground', className)}
      aria-hidden
    >
      <svg viewBox="0 0 24 24" className="size-5" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
        <path d="M7 3c0 6 10 6 10 12s-10 3-10 6" />
        <path d="M17 3c0 6-10 6-10 12s10 3 10 6" opacity="0.55" />
        <path d="M9 7h6M9 17h6" />
      </svg>
    </span>
  )
}

export function Logo({ className }: { className?: string }) {
  return (
    <span className={cn('flex items-center gap-2.5', className)}>
      <LogoMark />
      <span className="flex flex-col leading-tight">
        <span className="text-sm font-semibold tracking-tight text-foreground">GeneLink</span>
        <span className="text-xs text-muted-foreground">SeqWorks Pipelines</span>
      </span>
    </span>
  )
}
