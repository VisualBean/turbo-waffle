<script setup lang="ts">
import { useSettings } from '@/composables/useSettings'

const { theme, toggleTheme } = useSettings()

defineEmits<{
  add: []
}>()
</script>

<template>
  <header class="header">
    <div class="header-content">
      <div class="logo">
        <img src="/icon-128.png" alt="Turbo Waffle" class="logo-icon" />
        <h1 class="title">Turbo Waffle</h1>
      </div>
      <div class="header-actions">
        <button class="theme-toggle" @click="toggleTheme" :title="theme === 'dark' ? 'Switch to Waffle theme' : 'Switch to Dark theme'">
          <i v-if="theme === 'dark'" class="fa-solid fa-sun"></i>
          <i v-else class="fa-solid fa-moon"></i>
        </button>
        <button class="add-button" @click="$emit('add')">
          <i class="fa-solid fa-plus"></i>
          Add Connection
        </button>
      </div>
    </div>
  </header>
</template>

<style scoped>
.header {
  background-color: var(--bg-card);
  border-bottom: 1px solid var(--border-color);
  padding: 1rem 1.5rem;
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-icon {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: var(--radius);
}

.title {
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.theme-toggle {
  width: 2.5rem;
  height: 2.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius);
  color: var(--text-secondary);
  font-size: 1.125rem;
  cursor: pointer;
  transition: color 0.2s, border-color 0.2s;
}

.theme-toggle:hover {
  color: var(--accent);
  border-color: var(--accent);
}

.add-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.625rem 1rem;
  background-color: var(--accent);
  color: white;
  border: none;
  border-radius: var(--radius);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.add-button:hover {
  background-color: var(--accent-hover);
}
</style>
