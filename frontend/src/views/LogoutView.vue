<script setup lang="ts">
import { useAuthStore } from '@/stores/auth.ts'
import { useRouter } from 'vue-router'

const auth = useAuthStore()
const router = useRouter()

auth.logout().then(() => {
  console.log('waa')
  router.replace('/login')
})
</script>

<template>
  <div class="flex flex-items min-h-screen items-center justify-center">
    <p class="text-center">Logging out...</p>
  </div>
</template>

<style scoped></style>
