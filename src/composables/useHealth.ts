import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Connection, HealthResult } from '@/types/connection'

const healthStatuses = ref<Map<string, HealthResult>>(new Map())
let pollingInterval: ReturnType<typeof setInterval> | null = null

export function useHealth() {
  async function checkHealth(connection: Connection): Promise<HealthResult> {
    try {
      const result = await invoke<HealthResult>('check_health', { id: connection.id })
      healthStatuses.value.set(connection.id, result)
      return result
    } catch (err) {
      const result: HealthResult = {
        connectionId: connection.id,
        status: 'unknown',
        error: String(err),
        checkedAt: new Date().toISOString(),
      }
      healthStatuses.value.set(connection.id, result)
      return result
    }
  }

  async function checkAllHealth(connections: Connection[]) {
    const results = await Promise.all(connections.map((c) => checkHealth(c)))
    return results
  }

  function startPolling(connections: Connection[], intervalMs = 30000) {
    stopPolling()
    pollingInterval = setInterval(() => {
      checkAllHealth(connections)
    }, intervalMs)
  }

  function stopPolling() {
    if (pollingInterval) {
      clearInterval(pollingInterval)
      pollingInterval = null
    }
  }

  return {
    healthStatuses,
    checkHealth,
    checkAllHealth,
    startPolling,
    stopPolling,
  }
}
