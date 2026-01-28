<script setup lang="ts">
import type { HealthStatus } from '@/types/connection'

const props = defineProps<{
  status: HealthStatus
  latencyMs?: number
}>()

const statusIcon: Record<HealthStatus, string> = {
  online: '🟢',
  offline: '🔴',
  degraded: '🟡',
  unknown: '⚪'
}
</script>

<template>
  <div class="health-indicator" :class="status">
    <span class="dot">{{ statusIcon[props.status] }}</span>
    <span class="label">
      {{ status }}
      <span v-if="latencyMs !== undefined && status === 'online'" class="latency">
        ({{ latencyMs }}ms)
      </span>
    </span>
  </div>
</template>

<style scoped>
.health-indicator {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.75rem;
  text-transform: capitalize;
}

.dot {
  font-size: 0.85rem;
  line-height: 1;
}

.label {
  color: var(--text-secondary);
}

.latency {
  color: var(--text-muted);
}
</style>
