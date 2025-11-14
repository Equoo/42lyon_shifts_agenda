<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { getShiftsWeek } from '@/api.ts'
import { DateTime, Interval } from 'luxon'
import { Shift } from '@/types/shift.ts'
import { useToastStore } from '@/stores/toast.ts'
import ShiftCard from '@/components/ShiftCard.vue'

const toast = useToastStore()

let loaded = ref(false)
let shifts = [] as Shift[]

let currentDate = DateTime.now().minus({ days: 1 })

async function getShifts(date: DateTime) {
  loaded.value = false
  await getShiftsWeek(<string>date.toSQLDate())
    .then((res) => {
      const start = date
      const end = date.plus({ days: 7 })
      shifts = []
      Interval.fromDateTimes(start, end)
        .splitBy({ days: 1 })
        .map((d) => <DateTime>d.start)
        .forEach((d) => {
          const noon = d.set({ hour: 12, minute: 0, second: 0, millisecond: 0 })
          const dayShift = res.find(
            (s) =>
              d.toISODate() === s.datetime.toISODate() &&
              s.datetime.toISOTime() === noon.toISOTime(),
          )
          shifts.push(
            dayShift !== undefined ? dayShift : new Shift(<string>noon.toSQLDate(), 'day', []),
          )
          const night = d.set({ hour: 20, minute: 0, second: 0, millisecond: 0 })
          const nightShift = res.find(
            (s) =>
              d.toISODate() === s.datetime.toISODate() &&
              s.datetime.toISOTime() === night.toISOTime(),
          )
          shifts.push(nightShift || new Shift(<string>night.toSQLDate(), 'night', []))
        })
      loaded.value = true
    })
    .catch((e) => toast.error(e))
}

async function stepBack() {
  currentDate = currentDate.minus({ days: 7 })
  await getShifts(currentDate)
}

async function stepForward() {
  currentDate = currentDate.plus({ days: 7 })
  await getShifts(currentDate)
}

onMounted(async () => {
  await getShifts(currentDate)
})

function randomInt(min: number, max: number) {
  min = Math.ceil(min)
  max = Math.floor(max)
  return Math.floor(Math.random() * (max - min + 1)) + min
}

const randomWidthClasses = ['w-2xs', 'w-3xs', 'w-sm', 'w-xs']
</script>

<template>
  <div class="flex m-4">
    <button type="button" class="btn mx-2 w-1/3" @click="stepBack"><< Previous 7 days</button>
    <div class="w-1/2"></div>
    <button type="button" class="btn mx-2 w-1/3" @click="stepForward">Next 7 days >></button>
  </div>
  <div v-if="loaded" class="m-4 space-y-2">
    <ShiftCard
      v-for="shift in shifts"
      v-bind="shift"
      :class="shift.slot === 'day' ? 'mt-5' : 'mb-5'"
    />
  </div>
  <div v-else class="m-4 space-y-2">
    <div
      v-for="index in 14"
      class="outline-gray-600 outline rounded-xl flex h-17"
      :class="index % 2 === 1 ? 'mt-4' : 'mb-4'"
    >
      <div class="p-5">
        <div
          class="rounded-2xl bg-gray-700 animate-pulse h-7.5"
          :class="randomWidthClasses[randomInt(0, randomWidthClasses.length - 1)]"
        ></div>
      </div>
      <div class="flex p-4">
        <div v-for="circle in randomInt(1, 5)" class="rounded-full h-9 -mx-1.5 bg-gray-800">
          <div class="bg-gray-700 rounded-full w-7 h-7 m-1 animate-pulse"></div>
        </div>
      </div>
    </div>
  </div>
  <div class="flex m-4">
    <button type="button" class="btn mx-2 w-1/3" @click="stepBack"><< Previous 7 days</button>
    <div class="w-1/2"></div>
    <button type="button" class="btn mx-2 w-1/3" @click="stepForward">Next 7 days >></button>
  </div>
</template>

<style scoped></style>
