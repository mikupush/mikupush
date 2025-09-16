/**
 * Copyright 2025 Miku Push! Team
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
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
