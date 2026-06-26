import { ReactNode } from 'react'
import { Heading2 } from '@/components/Typography.tsx'

interface PageHeadingProps {
  backAction?: ReactNode;
  title: string;
}

export function PageHeading({ backAction, title }: PageHeadingProps) {
  return (
    <div className="flex border-b mb-6">
      {backAction}
      <Heading2 className={`border-none ${backAction ? 'ml-1' : ''}`}>{title}</Heading2>
    </div>
  )
}