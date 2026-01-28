import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Connection } from '@/types/connection'

const connections = ref<Connection[]>([])

const sortedConnections = computed(() => {
  return [...connections.value].sort((a, b) => a.order - b.order)
})

export function useConnections() {
  async function loadConnections() {
    try {
      connections.value = await invoke<Connection[]>('get_connections')
    } catch (err) {
      console.error('Failed to load connections:', err)
      connections.value = []
    }
  }

  async function saveConnection(connection: Connection) {
    try {
      await invoke('save_connection', { connection })
      await loadConnections()
    } catch (err) {
      console.error('Failed to save connection:', err)
      throw err
    }
  }

  async function deleteConnection(id: string) {
    try {
      await invoke('delete_connection', { id })
      await loadConnections()
    } catch (err) {
      console.error('Failed to delete connection:', err)
      throw err
    }
  }

  async function reorderConnections(ids: string[]) {
    try {
      await invoke('reorder_connections', { ids })
      await loadConnections()
    } catch (err) {
      console.error('Failed to reorder connections:', err)
      throw err
    }
  }

  return {
    connections: sortedConnections,
    loadConnections,
    saveConnection,
    deleteConnection,
    reorderConnections,
  }
}
