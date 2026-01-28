<script setup lang="ts">
import AppHeader from './components/AppHeader.vue'
import ConnectionGrid from './components/ConnectionGrid.vue'
import ConnectionForm from './components/ConnectionForm.vue'
import ToastStack from './components/ToastStack.vue'
import { useConnections } from './composables/useConnections'
import { useHealth } from './composables/useHealth'
import { ref, onMounted, onUnmounted } from 'vue'
import type { Connection } from './types/connection'

const { connections, loadConnections, saveConnection, deleteConnection, reorderConnections } = useConnections()
const { healthStatuses, checkAllHealth, startPolling, stopPolling } = useHealth()

const showForm = ref(false)
const editingConnection = ref<Connection | null>(null)

onMounted(async () => {
  await loadConnections()
  await checkAllHealth(connections.value)
  startPolling(connections.value)
})

onUnmounted(() => {
  stopPolling()
})

function handleAdd() {
  editingConnection.value = null
  showForm.value = true
}

function handleEdit(connection: Connection) {
  editingConnection.value = connection
  showForm.value = true
}

async function handleSave(connection: Connection) {
  await saveConnection(connection)
  showForm.value = false
  editingConnection.value = null
  await checkAllHealth(connections.value)
}

async function handleDelete(id: string) {
  await deleteConnection(id)
}

async function handleReorder(ids: string[]) {
  await reorderConnections(ids)
}

function handleCancel() {
  showForm.value = false
  editingConnection.value = null
}
</script>

<template>
  <div class="app">
    <AppHeader @add="handleAdd" />
    <main class="main">
      <ConnectionGrid
        :connections="connections"
        :health-statuses="healthStatuses"
        @edit="handleEdit"
        @delete="handleDelete"
        @reorder="handleReorder"
      />
    </main>
    <ConnectionForm
      v-if="showForm"
      :connection="editingConnection"
      @save="handleSave"
      @cancel="handleCancel"
    />
    <ToastStack />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  min-height: 100vh;
}

.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.main {
  flex: 1;
  padding: 1.5rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}
</style>
