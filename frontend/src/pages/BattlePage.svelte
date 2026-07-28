<script>
    import { onMount } from "svelte"
    import BattleReplay from '../components/BattleReplay.svelte'
    import { startBattle, ApiError } from '../lib/api/battles'

    let battle = $state(null)
    let starting = $state(false)
    let error = $state(null)
    let replayKey = $state(0)

    onMount(() => {
      handleStart()
    })

    async function handleStart() {
      starting = true
      error = null
      try {
        battle = await startBattle()
        replayKey++
      } catch (err) {
        error = err instanceof ApiError ? err.message : 'Nieoczekiwany blad'
      } finally {
        starting = false
      }
    }
</script>

<div class="max-w-3xl mx-auto px-4">
    {#if error}
        <p class="bg-red-500 text-black p-4 rounded-xl mb-6 font-medium">Blad: {error}</p>
    {/if}
    {#key replayKey}
        <BattleReplay {battle}>
            {#snippet finishedActions()}
                <button
                    onclick={handleStart}
                    disabled={starting}
                    class="px-6 py-2 rounded-lg font-semibold hover:text-(--ink) bg-(--gold) transition-all cursor-pointer"
                >
                    {starting ? 'Ladowanie...' : 'Nowa walka'}
                </button>
            {/snippet}
        </BattleReplay>
    {/key}
</div>
