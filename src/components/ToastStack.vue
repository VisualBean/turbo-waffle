<script setup lang="ts">
import { useToasts } from '@/composables/useToasts'

const { toasts, removeToast } = useToasts()
</script>

<template>
  <div class="toast-stack" role="status" aria-live="polite">
    <div v-for="toast in toasts" :key="toast.id" class="toast" :class="toast.type">
      <span class="message">{{ toast.message }}</span>
      <button class="close" @click="removeToast(toast.id)" aria-label="Dismiss notification">
        ×
      </button>
    </div>
  </div>
</template>

<style scoped>
.toast-stack {
  position: fixed;
  right: 1rem;
  bottom: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  z-index: 1000;
}

.toast {
  min-width: 240px;
  max-width: 360px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  padding: 0.75rem 0.875rem;
  border-radius: var(--radius);
  background-color: var(--bg-card);
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow);
  color: var(--text-primary);
  font-size: 0.875rem;
}

.toast.success {
  border-color: var(--status-online);
}

.toast.error {
  border-color: var(--status-offline);
}

.toast.info {
  border-color: var(--accent);
}

.message {
  flex: 1;
}

.close {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  font-size: 1.125rem;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.close:hover {
  color: var(--text-primary);
}
</style>
