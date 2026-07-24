<script>
    import CharacterCard from '../components/CharacterCard.svelte'
    import { startBattle, ApiError } from '../lib/api/battles';

    let battle = $state(null)
    let revealedCount = $state(0)
    let revealing = $state(false)
    let starting = $state(false)
    let error = $state(null)

    const REVEAL_DELAY_MS = 600

    const isFinished = $derived(battle !== null && revealedCount === battle.turns.length)
    const currentTurn = $derived(
      battle && revealedCount > 0 ? battle.turns[revealedCount - 1] : null
    )

    const cardA = $derived.by(() => {
      if (!battle) return null
      const base = battle.character_a_start
      const snapshot = revealedCount > 0 ? battle.turns[revealedCount - 1].character_a_after : base
      return { name: base.name, maxHp: base.max_hp, strength: base.strength, spell: base.spell, ...snapshot}
    })

    const cardB = $derived.by(() => {
      if (!battle) return null
      const base = battle.character_b_start
      const snapshot = revealedCount > 0 ? battle.turns[revealedCount - 1].character_b_after : base
      return { name: base.name, maxHp: base.max_hp, strength: base.strength, spell: base.spell, ...snapshot}
    })

    const winnerName = $derived.by(() => {
      if (!isFinished) return null
      return battle.winner_team === 'A' ? battle.character_a_start.name : battle.character_b_start.name
    })

    async function handleStart() {
      starting = true
      error = null
      try {
        battle = await startBattle()
        revealedCount = 0
      } catch (err) {
        error = err instanceof ApiError ? err.message : 'Nieoczekiwany blad'
      } finally {
        starting = false
      }
    }

    function handleNextTurn() {
      if (!battle || revealing || isFinished) return

      revealing = true
      setTimeout(() => {
        revealedCount += 1
        revealing = false
      }, REVEAL_DELAY_MS)
    }
</script>

<div class="max-w-3xl mx-auto p-4">
    {#if error}
        <p class="bg-red-500 text-black p-4 rounded-xl mb-6 font-medium">Blad: {error}</p>
    {/if}

    {#if !battle}
        <div class="text-center py-12 bg-gray-700 border rounded-xl">
            <button
                onclick={handleStart}
                disabled={starting}
                class="px-6 py-3 bg-teal-500 hover:bg-teal-800 text-white rounded-lg font-semibold disabled:opacity-50 active:scale-95 transition-all cursor-pointer"
            >
                {starting ? 'Ladowanie...' : 'Rozpocznij gre'}
            </button>
        </div>
    {:else}
        <div class="grid grid-cols-2 gap-4 mb-4">
            <CharacterCard {...cardA} highlighted={currentTurn?.attacker_team === 'A'} />
            <CharacterCard {...cardB} highlighted={currentTurn?.attacker_team === 'B'} />
        </div>

        <div class="min-h-16 bg-gray-700 text-white p-3 mb-4 text-sm border rounded-xl">
            {#if currentTurn}
                <p>
                    <strong>{currentTurn.attacker_team === 'A' ? cardA.name : cardB.name}</strong>
                    atakuje zadajac {currentTurn.base_damage} obrazen!
                </p>
                {#if currentTurn.spell_triggered}
                    <p class="mt-1 text-purple-300">
                        {currentTurn.spell_triggered}: {currentTurn.spell_description}
                    </p>
                {/if}
            {:else}
                <p class="text-white">Kliknij przycisk zeby rozpoczac</p>
            {/if}
        </div>

        {#if isFinished}
            <div class="text-center py-4 bg-gray-700 border rounded-xl">
            <p class="text-xl font-bold mb-4 text-white">{winnerName} wygrywa!</p>
                <button
                    onclick={handleStart}
                    disabled={starting}
                    class="px-6 py-3 bg-teal-500 hover:bg-teal-800 text-white rounded-lg font-semibold disabled:opacity-50 active:scale-95 transition-all cursor-pointer"
                >
                    Nowa walka
                </button>
            </div>
        {:else}
            <div class="text-center">
                <button
                    onclick={handleNextTurn}
                    disabled={revealing}
                    class="px-6 py-3 bg-teal-500 hover:bg-teal-800 text-white rounded-lg font-semibold disabled:opacity-50 active:scale-95 transition-all cursor-pointer"
                >
                    {revealing ? '...' : 'Nastepna tura'}
                </button>
            </div>
        {/if}
    {/if}
</div>
