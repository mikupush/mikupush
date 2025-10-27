import { invoke } from '@tauri-apps/api/core'

export function openAboutWindow() {
  invoke('open_about_window').catch(console.error)
}