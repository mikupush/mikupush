import { invoke } from '@tauri-apps/api/core'
import { ConfigKey } from '@/constants/config.ts'

export async function applyConfig(key: ConfigKey, value: string) {
  await invoke('set_config_value', { key, value })
}

export async function getConfig(key: ConfigKey): Promise<string> {
  return await invoke<string>('get_config_value', { key })
}
