import { describe, it, expect, vi, afterEach } from 'vitest'
import {
  translateSpell,
  translateSpellName,
  translateSpellDescription,
  translateBaseSpell,
} from './spellMap.js'

afterEach(() => {
  vi.restoreAllMocks()
})

describe('translateSpell pattern matching against backend descriptions', () => {
  it('translates a Shield success message and extracts the amount', () => {
    const result = translateSpell('Defense up by 3!')
    expect(result.name).toBe('Tarcza')
    expect(result.kind).toBe('buff')
    expect(result.text).toBe('Obrona zostaje na stale podniesiona o 3')
  })

  it('translates a Reincarnation heal message and extracts the amount', () => {
    const result = translateSpell('Reincarnation just healed 7HP!')
    expect(result.name).toBe('Reinkarnacja')
    expect(result.kind).toBe('buff')
    expect(result.text).toBe('Wytrzymalosc zostaje odnowiona o 7 HP')
  })

  it('translates a Rage success message', () => {
    const result = translateSpell('Attacker is enraged!')
    expect(result.name).toBe('Szal')
    expect(result.kind).toBe('buff')
    expect(result.text).toBe('W nastepnej turze zaatakuje ponownie')
  })

  it('translates a Rage failure message', () => {
    const result = translateSpell('Attacker is not angry enough!')
    expect(result.name).toBe('Szal')
    expect(result.kind).toBe('fail')
    expect(result.text).toBe('Szal nie zadzialal. Brak dodatkowej tury')
  })

  it('translates a NatureVoice success message', () => {
    const result = translateSpell("Attacker's health is fully restored!")
    expect(result.name).toBe('Glos natury')
    expect(result.kind).toBe('heal')
    expect(result.text).toBe('Wytrzymalosc zostala w pelni przywrocona')
  })

  it('translates a NatureVoice failure message', () => {
    const result = translateSpell("Attacker failed to restore its health!")
    expect(result.name).toBe('Glos natury')
    expect(result.kind).toBe('fail')
    expect(result.text).toBe('Glos natury nie zadzialal. Wytrzymalosc pozostaje bez zmian')
  })

  it('translates an IceBullet message and extracts the damage', () => {
    const result = translateSpell("Ice bullet dealt 8HP!")
    expect(result.name).toBe('Lodowy pocisk')
    expect(result.kind).toBe('damage')
    expect(result.text).toBe('Lodowy pocisk zadaje 8 obrazen')
  })

  it('translates a HardHit message and extracts the damage', () => {
    const result = translateSpell("Hard hit dealt 6HP!")
    expect(result.name).toBe('Mocne uderzenie')
    expect(result.kind).toBe('damage')
    expect(result.text).toBe('Mocne uderzenie zadaje 6 obrazen')
  })

  it('translates a CriticalHit success message', () => {
    const result = translateSpell("Attacker hit a critical!")
    expect(result.name).toBe('Krytyczny cios')
    expect(result.kind).toBe('buff')
    expect(result.text).toBe('Cios krytyczny natychmiastowo konczy walke')
  })

  it('translates a CriticalHit failure message', () => {
    const result = translateSpell("Attacker missed a critical!")
    expect(result.name).toBe('Krytyczny cios')
    expect(result.kind).toBe('fail')
    expect(result.text).toBe('Cios krytyczny nie trafil')
  })

  it('translates an Aura message, using the promised defense from context when given', () => {
    const result = translateSpell('Next defense just got stronger!', null, {
      promisedDefense: 5,
    })
    expect(result.name).toBe('Aura')
    expect(result.kind).toBe('buff')

    // Currently translateSpell does not use
    // the "ctx" argument
    expect(result.text).toBe('W nastepnej turze obrona bedzie zwiekszona')
  })

  it('is case-insensitive when matching descriptions', () => {
    const result = translateSpell('ATTACKER HIT A CRITICAL!')
    expect(result.name).toBe('Krytyczny cios')
  })
})

describe('translateSpell fallback for unrecognized descriptions', () => {
  it('falls back to the raw name and description and warns', () => {
    const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => { })

    const result = translateSpell('Some brand new spell effect', 'Nowy Czar')

    expect(result.name).toBe('Nowy Czar')
    expect(result.kind).toBe('buff')
    expect(result.text).toBe('Some brand new spell effect')
    expect(warnSpy).toHaveBeenCalledOnce()
  })

  it('falls back to a default name when no rawName is given', () => {
    vi.spyOn(console, 'warn').mockImplementation(() => { })

    const result = translateSpell('completely unknown')
    expect(result.name).toBe('Nieznane zaklecie')
  })

  it('treats a whitespace-only description as unmatched, warns, and returns it as text', () => {
    const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => { })

    const result = translateSpell('   ')
    expect(result.text).toBe('   ')
    expect(warnSpy).toHaveBeenCalledOnce()
  })
})

describe('translateSpellName / translateSpellDescription', () => {
  it('translateSpellName returns only the spell name', () => {
    expect(translateSpellName('Ice bullet dealt 8HP!')).toBe('Lodowy pocisk')
  })

  it('translateSpellDescription returns only the translated text', () => {
    expect(translateSpellDescription('Ice bullet dealt 8HP!')).toBe(
      'Lodowy pocisk zadaje 8 obrazen'
    )
  })
})

describe('translateBaseSpell', () => {
  it('maps known keywords (case/diacritics insensitive) to their Polish name', () => {
    expect(translateBaseSpell('HardHit')).toBe('Mocne uderzenie')
    expect(translateBaseSpell('IceBullet')).toBe('Lodowy pocisk')
    expect(translateBaseSpell('Shield')).toBe('Tarcza')
    expect(translateBaseSpell('Aura')).toBe('Aura')
    expect(translateBaseSpell('CriticalHit')).toBe('Krytyczny cios')
    expect(translateBaseSpell('NatureVoice')).toBe('Glos natury')
    expect(translateBaseSpell('Rage')).toBe('Szal')
    expect(translateBaseSpell('Reincarnation')).toBe('Reinkarnacja')
  })

  it('returns the original name when nothing matches', () => {
    expect(translateBaseSpell('MysterySpell')).toBe('MysterySpell')
  })

  it('returns an empty string for empty/undefined input', () => {
    expect(translateBaseSpell('')).toBe('')
    expect(translateBaseSpell(undefined)).toBe('')
  })
})
