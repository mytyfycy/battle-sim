<script>
    import BattleScene from './BattleScene.svelte'
    import StatPanel from './StatPanel.svelte'
    import BattleLog from './BattleLog.svelte'
    import { translateSpell } from '../lib/spellMap'

    let { battle, instant = false, finishedActions = null} = $props()

    const REVEAL_DELAY_MS = 250

    let revealedCount = $state(0)
    let revealing = $state(false)

    const totalTurns = $derived(battle?.turns?.length ?? 0)
    const isFinished = $derived(totalTurns > 0 && revealedCount === totalTurns)
    const currentTurn = $derived(revealedCount > 0 ? battle.turns[revealedCount - 1] : null)
    const attackerTeam = $derived(currentTurn?.attacker_team ?? null)
    const defenderTeam = $derived(attackerTeam === 'A' ? 'B' : attackerTeam === 'B' ? 'A' : null)

    const cardA = $derived.by(() => {
      if (!battle) return null

      const base = battle.character_a_start
      const snapshot = revealedCount > 0 ? battle.turns[revealedCount - 1].character_a_after : base
      return { name: base.name, maxHp: base.max_hp, strength: base.strength, spell: base.spell, ...snapshot }
    })

    const cardB = $derived.by(() => {
      if (!battle) return null

      const base = battle.character_b_start
      const snapshot = revealedCount > 0 ? battle.turns[revealedCount - 1].character_b_after : base
      return { name: base.name, maxHp: base.max_hp, strength: base.strength, spell: base.spell, ...snapshot }
    })

    const winnerName = $derived.by(() => {
      if (!isFinished || !battle) return null

      return battle.winner_team === 'A' ? battle.character_a_start.name : battle.character_b_start.name
    })

    const spellResult = $derived(
      currentTurn?.spell_description ? translateSpell(currentTurn.spell_description, currentTurn.spell_triggered) : null
    )
    const spellNamePl = $derived(spellResult?.name ?? null)
    const spellTextPl = $derived(spellResult?.text ?? null)
    const spellKind = $derived(spellResult?.kind ?? 'damage')

    const floatSpellText = $derived(spellNamePl ? `* ${spellNamePl}` : null)
    const floatDamageText = $derived(currentTurn ? `-${currentTurn.base_damage}` : null)

    const floatA = $derived(!currentTurn ? null : attackerTeam === 'A' ? floatSpellText : defenderTeam === 'A' ? floatDamageText : null)
    const floatB = $derived(!currentTurn ? null : attackerTeam === 'B' ? floatSpellText : defenderTeam === 'B' ? floatDamageText : null)

    const floatKindA = $derived(attackerTeam === 'A' ? spellKind : 'damage')
    const floatKindB = $derived(attackerTeam === 'B' ? spellKind : 'damage')

    const attackerName = $derived(!currentTurn ? null : attackerTeam === 'A' ? cardA?.name : cardB?.name)
    const defenderName = $derived(!currentTurn ? null : defenderTeam === 'A' ? cardA?.name : cardB?.name)

    function revealNext() {
      if (!battle || revealing || isFinished) return

      if (instant) {
        revealedCount++
        return
      }

      revealing = true
      setTimeout(() => {
        revealedCount++
        revealing = false
      }, REVEAL_DELAY_MS)
    }

    function revealPrev() {
      if (revealedCount > 0) revealedCount--
    }

    function revealAll() {
      revealedCount = totalTurns
    }

    export function reset() {
      revealedCount = 0
      revealing = false
    }
</script>

{#if battle}
    <div class="max-w-3xl mx-auto">
        <BattleScene
            {cardA}
            {cardB}
            {attackerTeam}
            turnKey={revealedCount}
            {floatA}
            {floatB}
            {floatKindA}
            {floatKindB}
        />

        <div class="mt-4">
            <BattleLog
                {attackerName}
                {defenderName}
                damage={currentTurn?.base_damage ?? null}
                spellName={spellNamePl}
                spellText={spellTextPl}
                turnKey={revealedCount}
            />
        </div>

        <div class="grid grid-cols-2 gap-4 my-4">
            <StatPanel {...cardA} team="A" highlighted={attackerTeam === 'A'} />
            <StatPanel {...cardB} team="B" highlighted={attackerTeam === 'B'} />
        </div>

        {#if isFinished}
            <div class="text-center">
                <p class="text-xl font-bold mb-4" style="color: var(--gold);">{winnerName} wygrywa!</p>
                <div class="flex items-center justify-center gap-3 flex-wrap">
                    {#if revealedCount > 0}
                        <button
                            onclick={revealPrev}
                            class="px-4 py-2 rounded-lg font-medium border border-(--panel-border) text-(--ink-dim) hover:text-(--ink) active:scale-95 cursor-pointer transition-all"
                        >
                            Poprzednia tura
                        </button>
                    {/if}
                    {#if finishedActions}
                        {@render finishedActions()}
                    {/if}
                </div>
            </div>
        {:else}
            <div class="flex items-center justify-center gap-3 flex-wrap">
                {#if revealedCount > 0}
                    <button
                        onclick={revealPrev}
                        class="px-4 py-2 rounded-lg font-medium border cursor-pointer border-(--panel-border) text-(--ink-dim) hover:text-(--ink) active:scale-95 transition-all"
                    >
                        Poprzednia tura
                    </button>
                {/if}
                <button
                    onclick={revealNext}
                    disabled={revealing}
                    class="px-6 py-3 rounded-lg font-semibold bg-(--gold) hover:text-(--ink) active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer transition-all"
                >
                    {revealing ? '...' : 'Nastepna tura'}
                </button>
                {#if instant && revealedCount < totalTurns}
                    <button
                        onclick={revealAll}
                        class="px-4 py-2 rounded-lg font-medium border active:scale-95"
                        style="border-color: var(--panel-border); color: var(--ink-dim);"
                    >
                        Pokaz cala walke
                    </button>
                {/if}
            </div>
        {/if}
    </div>
{/if}
