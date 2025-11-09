<script setup lang="ts">
import { useAuthStore } from '@/stores/auth.ts'

const authStore = useAuthStore()

async function login() {
  await authStore.login()
}
</script>

<template>
  <div class="flex flex-items min-h-screen items-center justify-center">
    <button class="btn text-center" @click="login">Log in with 42 intra</button>
  </div>
</template>

<style scoped></style>
