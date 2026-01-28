<script setup lang="ts">
import { ref, computed } from 'vue'
import { BRAND_ICONS, ICON_COLORS } from '@/types/connection'

const props = defineProps<{
  icon: string
  color: string
  defaultIcon: string
}>()

const emit = defineEmits<{
  'update:icon': [value: string]
  'update:color': [value: string]
}>()

const isOpen = ref(false)

const displayIcon = computed(() => props.icon || props.defaultIcon)
const displayColor = computed(() => props.color || 'var(--text-secondary)')

function selectIcon(iconClass: string) {
  emit('update:icon', iconClass)
}

function selectColor(color: string) {
  emit('update:color', props.color === color ? '' : color)
}

function close() {
  isOpen.value = false
}
</script>

<template>
  <div class="icon-picker">
    <button type="button" class="picker-trigger" @click="isOpen = true">
      <span class="preview-icon" :style="{ color: displayColor }">
        <i :class="displayIcon"></i>
      </span>
      <span class="picker-label">
        {{ icon ? 'Custom Icon' : 'Default Icon' }}
        <span class="picker-hint">Click to change</span>
      </span>
      <i class="fa-solid fa-chevron-right"></i>
    </button>

    <Teleport to="body">
      <div v-if="isOpen" class="picker-overlay" @click.self="close">
        <div class="picker-modal">
          <div class="picker-header">
            <h3 class="picker-title">Choose Icon</h3>
            <button type="button" class="close-btn" @click="close">
              <i class="fa-solid fa-xmark"></i>
            </button>
          </div>

          <div class="picker-body">
            <div class="picker-section">
              <label class="section-label">Icon</label>
              <div class="icon-grid">
                <button
                  v-for="item in BRAND_ICONS"
                  :key="item.class"
                  type="button"
                  class="icon-btn"
                  :class="{ active: icon === item.class }"
                  :title="item.name"
                  @click="selectIcon(item.class)"
                >
                  <i v-if="item.class" :class="item.class"></i>
                  <span v-else class="default-icon">
                    <i :class="defaultIcon"></i>
                  </span>
                </button>
              </div>
            </div>

            <div class="picker-section">
              <label class="section-label">Color</label>
              <div class="color-grid">
                <button
                  type="button"
                  class="color-btn default-color"
                  :class="{ active: !color }"
                  title="Default"
                  @click="selectColor('')"
                >
                  <i class="fa-solid fa-rotate-left"></i>
                </button>
                <button
                  v-for="c in ICON_COLORS"
                  :key="c"
                  type="button"
                  class="color-btn"
                  :class="{ active: color === c }"
                  :style="{ backgroundColor: c }"
                  @click="selectColor(c)"
                ></button>
              </div>
            </div>

            <div class="preview-section">
              <label class="section-label">Preview</label>
              <div class="preview-box">
                <span class="preview-large" :style="{ color: displayColor }">
                  <i :class="displayIcon"></i>
                </span>
              </div>
            </div>
          </div>

          <div class="picker-footer">
            <button type="button" class="btn primary" @click="close">Done</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.icon-picker {
  width: 100%;
}

.picker-trigger {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 0.75rem;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius);
  cursor: pointer;
  transition: border-color 0.2s;
}

.picker-trigger:hover {
  border-color: var(--accent);
}

.preview-icon {
  width: 2.25rem;
  height: 2.25rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-card);
  border-radius: var(--radius);
  font-size: 1.25rem;
}

.picker-label {
  flex: 1;
  text-align: left;
  font-size: 0.875rem;
  color: var(--text-primary);
}

.picker-hint {
  display: block;
  font-size: 0.75rem;
  color: var(--text-muted);
}

.picker-trigger > i {
  color: var(--text-muted);
  font-size: 0.75rem;
}

.picker-overlay {
  position: fixed;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
  z-index: 200;
}

.picker-modal {
  background-color: var(--bg-card);
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color);
  width: 100%;
  max-width: 400px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.picker-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-color);
}

.picker-title {
  font-size: 1rem;
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
  font-size: 1rem;
}

.close-btn:hover {
  color: var(--text-primary);
  background-color: var(--bg-primary);
}

.picker-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.picker-section {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.section-label {
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
}

.icon-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(2.5rem, 1fr));
  gap: 0.375rem;
}

.icon-btn {
  aspect-ratio: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius);
  color: var(--text-secondary);
  font-size: 1.125rem;
  cursor: pointer;
  transition: border-color 0.2s, color 0.2s, background-color 0.2s;
}

.icon-btn:hover {
  border-color: var(--text-muted);
  color: var(--text-primary);
}

.icon-btn.active {
  border-color: var(--accent);
  color: var(--accent);
  background-color: var(--bg-card-hover);
}

.icon-btn .default-icon {
  opacity: 0.5;
}

.color-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.color-btn {
  width: 2rem;
  height: 2rem;
  border: 2px solid var(--border-color);
  border-radius: var(--radius);
  cursor: pointer;
  transition: transform 0.2s, border-color 0.2s;
}

.color-btn:hover {
  transform: scale(1.1);
  border-color: var(--text-muted);
}

.color-btn.active {
  border-color: var(--text-primary);
  transform: scale(1.1);
}

.color-btn.default-color {
  background-color: var(--bg-primary);
  color: var(--text-muted);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
}

.preview-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

.preview-box {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
}

.preview-large {
  width: 4rem;
  height: 4rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--bg-primary);
  border-radius: var(--radius-lg);
  font-size: 2rem;
}

.picker-footer {
  padding: 1rem 1.25rem;
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
}

.btn {
  padding: 0.5rem 1.25rem;
  border: none;
  border-radius: var(--radius);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
}

.btn.primary {
  background-color: var(--accent);
  color: white;
}

.btn.primary:hover {
  background-color: var(--accent-hover);
}
</style>
