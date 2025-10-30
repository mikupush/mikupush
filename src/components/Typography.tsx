/**
 * Miku Push! is a simple, lightweight, and open-source WeTransfer alternative for desktop.
 * Copyright (C) 2025  Miku Push! Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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