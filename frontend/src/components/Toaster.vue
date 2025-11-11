<script setup lang="ts">
import { useToastStore } from '@/stores/toast.ts'

const toastStore = useToastStore()
</script>

<template>
  <Teleport to="body">
    <TransitionGroup
      name="toasts"
      tag="div"
      class="fixed top-20 right-2 flex flex-col float-right mr-5 space-y-3 w-x"
    >
      <div
        v-for="toast in toastStore.toasts"
        :key="toast.id"
        :class="'toast-' + toast.type"
        class="outline outline-gray-300/80 bg-gray-200/70 dark:outline-gray-500/70 dark:bg-gray-700/70 rounded-md rounded-l-xs px-3 py-1.5 border-l-4 w-xs max-w-xs"
      >
        <button
          @click="toastStore.dismiss(toast.id)"
          class="float-right ml-3 cursor-pointer rounded-full text-center px-1"
        >
          &#10006;
        </button>
        {{ toast.msg }}
      </div>
    </TransitionGroup>
  </Teleport>
</template>

<style scoped></style>
