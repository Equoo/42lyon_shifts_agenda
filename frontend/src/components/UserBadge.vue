<script setup lang="ts">
import { UserGrade } from '@/types/user.ts'

defineProps<{
  login: string
  img_url: string
  grade: UserGrade
}>()
</script>

<template>
  <div
    class="flex space-x-2 rounded-md px-3 py-1.5 max-w-fit"
    :class="'badge-grade-' + UserGrade[grade]?.toLowerCase()"
  >
    <img class="rounded-full aspect-square h-6.5" :src="img_url" :alt="login" />
    <span>{{ login }}</span>
  </div>
</template>

<style scoped></style>
