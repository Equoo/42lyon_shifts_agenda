<script setup lang="ts">
import { getMe, login } from '@/api.ts'
import { ref } from 'vue'
import type { User } from '@/types/user.ts'

const num = ref(0)

async function randomNumber() {
  const button = <HTMLInputElement>document.querySelector('#wawa')
  button.disabled = true
  num.value = Math.random()
  await new Promise((f) => setTimeout(f, 1000))
  button.disabled = false
}

const me = await getMe().catch(async (res) => {
  console.log(res)
  if (res.status === 401) {
    await login().then(async (uri) => {
      console.log('logging in in 3 seconds')
      await new Promise((wawa) => setTimeout(wawa, 3000))
      window.location.href = uri
      await new Promise((wawa) => setTimeout(wawa, 9999))
    })
  }
})

let username: string;
if (me === undefined)
  username = "world";
else
  username = (<User>me).login
</script>

<template>
  <h1 class="text-3xl text-center m-5">Hello, {{ username }}!</h1>
  <div class="m-4 space-y-1">
    <p>Number is: {{ num }}</p>
    <button class="btn" id="wawa" @click="randomNumber">Random</button>
  </div>
</template>

<style scoped></style>
