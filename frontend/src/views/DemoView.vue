<script setup lang="ts">
import { User, UserGrade } from '@/types/user.ts'
import UserBadge from '@/components/UserBadge.vue'
import ShiftCard from '@/components/ShiftCard.vue'
import { useAuthStore } from '@/stores/auth.ts'
import { DateTime } from 'luxon'

const auth = useAuthStore()

function randomAvatar(seed: any) {
  return `https://api.dicebear.com/7.x/identicon/png?seed=${seed}`
}

const users = Object.values(UserGrade)
  .map((grade) => <string>UserGrade[<number>grade]?.toString())
  .filter((name) => name.length > 1)
  .map((name) => new User(name, randomAvatar(name), name))

const shiftUsers = [
  <User>auth.user,
  new User('mrwawa', randomAvatar('mrwawa'), 'Novice'),
  new User('arnold', randomAvatar('arnold'), 'President'),
]
</script>

<template>
  <div class="m-4">
    <ShiftCard :datetime="DateTime.now()" , slot="day" :users="shiftUsers" />
    <hr class="my-5" />
    <UserBadge v-for="user in users" v-bind="user" class="m-2" />
  </div>
</template>

<style scoped></style>
