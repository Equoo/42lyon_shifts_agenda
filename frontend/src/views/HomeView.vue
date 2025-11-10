<script setup lang="ts">
import { ref } from 'vue'
import { useToastStore } from '@/stores/toast.ts'

const toast = useToastStore()

const num = ref(0)

async function randomNumber() {
  const button = <HTMLInputElement>document.querySelector('#wawa')
  button.disabled = true
  num.value = Math.random()
  await new Promise((f) => setTimeout(f, 1000))
  button.disabled = false
}
</script>

<template>
  <h1 class="text-3xl text-center m-5">Hello!</h1>
  <div class="m-4 space-y-1">
    <p>Number is: {{ num }}</p>
    <div class="flex flex-col space-y-1 max-w-50">
      <button class="btn" id="wawa" @click="randomNumber">Random</button>
      <button class="btn" @click="toast.info('wawa')">Notify me</button>
    </div>
  </div>
</template>

<style scoped></style>
