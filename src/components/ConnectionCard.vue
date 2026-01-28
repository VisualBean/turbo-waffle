<script setup lang="ts">
import { computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Connection, HealthResult } from '@/types/connection'
import HealthIndicator from './HealthIndicator.vue'
import { useToasts } from '@/composables/useToasts'

const props = defineProps<{
  connection: Connection
  health?: HealthResult
}>()

const emit = defineEmits<{
  edit: [connection: Connection]
  delete: [id: string]
}>()

const { pushToast } = useToasts()

const hasCustomIcon = computed(() => !!props.connection.icon)

const iconColor = computed(() => {
  if (props.connection.iconColor) return props.connection.iconColor
  switch (props.connection.config.type) {
    case 'website': return 'var(--icon-website)'
    case 'ssh': return 'var(--icon-ssh)'
  }
})

const defaultIconClass = computed(() => {
  switch (props.connection.config.type) {
    case 'website': return 'fa-solid fa-globe'
    case 'ssh': return 'fa-solid fa-terminal'
  }
})

const connectionInfo = computed(() => {
  const config = props.connection.config
  switch (config.type) {
    case 'website':
      return config.url
    case 'ssh':
      return `${config.username}@${config.host}:${config.port}`
  }
})

const canWake = computed(() => {
  if (props.connection.config.type !== 'ssh') return false
  return props.connection.config.wolEnabled
})

const healthStatus = computed(() => props.health?.status ?? 'unknown')
const latencyMs = computed(() => props.health?.latencyMs)

async function handleConnect() {
  try {
    await invoke('open_connection', { id: props.connection.id })
  } catch (err) {
    console.error('Failed to open connection:', err)
  }
}

async function handleWake() {
  try {
    await invoke('send_wol', { id: props.connection.id })
    pushToast('Wake-on-LAN packet sent.', 'success')
  } catch (err) {
    console.error('Failed to send WOL:', err)
    const message = err instanceof Error ? err.message : String(err)
    pushToast(`Wake failed: ${message}`, 'error')
  }
}

function handleEdit() {
  emit('edit', props.connection)
}

function handleDelete() {
  if (confirm(`Delete "${props.connection.name}"?`)) {
    emit('delete', props.connection.id)
  }
}
</script>

<template>
  <div class="card">
    <div class="card-header">
      <div class="type-icon" :style="{ color: iconColor }">
        <i :class="hasCustomIcon ? connection.icon : defaultIconClass"></i>
      </div>
      <div class="card-title-section">
        <h3 class="card-title">{{ connection.name }}</h3>
        <HealthIndicator :status="healthStatus" :latency-ms="latencyMs" />
      </div>
      <div class="card-menu">
        <button class="menu-btn" @click="handleEdit" title="Edit">
          <i class="fa-solid fa-pen"></i>
        </button>
        <button class="menu-btn delete" @click="handleDelete" title="Delete">
          <i class="fa-solid fa-trash"></i>
        </button>
      </div>
    </div>
    <div class="card-body">
      <p class="connection-info">{{ connectionInfo }}</p>
    </div>
    <div class="card-actions">
      <button class="action-btn primary" @click="handleConnect">
        Connect
      </button>
      <button v-if="canWake" class="action-btn secondary" @click="handleWake">
        Wake
      </button>
    </div>
  </div>
</template>

<style scoped>
.card {
  background-color: var(--bg-card);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color);
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  transition: border-color 0.2s, box-shadow 0.2s;
}

.card:hover {
  border-color: var(--accent);
  box-shadow: var(--shadow);
}

.card-header {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
}

.type-icon {
  flex-shrink: 0;
  width: 2.5rem;
  height: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
  border-radius: var(--radius);
  font-size: 1.25rem;
}

.card-title-section {
  flex: 1;
  min-width: 0;
}

.card-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-menu {
  display: flex;
  gap: 0.25rem;
}

.menu-btn {
  width: 1.75rem;
  height: 1.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: var(--radius);
  color: var(--text-muted);
  cursor: pointer;
  transition: color 0.2s, background-color 0.2s;
  font-size: 0.875rem;
}

.menu-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-primary);
}

.menu-btn.delete:hover {
  color: var(--status-offline);
}

.card-body {
  flex: 1;
}

.connection-info {
  font-size: 0.875rem;
  color: var(--text-secondary);
  word-break: break-all;
}

.card-actions {
  display: flex;
  gap: 0.5rem;
  padding-top: 0.5rem;
  border-top: 1px solid var(--border-color);
}

.action-btn {
  flex: 1;
  padding: 0.5rem;
  border: none;
  border-radius: var(--radius);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.action-btn.primary {
  background-color: var(--accent);
  color: white;
}

.action-btn.primary:hover {
  background-color: var(--accent-hover);
}

.action-btn.secondary {
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.action-btn.secondary:hover {
  background-color: var(--bg-card-hover);
}
</style>
