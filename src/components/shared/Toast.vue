<template>
  <teleport to="body">
    <div class="toast" :class="{ show: visible }">{{ message }}</div>
  </teleport>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const props = defineProps<{ message: string; duration?: number }>()
const visible = ref(false)

function show() {
  visible.value = true
  setTimeout(() => { visible.value = false }, props.duration ?? 3000)
}

onMounted(show)
</script>
