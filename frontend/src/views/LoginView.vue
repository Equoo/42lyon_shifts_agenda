<script setup lang="ts">
import { useAuthStore } from '@/stores/auth.ts'
import { useToastStore } from '@/stores/toast.ts'

const authStore = useAuthStore()
const toast = useToastStore()

async function login() {
  await authStore.login().catch((e) => {
    if (e.status == 502)
      toast.error('Failed to reach server')
    else
      toast.error(`An unexpected error occurred: ${e.data}`)
  })
}
</script>

<template>
  <div class="flex flex-items min-h-screen items-center justify-center">
    <button class="btn text-center" @click="login">Log in with 42 intra</button>
  </div>
</template>

<style scoped></style>
