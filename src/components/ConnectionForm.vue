<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Connection, ConnectionType, ConnectionConfig } from '@/types/connection'
import IconPicker from './IconPicker.vue'

const props = defineProps<{
  connection: Connection | null
}>()

const emit = defineEmits<{
  save: [connection: Connection]
  cancel: []
}>()

const connectionType = ref<ConnectionType>('website')
const name = ref('')
const selectedIcon = ref('')
const selectedColor = ref('')

const websiteUrl = ref('')
const websiteCheckPath = ref('')

const sshHost = ref('')
const sshPort = ref(22)
const sshUsername = ref('')
const sshWolEnabled = ref(false)
const sshMacAddress = ref('')
const sshBroadcastAddr = ref('')
const sshMacLookupStatus = ref<'idle' | 'loading' | 'success' | 'error'>('idle')

watch(
  () => props.connection,
  (conn) => {
    if (conn) {
      name.value = conn.name
      selectedIcon.value = conn.icon || ''
      selectedColor.value = conn.iconColor || ''
      connectionType.value = conn.config.type

      if (conn.config.type === 'website') {
        websiteUrl.value = conn.config.url
        websiteCheckPath.value = conn.config.checkPath || ''
      } else if (conn.config.type === 'ssh') {
        sshHost.value = conn.config.host
        sshPort.value = conn.config.port
        sshUsername.value = conn.config.username
        sshWolEnabled.value = conn.config.wolEnabled
        sshMacAddress.value = conn.config.macAddress || ''
        sshBroadcastAddr.value = conn.config.broadcastAddr || ''
      }
    } else {
      resetForm()
    }
  },
  { immediate: true }
)

watch(sshWolEnabled, async (enabled) => {
  if (enabled && !sshMacAddress.value && sshHost.value) {
    await lookupMacAddress()
  }
})

async function lookupMacAddress() {
  if (!sshHost.value) return

  sshMacLookupStatus.value = 'loading'
  try {
    const mac = await invoke<string>('lookup_mac', { host: sshHost.value })
    sshMacAddress.value = mac
    sshMacLookupStatus.value = 'success'
  } catch {
    sshMacLookupStatus.value = 'error'
  }
}

function resetForm() {
  connectionType.value = 'website'
  name.value = ''
  selectedIcon.value = ''
  selectedColor.value = ''
  websiteUrl.value = ''
  websiteCheckPath.value = ''
  sshHost.value = ''
  sshPort.value = 22
  sshUsername.value = ''
  sshWolEnabled.value = false
  sshMacAddress.value = ''
  sshBroadcastAddr.value = ''
  sshMacLookupStatus.value = 'idle'
}

const defaultIcon = computed(() => {
  switch (connectionType.value) {
    case 'website': return 'fa-solid fa-globe'
    case 'ssh': return 'fa-solid fa-terminal'
  }
})

const isValid = computed(() => {
  if (!name.value.trim()) return false

  switch (connectionType.value) {
    case 'website':
      return !!websiteUrl.value.trim()
    case 'ssh': {
      const baseValid = !!sshHost.value.trim() && !!sshUsername.value.trim() && sshPort.value > 0
      if (sshWolEnabled.value) {
        return baseValid && !!sshMacAddress.value.trim()
      }
      return baseValid
    }
    default:
      return false
  }
})

function handleSubmit() {
  if (!isValid.value) return

  let config: ConnectionConfig

  switch (connectionType.value) {
    case 'website':
      config = {
        type: 'website',
        url: websiteUrl.value.trim(),
        checkPath: websiteCheckPath.value.trim() || undefined,
      }
      break
    case 'ssh':
      config = {
        type: 'ssh',
        host: sshHost.value.trim(),
        port: sshPort.value,
        username: sshUsername.value.trim(),
        wolEnabled: sshWolEnabled.value,
        macAddress: sshMacAddress.value.trim() || undefined,
        broadcastAddr: sshBroadcastAddr.value.trim() || undefined,
      }
      break
  }

  const now = new Date().toISOString()
  const connection: Connection = {
    id: props.connection?.id || crypto.randomUUID(),
    name: name.value.trim(),
    icon: selectedIcon.value || undefined,
    iconColor: selectedColor.value || undefined,
    order: props.connection?.order ?? 999999,
    config,
    createdAt: props.connection?.createdAt || now,
    updatedAt: now,
  }

  emit('save', connection)
}
</script>

<template>
  <div class="modal-overlay" @click.self="$emit('cancel')">
    <div class="modal">
      <div class="modal-header">
        <h2 class="modal-title">{{ connection ? 'Edit' : 'Add' }} Connection</h2>
        <button class="close-btn" @click="$emit('cancel')">
          <i class="fa-solid fa-xmark"></i>
        </button>
      </div>

      <form class="form" @submit.prevent="handleSubmit">
        <div class="form-group">
          <label class="label">Name</label>
          <input v-model="name" type="text" class="input" placeholder="My Connection" />
        </div>

        <div class="form-group">
          <label class="label">Type</label>
          <div class="type-buttons">
            <button
              type="button"
              class="type-btn"
              :class="{ active: connectionType === 'website' }"
              @click="connectionType = 'website'"
            >
              <i class="fa-solid fa-globe"></i>
              Website
            </button>
            <button
              type="button"
              class="type-btn"
              :class="{ active: connectionType === 'ssh' }"
              @click="connectionType = 'ssh'"
            >
              <i class="fa-solid fa-terminal"></i>
              SSH
            </button>
          </div>
        </div>

        <div class="form-group">
          <label class="label">Icon</label>
          <IconPicker
            v-model:icon="selectedIcon"
            v-model:color="selectedColor"
            :default-icon="defaultIcon"
          />
        </div>

        <template v-if="connectionType === 'website'">
          <div class="form-group">
            <label class="label">URL</label>
            <input v-model="websiteUrl" type="url" class="input" placeholder="https://example.com" />
          </div>
          <div class="form-group">
            <label class="label">Health Check Path (optional)</label>
            <input v-model="websiteCheckPath" type="text" class="input" placeholder="/health" />
          </div>
        </template>

        <template v-else-if="connectionType === 'ssh'">
          <div class="form-row">
            <div class="form-group flex-2">
              <label class="label">Host</label>
              <input v-model="sshHost" type="text" class="input" placeholder="192.168.1.100" />
            </div>
            <div class="form-group flex-1">
              <label class="label">Port</label>
              <input v-model.number="sshPort" type="number" class="input" min="1" max="65535" />
            </div>
          </div>
          <div class="form-group">
            <label class="label">Username</label>
            <input v-model="sshUsername" type="text" class="input" placeholder="root" />
          </div>

          <div class="form-group">
            <label class="checkbox-label">
              <input v-model="sshWolEnabled" type="checkbox" />
              Enable Wake-on-LAN
            </label>
          </div>

          <template v-if="sshWolEnabled">
            <div class="form-group">
              <label class="label">
                MAC Address
                <span class="required">*</span>
              </label>
              <div class="input-with-button">
                <input
                  v-model="sshMacAddress"
                  type="text"
                  class="input"
                  placeholder="AA:BB:CC:DD:EE:FF"
                  :class="{ error: sshWolEnabled && !sshMacAddress }"
                />
                <button
                  type="button"
                  class="lookup-btn"
                  :disabled="!sshHost || sshMacLookupStatus === 'loading'"
                  @click="lookupMacAddress"
                >
                  <span v-if="sshMacLookupStatus === 'loading'">...</span>
                  <span v-else>Lookup</span>
                </button>
              </div>
              <span v-if="sshMacLookupStatus === 'error'" class="hint error-text">
                Could not find MAC in ARP table. Try pinging the host first.
              </span>
              <span v-else-if="sshMacLookupStatus === 'success'" class="hint success-text">
                MAC address found!
              </span>
            </div>
            <div class="form-group">
              <label class="label">Broadcast Address (optional)</label>
              <input v-model="sshBroadcastAddr" type="text" class="input" placeholder="192.168.1.255" />
            </div>
          </template>
        </template>

        <div class="form-actions">
          <button type="button" class="btn secondary" @click="$emit('cancel')">Cancel</button>
          <button type="submit" class="btn primary" :disabled="!isValid">
            {{ connection ? 'Save' : 'Add' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
  z-index: 100;
}

.modal {
  background-color: var(--bg-card);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color);
  width: 100%;
  max-width: 520px;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-color);
}

.modal-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  width: 2rem;
  height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: var(--radius);
  color: var(--text-muted);
  cursor: pointer;
  font-size: 1.125rem;
}

.close-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-primary);
}

.form {
  padding: 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.form-row {
  display: flex;
  gap: 0.75rem;
}

.form-row .form-group {
  flex: 1;
}

.flex-1 { flex: 1; }
.flex-2 { flex: 2; }

.label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.required {
  color: var(--status-offline);
}

.input {
  padding: 0.625rem 0.75rem;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius);
  color: var(--text-primary);
  font-size: 0.875rem;
}

.input:focus {
  outline: none;
  border-color: var(--accent);
}

.input::placeholder {
  color: var(--text-muted);
}

.input.error {
  border-color: var(--status-offline);
}

.input-with-button {
  display: flex;
  gap: 0.5rem;
}

.input-with-button .input {
  flex: 1;
}

.lookup-btn {
  padding: 0.625rem 0.75rem;
  background-color: var(--bg-card-hover);
  border: 1px solid var(--border-color);
  border-radius: var(--radius);
  color: var(--text-primary);
  font-size: 0.875rem;
  cursor: pointer;
  white-space: nowrap;
}

.lookup-btn:hover:not(:disabled) {
  background-color: var(--bg-primary);
}

.lookup-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.hint {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.error-text {
  color: var(--status-offline);
}

.success-text {
  color: var(--status-online);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-primary);
  cursor: pointer;
}

.checkbox-label input {
  accent-color: var(--accent);
  width: 1rem;
  height: 1rem;
}

.type-buttons {
  display: flex;
  gap: 0.5rem;
}

.type-btn {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.375rem;
  padding: 0.75rem;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius);
  color: var(--text-secondary);
  font-size: 0.75rem;
  cursor: pointer;
  transition: border-color 0.2s, color 0.2s;
}

.type-btn i {
  font-size: 1.25rem;
}

.type-btn:hover {
  border-color: var(--text-muted);
}

.type-btn.active {
  border-color: var(--accent);
  color: var(--accent);
}

.radio-group {
  display: flex;
  gap: 1rem;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-primary);
  cursor: pointer;
}

.radio-label input {
  accent-color: var(--accent);
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding-top: 0.5rem;
}

.btn {
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: var(--radius);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.btn.primary {
  background-color: var(--accent);
  color: white;
}

.btn.primary:hover:not(:disabled) {
  background-color: var(--accent-hover);
}

.btn.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn.secondary {
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn.secondary:hover {
  background-color: var(--bg-card-hover);
}
</style>
