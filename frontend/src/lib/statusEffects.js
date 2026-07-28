export function getActiveDefenseBonus(statusList) {
  if (!Array.isArray(statusList)) return 0

  return statusList.reduce((sum, status) => {
    const amount = status?.kind?.ExtraDefenseAura?.amount
    return typeof amount === 'number' ? sum + amount : sum
  }, 0)
}

export function getEffectiveDefense(baseDefense, statusList) {
  return (baseDefense ?? 0) + getActiveDefenseBonus(statusList)
}

export function describeStatuses(statusList) {
  if (!Array.isArray(statusList)) return []

  return statusList.map((status) => {
    const kindName = status?.kind ? Object.keys(status.kind)[0] : null

    if (kindName === 'ExtraDefenseAura') {
      const amount = status.kind.ExtraDefenseAura?.amount ?? '?'
      return `+${amount} obrony (Aura)`
    }

    return kindName ? `Aktywny efekt: ${kindName}` : 'Aktywny efekt'
  })
}
