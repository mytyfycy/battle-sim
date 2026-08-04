import { describe, it, expect } from 'vitest'
import {
  getActiveDefenseBonus,
  getEffectiveDefense,
  describeStatuses,
} from './statusEffects.js'

describe('getActiveDefenseBonus', () => {
  it('returns 0 when the status list is not an array', () => {
    expect(getActiveDefenseBonus(undefined)).toBe(0)
    expect(getActiveDefenseBonus(null)).toBe(0)
    expect(getActiveDefenseBonus('not an array')).toBe(0)
  })

  it('returns 0 for an empty status list', () => {
    expect(getActiveDefenseBonus([])).toBe(0)
  })

  it('sums the amount of every ExtraDefenseAura status', () => {
    const status_list = [
      { kind: { ExtraDefenseAura: { amount: 5 } } },
      { kind: { ExtraDefenseAura: { amount: 3 } } },
    ]
    expect(getActiveDefenseBonus(status_list)).toBe(8)
  })

  it('ignores statuses without a numeric amount', () => {
    const status_list = [
      { kind: { ExtraDefenseAura: { amount: 5 } } },
      { kind: { ExtraDefenseAura: {} } },
      { kind: null },
      {},
    ]
    expect(getActiveDefenseBonus(status_list)).toBe(5)
  })
})

describe('getEffectiveDefense', () => {
  it('adds the active defense bonus to the base defense', () => {
    const status_list = [{ kind: { ExtraDefenseAura: { amount: 5 } } }]
    expect(getEffectiveDefense(10, status_list)).toBe(15)
  })

  it('treats a missing base defense as 0', () => {
    expect(getEffectiveDefense(undefined, [])).toBe(0)
    expect(getEffectiveDefense(null, [])).toBe(0)
  })

  it('returns just the base defense when there is no status list', () => {
    expect(getEffectiveDefense(10, undefined)).toBe(10)
  })
})

describe('describeStatuses', () => {
  it('returns an empty array when the status list is not an array', () => {
    expect(describeStatuses(undefined)).toEqual([])
    expect(describeStatuses(null)).toEqual([])
  })

  it('describes an ExtraDefenseAura status in Polish', () => {
    const status_list = [{ kind: { ExtraDefenseAura: { amount: 5 } } }]
    expect(describeStatuses(status_list)).toEqual(['+5 obrony (Aura)'])
  })

  it('uses a "?" placeholder when the amount is missing', () => {
    const status_list = [{ kind: { ExtraDefenseAura: {} } }]
    expect(describeStatuses(status_list)).toEqual(['+? obrony (Aura)'])
  })

  it('falls back to a generic description for unknown status kinds', () => {
    const status_list = [{ kind: { SomeOtherStatus: {} } }]
    expect(describeStatuses(status_list)).toEqual(['Aktywny efekt: SomeOtherStatus'])
  })

  it('falls back to a plain label when there is no kind at all', () => {
    const status_list = [{}]
    expect(describeStatuses(status_list)).toEqual(['Aktywny efekt'])
  })
})
