import type { User } from '@/types/user.ts'
import { DateTime } from 'luxon'

export type ShiftSlot = 'day' | 'night'

export class Shift {
  datetime: DateTime
  slot: ShiftSlot
  users: User[]

  constructor(dateStr: string, slot: ShiftSlot, users: User[]) {
    const time = slot == 'day' ? 12 : 20
    this.datetime = DateTime.fromSQL(dateStr).plus({ hour: time })
    this.slot = slot
    this.users = users
  }
}
