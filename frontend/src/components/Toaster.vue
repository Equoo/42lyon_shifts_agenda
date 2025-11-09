<script setup lang="ts">
import { useToastStore } from '@/stores/toast.ts'

const toastStore = useToastStore()
</script>

<template>
  <Teleport to="body">
    <TransitionGroup
      name="toasts"
      tag="div"
      class="fixed top-5 right-2 flex flex-col float-right mr-5 space-y-3 w-x"
    >
      <div
        v-for="toast in toastStore.toasts"
        :key="toast.id"
        :class="'toast-' + toast.type"
        class="outline outline-gray-300/80 bg-gray-200 dark:outline-gray-500/70 dark:bg-gray-700 rounded-md px-3 py-1 border-b-3"
      >
        {{ toast.msg }}
        <button @click="toastStore.dismiss(toast.id)" class="btn float-right ml-3">X</button>
      </div>
    </TransitionGroup>
  </Teleport>
</template>

<style scoped></style>
