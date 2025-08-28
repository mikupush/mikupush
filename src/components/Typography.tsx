import { cn } from '@/lib/utils'

export type FontWeight = 'normal'
  | 'medium'
  | 'semibold'
  | 'bold'

export type TextAlign = 'left' | 'right' | 'center' | 'justify'

export interface TypographyProps {
  children: string | string[]
  weight?: FontWeight
  align?: TextAlign
  className?: string
}

export const fontWeightClass = (weight?: FontWeight) => `font-${weight ?? 'normal'}`
export const textAlignClass = (align?: TextAlign) => `text-${align ?? 'left'}`

export const withDefaultClasses = (props: TypographyProps, ...classes: string[]) => cn(
  fontWeightClass(props.weight),
  textAlignClass(props.align),
  'whitespace-pre-line',
  ...classes,
  ...(props.className ?? '').split(' ')
)

export function Small(props: TypographyProps) {
  const classes = withDefaultClasses(
    props,
    'text-sm',
    'leading-none',
  )

  return (
    <small className={classes}>{props.children}</small>
  )
}

export function Paragraph(props: TypographyProps) {
  const classes = withDefaultClasses(
    props,
    'leading-7',
    '[&:not(:first-child)]:mt-6',
  )

  return (
    <p className={classes}>{props.children}</p>
  )
}

export function Large(props: Omit<TypographyProps, 'weight'>) {
  const classes = withDefaultClasses(
    props,
    'text-lg',
    'font-semibold'
  )

  return (
    <div className={classes}>{props.children}</div>
  )
}
