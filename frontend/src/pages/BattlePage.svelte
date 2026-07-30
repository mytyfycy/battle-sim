<script>
    import BattleReplay from '../components/BattleReplay.svelte'
    import { startBattle, ApiError } from '../lib/api/battles'

    let battle = $state(null)
    let starting = $state(false)
    let error = $state(null)
    let replayKey = $state(0)

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

<div class="max-w-3xl lg:max-w-400 mx-auto px-4">
    {#if error}
        <p class="bg-red-500 text-black p-4 rounded-xl mb-6 font-medium">Blad: {error}</p>
    {/if}
    {#if !battle}
        <div class="max-w-3xs mx-auto text-center py-4 border-2 rounded-xl bg-(--panel) border-(--panel-border)">
            <button
                onclick={handleStart}
                disabled={starting}
                class="px-6 py-3 bg-(--gold) hover:text-(--ink) rounded-lg font-semibold disabled:opacity-50 active:scale-95 transition-all cursor-pointer"
                >
                    {starting ? 'Ladowanie...' : 'Rozpocznij gre'}
                </button>
        </div>
    {:else}
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
    {/if}
</div>
