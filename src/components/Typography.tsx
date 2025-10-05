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
import { JSX } from 'react/jsx-runtime'
import * as React from 'react'

export interface TypographyProps {
  children: string | string[]
  className?: string
  as?: keyof JSX.IntrinsicElements
}

export function Heading1(props: TypographyProps) {
  const classes = cn(
    'scroll-m-20 text-4xl tracking-tight text-balance font-extrabold text-center',
    props.className
  )

  const Component = props.as ?? 'h1'
  return React.createElement(Component, { className: classes }, props.children)
}

export function Heading2(props: TypographyProps) {
  const classes = cn(
    'scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0',
    props.className
  )

  const Component = props.as ?? 'h2'
  return React.createElement(Component, { className: classes }, props.children)
}

export function Heading3(props: TypographyProps) {
  const classes = cn(
    'scroll-m-20 text-2xl font-semibold tracking-tight',
    props.className
  )

  const Component = props.as ?? 'h3'
  return React.createElement(Component, { className: classes }, props.children)
}

export function Heading4(props: TypographyProps) {
  const classes = cn(
    'scroll-m-20 text-xl font-semibold tracking-tight',
    props.className
  )

  const Component = props.as ?? 'h4'
  return React.createElement(Component, { className: classes }, props.children)
}

export function Small(props: TypographyProps) {
  const classes = cn(
    'text-sm leading-none font-medium',
    props.className
  )

  const Component = props.as ?? 'small'
  return React.createElement(Component, { className: classes }, props.children)
}

export function Paragraph(props: TypographyProps) {
  const classes = cn(
    'leading-7 [&:not(:first-child)]:mt-6',
    props.className
  )

  const Component = props.as ?? 'p'
  return React.createElement(Component, { className: classes }, props.children)
}

export function Large(props: TypographyProps) {
  const classes = cn(
    'text-lg font-semibold',
    props.className
  )

  const Component = props.as ?? 'div'
  return React.createElement(Component, { className: classes }, props.children)
}