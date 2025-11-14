<script setup lang="ts">
import { useToastStore } from '@/stores/toast.ts'

const toastStore = useToastStore()

function pauseToast(toastId: number) {
  const elem = document.getElementById('toast-' + toastId)
  if (!elem?.classList.contains('pause-animation')) elem?.classList.add('pause-animation')
  toastStore.pause(toastId)
}

function resumeToast(toastId: number) {
  const elem = document.getElementById('toast-' + toastId)
  if (elem?.classList.contains('pause-animation')) elem?.classList.remove('pause-animation')
  toastStore.resume(toastId)
}
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
        @mouseover="pauseToast(toast.id)"
        @mouseleave="resumeToast(toast.id)"
        class="outline outline-gray-300/80 bg-gray-200/70 dark:outline-gray-500/70 dark:bg-gray-700/70 rounded-md rounded-l-xs border-l-4 w-xs max-w-xs overflow-hidden"
      >
        <div class="px-3 py-1.5">
          <button
            @click="toastStore.dismiss(toast.id)"
            class="float-right ml-3 cursor-pointer rounded-full text-center px-1"
          >
            &#10006;
          </button>
          {{ toast.msg }}
        </div>
        <div
          class="h-1.25 bg-gray-500 transition-all toast-fade-timer"
          :id="'toast-' + toast.id"
        ></div>
      </div>
    </TransitionGroup>
  </Teleport>
</template>

<style scoped></style>
