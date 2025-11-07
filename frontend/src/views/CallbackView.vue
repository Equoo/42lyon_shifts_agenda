<script setup lang="ts">
import { onMounted } from 'vue'
import { handleLoginCallback } from '@/api.ts'
import { useRoute, useRouter } from 'vue-router'

const router = useRouter()
const route = useRoute()

const code = <String>route.query.code

onMounted(async () => {
  if (code === undefined) {
    console.error('invalid usage of route')
    return
  }
  await handleLoginCallback(code)
    .then((res) => {
      console.log(res)
      router.replace({ path: '/' })
    })
    .catch((e) => {
      console.error(e)
    })
})
</script>

<template>
  <div v-if="code === undefined">Missing code</div>
  <div v-else>Authenticating...</div>
</template>

<style scoped></style>
