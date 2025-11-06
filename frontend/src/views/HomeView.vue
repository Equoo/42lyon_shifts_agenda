<script setup lang="ts">
import { ref } from 'vue'

const num = ref(0)

function randomNumber() {
  num.value = Math.random()
}
</script>

<template>
  <h1 class="text-3xl text-center m-5">Hello, world!</h1>
  <div class="m-4 space-y-1">
    <p>Number is: {{ num }}</p>
    <button
      class="bg-blue-400 hover:bg-blue-500 text-gray-200 rounded-xl p-1"
      @click="randomNumber"
    >
      Random
    </button>
  </div>
</template>

<style scoped></style>
