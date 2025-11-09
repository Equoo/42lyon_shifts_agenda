<script setup lang="ts">
import { ref } from 'vue'
import { useAuthStore } from '@/stores/auth.ts'

const authStore = useAuthStore()

const num = ref(0)

async function randomNumber() {
  const button = <HTMLInputElement>document.querySelector('#wawa')
  button.disabled = true
  num.value = Math.random()
  await new Promise((f) => setTimeout(f, 1000))
  button.disabled = false
}

const login = authStore.user?.login
</script>

<template>
  <h1 class="text-3xl text-center m-5">Hello, {{ login }}!</h1>
  <div class="m-4 space-y-1">
    <p>Number is: {{ num }}</p>
    <button class="btn" id="wawa" @click="randomNumber">Random</button>
  </div>
</template>

<style scoped></style>
