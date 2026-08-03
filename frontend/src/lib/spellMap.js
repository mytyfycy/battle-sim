const BASE_SPELL_NAMES = [
  { pl: 'Mocne uderzenie', keyword: 'hardhit' },
  { pl: 'Lodowy pocisk', keyword: 'icebullet' },
  { pl: 'Tarcza', keyword: 'shield' },
  { pl: 'Aura', keyword: 'aura' },
  { pl: 'Krytyczny cios', keyword: 'criticalhit' },
  { pl: 'Glos natury', keyword: 'naturevoice' },
  { pl: 'Szal', keyword: 'rage' },
  { pl: 'Reinkarnacja', keyword: 'reincarnation' },
]

const PATTERNS = [
  {
    test: /^Defense up by (\d+)!$/i,
    name: 'Tarcza',
    kind: 'buff',
    text: (m) => `Obrona zostaje na stale podniesiona o ${m[1]}`,
  },
  {
    test: /^Reincarnation just healed (\d+)\s*HP!$/i,
    name: 'Reinkarnacja',
    kind: 'buff',
    text: (m) => `Wytrzymalosc zostaje odnowiona o ${m[1]} HP`,
  },
  {
    test: /^Attacker is enraged!$/i,
    name: 'Szal',
    kind: '',
    text: (m) => `W nastepnej turze zaatakuje ponownie`,
  },
  {
    test: /^Attacker is not angry enough!$/i,
    name: 'Szal',
    kind: 'fail',
    text: (m) => `Szal nie zadzialal. Brak dodatkowej tury`,
  },
  {
    test: /^Attacker'?s health is fully restored!$/i,
    name: 'Glos natury',
    kind: 'heal',
    text: (m) => `Wytrzymalosc zostala w pelni przywrocona`,
  },
  {
    test: /^Attacker failed to restore its health!$/i,
    name: 'Glos natury',
    kind: 'fail',
    text: () => `Glos natury nie zadzialal. Wytrzymalosc pozostaje bez zmian`,
  },
  {
    test: /^Ice bullet dealt (\d+)\s*HP!$/i,
    name: 'Lodowy pocisk',
    kind: 'damage',
    text: (m) => `Lodowy pocisk zadaje ${m[1]} obrazen`,
  },
  {
    test: /^Hard hit dealt (\d+)\s*HP!$/i,
    name: 'Mocne uderzenie',
    kind: 'damage',
    text: (m) => `Mocne uderzenie zadaje ${m[1]} obrazen`,
  },
  {
    test: /^Attacker hit a critical!$/i,
    name: 'Krytyczny cios',
    kind: 'buff',
    text: (m) => `Cios krytyczny natychmiastowo konczy walke`,
  },
  {
    test: /^Attacker missed a critical!$/i,
    name: 'Krytyczny cios',
    kind: 'fail',
    text: (m) => `Cios krytyczny nie trafil`,
  },
  {
    test: /^Next defense just got stronger!$/i,
    name: 'Aura',
    kind: 'buff',
    text: (m, ctx) =>
      ctx?.promisedDefense
        ? `W nastepnej turze obrona bedzie zwiekszona o ${ctx.promisedDefense}`
        : `W nastepnej turze obrona bedzie zwiekszona`
  },
]

function matchDescription(rawDescription) {
  const desc = String(rawDescription ?? '').trim()
  if (!desc) return null

  for (const pattern of PATTERNS) {
    const m = desc.match(pattern.test)
    if (m) return { name: pattern.name, kind: pattern.kind, text: pattern.text(m) }
  }

  return null
}

export function translateSpell(rawDescription, rawName) {
  const matched = matchDescription(rawDescription)
  if (matched) return matched

  console.warn('[spellMap] Nierozpoznane zaklecie: ', rawDescription)

  return {
    name: rawName || 'Nieznane zaklecie',
    kind: 'buff',
    text: rawDescription || '',
  }
}

export function translateSpellName(rawDescription, rawName) {
  return translateSpell(rawDescription, rawName).name
}

export function translateSpellDescription(rawDescription, rawName) {
  return translateSpell(rawDescription, rawName).text
}

function normalizeKey(str) {
  return String(str ?? '').toLowerCase().normalize('NFKD').replace(/[^a-z]/g, '')
}

export function translateBaseSpell(rawName) {
  const norm = normalizeKey(rawName)
  if (!norm) return rawName ?? ''

  const found = BASE_SPELL_NAMES.find((s) => norm.includes(s.keyword))
  return found ? found.pl : rawName
}
