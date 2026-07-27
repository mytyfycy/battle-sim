<script>
    import { translateBaseSpell } from "../lib/spellMap"
    import { getActiveDefenseBonus, describeStatuses } from "../lib/statusEffects"

    let { name, hp, maxHp, strength, defense, spell, status_list = [], team = 'A', highlighted = false} = $props()

    const hpPercent = $derived(Math.max(0, Math.min(100, (hp / maxHp) * 100)))
    const teamColor = $derived(team === 'A' ? 'var(--teal)' : 'var(--crimson)')
    const spellPl = $derived(translateBaseSpell(spell))
    const defenseBonus = $derived(getActiveDefenseBonus(status_list))
    const effectiveDefense = $derived(defense + defenseBonus)
    const statusBadges = $derived(describeStatuses(status_list))
</script>

<div
    class="rounded-lg border-2 p-4 transition-colors duration-300"
    style="background: var(--panel); border-color: {highlighted ? 'var(--gold)' : 'var(--panel-border)'};
    {highlighted ? 'box-shadow: 0 0 16px rgba(200,170,100,0.25);' : ''}
">
    <h2 class="font-semibold text-lg mb-2 flex items-center gap-2">
        <span class="inline-block w-2.5 h-2.5 rotate-45" style="background: {teamColor};"></span>
        <span style="color: var(--ink);">{name}</span>
    </h2>

    <div class="mb-3">
        <div class="flex justify-between text-xs mb-1" style="color: var(--ink-dim);">
            <span>HP</span>
            <span>{Math.max(0, hp)} / {maxHp}</span>
        </div>
        <div class="w-full h-2.5 overflow-hidden rounded-full"
            style="background: rgba(0,0,0,0.5); border: 1px solid var(--panel-border);">
                <div class="h-full transition-all duration-250"
                    style="width: {hpPercent}%;
                    background: {hpPercent > 50 ? 'var(--emerald)'
                    : hpPercent > 20 ? 'var(--gold)' : 'var(--crimson)'};">
                </div>
            </div>
    </div>

    <dl class="grid grid-cols-2 gap-x-4 gap-y-1.5 text-sm">
        <dt style="color: var(--ink-dim);">Sila</dt>
        <dd class="text-right" style="color: var(--ink);">{strength}</dd>
        <dt style="color: var(--ink-dim);">Obrona</dt>
        <dd class="text-right" style="color: var(--ink);">
            {effectiveDefense}
            {#if defenseBonus > 0}
                <span style="color: var(--gold);">({defense}+{defenseBonus})</span>
            {/if}
        </dd>
        <dt style="color: var(--ink-dim);">Zaklecie</dt>
        <dd class="text-right" style="color: var(--gold);">{spellPl}</dd>
    </dl>

    {#if statusBadges.length > 0}
        <div class="mt-2.5 flex flex-wrap gap-1.5">
            {#each statusBadges as badge}
                <span class="text-xs px-1.5 py-0.5 rounded"
                    style="background: rgba(210, 175, 108, 0.1); color: var(--gold);"
                >
                    {badge}
                </span>
            {/each}
        </div>
    {/if}

</div>
