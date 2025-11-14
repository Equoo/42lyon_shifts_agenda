<script setup lang="ts">
import { DateTime } from 'luxon'
import { type User, UserGrade } from '@/types/user.ts'
import UserBadge from '@/components/UserBadge.vue'
import { ref } from 'vue'
import { useAuthStore } from '@/stores/auth.ts'
import type { ShiftSlot } from '@/types/shift.ts'
import { registerToShift, unregisterFromShift } from '@/api.ts'
import { useToastStore } from '@/stores/toast.ts'

const auth = useAuthStore()
const toast = useToastStore()

const props = defineProps<{
  datetime: DateTime
  slot: ShiftSlot
  users: User[]
}>()

const dateStr = props.datetime.toFormat('dd/MM/yyyy HH:mm')
const shiftId = props.datetime.toFormat('dd-MM-yyyy-HH-mm')

const expanded = ref(false)
const inShift = ref(props.users.some((u) => u.login == auth.user?.login))
const now = DateTime.now()

async function register() {
  await registerToShift(<string>props.datetime.toSQLDate(), props.slot)
    .then(() => {
      toast.success('Registered to shift')
      props.users.push(<User>auth.user)
      inShift.value = true
    })
    .catch((e) => {
      toast.error(e)
    })
}

async function unregister() {
  await unregisterFromShift(<string>props.datetime.toSQLDate(), props.slot)
    .then(() => {
      toast.success('Unregistered from shift')
      const index = props.users.indexOf(<User>auth.user)
      props.users.splice(index, 1)
      inShift.value = false
    })
    .catch((e) => {
      toast.error(e)
    })
}
</script>

<template>
  <div
    class="outline-gray-600 outline rounded-xl overflow-hidden transition-all"
    :class="expanded ? 'max-h-45' : 'max-h-17'"
    :id="shiftId"
  >
    <div
      class="bg-gray-700/30 hover:cursor-pointer hover:bg-white/10 transition-colors duration-300 select-none flex"
      @click="expanded = !expanded"
    >
      <div class="font-medium text-xl p-5">{{ dateStr }}</div>
      <div class="flex p-4">
        <div
          v-for="user in users"
          class="rounded-full h-9 -mx-1.5"
          :class="'badge-grade-' + UserGrade[user.grade].toLowerCase()"
        >
          <img :src="user.img_url" :alt="user.login" class="rounded-full aspect-square h-7 m-1" />
        </div>
      </div>
    </div>
    <div class="overflow-hidden rounded-b-xl">
      <hr class="border-gray-600" />
      <div class="bg-gray-700/10">
        <div class="flex flex-row space-x-2 p-3">
          <span class="m-1.5 mx-2">Users registered:</span>
          <UserBadge v-for="user in users" v-bind="user" />
          <span v-if="users.length === 0" class="m-1.5">Nobody :(</span>
        </div>
        <div class="px-1 pb-1">
          <button v-if="now > datetime" type="button" class="btn m-2" disabled>Shift passed</button>
          <button v-else-if="inShift" type="button" class="btn m-2" @click="unregister">
            Unregister from shift
          </button>
          <button v-else type="button" class="btn m-2" @click="register">Register to shift</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
