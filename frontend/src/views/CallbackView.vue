<script setup lang="ts">
import { onMounted } from 'vue'
import { getMe, handleLoginCallback } from '@/api.ts'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth.ts'
import { useToastStore } from '@/stores/toast.ts'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const toast = useToastStore()

const code = <String>route.query.code

onMounted(async () => {
  if (code === undefined) {
    toast.error('Invalid usage of route')
    await router.replace('/login')
    return
  }
  await handleLoginCallback(code)
    .then(getMe)
    .then((user) => {
      toast.success('Logged in! Welcome, ' + user.login + '!')
      authStore.user = user
      router.replace({ path: '/' })
    })
    .catch((e) => {
      console.error(e)
      toast.error('Invalid code')
      router.replace('/login')
    })
})
</script>

<template>
  <div class="flex flex-items min-h-screen items-center justify-center">
    <p class="text-center">Authenticating...</p>
  </div>
</template>

<style scoped></style>
