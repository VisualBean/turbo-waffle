<script setup lang="ts">
import { ref } from 'vue'
import type { Connection, HealthResult } from '@/types/connection'
import ConnectionCard from './ConnectionCard.vue'

const props = defineProps<{
  connections: Connection[]
  healthStatuses: Map<string, HealthResult>
}>()

const emit = defineEmits<{
  edit: [connection: Connection]
  delete: [id: string]
  reorder: [ids: string[]]
}>()

const draggedId = ref<string | null>(null)
const dragOverId = ref<string | null>(null)

function handleDragStart(e: DragEvent, id: string) {
  draggedId.value = id
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move'
    e.dataTransfer.setData('text/plain', id)
  }
}

function handleDragOver(e: DragEvent, id: string) {
  e.preventDefault()
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = 'move'
  }
  dragOverId.value = id
}

function handleDragLeave() {
  dragOverId.value = null
}

function handleDrop(e: DragEvent, targetId: string) {
  e.preventDefault()
  dragOverId.value = null

  if (!draggedId.value || draggedId.value === targetId) {
    draggedId.value = null
    return
  }

  const ids = props.connections.map(c => c.id)
  const draggedIndex = ids.indexOf(draggedId.value)
  const targetIndex = ids.indexOf(targetId)

  if (draggedIndex === -1 || targetIndex === -1) {
    draggedId.value = null
    return
  }

  ids.splice(draggedIndex, 1)
  ids.splice(targetIndex, 0, draggedId.value)

  emit('reorder', ids)
  draggedId.value = null
}

function handleDragEnd() {
  draggedId.value = null
  dragOverId.value = null
}
</script>

<template>
  <div v-if="connections.length === 0" class="empty-state">
    <svg class="empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
      <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
      <line x1="9" y1="9" x2="15" y2="15" />
      <line x1="15" y1="9" x2="9" y2="15" />
    </svg>
    <h2 class="empty-title">No connections yet</h2>
    <p class="empty-text">Add your first connection to get started</p>
  </div>
  <div v-else class="grid">
    <div
      v-for="connection in connections"
      :key="connection.id"
      class="card-wrapper"
      :class="{
        dragging: draggedId === connection.id,
        'drag-over': dragOverId === connection.id && draggedId !== connection.id
      }"
      draggable="true"
      @dragstart="handleDragStart($event, connection.id)"
      @dragover="handleDragOver($event, connection.id)"
      @dragleave="handleDragLeave"
      @drop="handleDrop($event, connection.id)"
      @dragend="handleDragEnd"
    >
      <ConnectionCard
        :connection="connection"
        :health="healthStatuses.get(connection.id)"
        @edit="emit('edit', $event)"
        @delete="emit('delete', $event)"
      />
    </div>
  </div>
</template>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}

.card-wrapper {
  transition: transform 0.2s, opacity 0.2s;
  cursor: grab;
}

.card-wrapper:active {
  cursor: grabbing;
}

.card-wrapper.dragging {
  opacity: 0.5;
  transform: scale(0.98);
}

.card-wrapper.drag-over {
  transform: scale(1.02);
}

.card-wrapper.drag-over::before {
  content: '';
  position: absolute;
  inset: -4px;
  border: 2px dashed var(--accent);
  border-radius: var(--radius-lg);
  pointer-events: none;
}

.card-wrapper {
  position: relative;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.empty-icon {
  width: 4rem;
  height: 4rem;
  color: var(--text-muted);
  margin-bottom: 1rem;
}

.empty-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.empty-text {
  color: var(--text-secondary);
}
</style>
