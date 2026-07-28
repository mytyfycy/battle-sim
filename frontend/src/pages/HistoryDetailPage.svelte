<script>
    import { onMount } from "svelte"
    import BattleReplay from '../components/BattleReplay.svelte'
    import { getBattle, ApiError } from '../lib/api/battles'

    let { battleId, navigate } = $props()

    let battle = $state(null)
    let loading = $state(true)
    let error = $state(null)

    async function load(id) {
      loading = true
      error = null
      battle = null
      try {
        battle = await getBattle(id)
      } catch (err) {
        error = err instanceof ApiError ? err.message : 'Nieoczekiwany blad'
      } finally {
        loading = false
      }
    }

    onMount(() => load(battleId))

    $effect(() => {
      load(battleId)
    })

    function backToHistory() {
      navigate('#/history')
    }
</script>

<div class="max-w-3xl mx-auto">
    <div class="flex items-center justify-between gap-4">
        <button onclick={backToHistory} class="mb-2 text-sm font-medium text-(--gold) hover:text-(--ink) active:scale-95 cursor-pointer transition-all">
            {'<-'} Wroc do historii walk
        </button>

        <h1 class="text-xl font-semibold mb-2 text-(--ink-dim)">Walka #{battleId}</h1>
    </div>

    {#if loading}
        <p class="text-(--ink-dim)">Wczytywanie walki...</p>
    {:else if error}
        <p class="bg-red-500 text-black p-4 rounded-xl mb-6 font-medium">Blad: {error}</p>
    {:else if battle}
        <BattleReplay battle={battle.full_result} instant={true} />
    {/if}
</div>
