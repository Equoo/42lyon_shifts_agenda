<script setup lang="ts">
import { ref } from 'vue'

const num = ref(0)

function randomNumber() {
  num.value = Math.random()
}
</script>

<template>
  <h1>Hello, world!</h1>
  <p>Number is: {{ num }}</p>
  <button @click="randomNumber">Random</button>
</template>

<style scoped></style>
