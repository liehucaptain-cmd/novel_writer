<template>
  <teleport to="body">
    <div class="modal-overlay" :class="{ show: modelValue }" @click.self="$emit('update:modelValue', false)">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <div class="modal-title">{{ title }}</div>
          <button class="close-btn" @click="$emit('update:modelValue', false)">×</button>
        </div>
        <slot />
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'

const props = defineProps<{ modelValue: boolean; title: string }>()
const emit = defineEmits(['update:modelValue'])

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape' && props.modelValue) emit('update:modelValue', false)
}
onMounted(() => document.addEventListener('keydown', onKeydown))
onUnmounted(() => document.removeEventListener('keydown', onKeydown))
</script>
