import * as React from 'react'
import { SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar.tsx'
import { NavLink, useMatch } from 'react-router'

interface SidebarLinkMenuItemProps {
  to: string
  children: React.ReactNode
}

export default function SidebarLinkMenuItem({ to, children }: SidebarLinkMenuItemProps) {
  const match = useMatch(to)

  return (
    <SidebarMenuItem>
      <SidebarMenuButton asChild isActive={match !== null}>
        <NavLink to={to}>
          {children}
        </NavLink>
      </SidebarMenuButton>
    </SidebarMenuItem>
  )
}