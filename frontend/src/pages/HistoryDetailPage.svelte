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

<div class="max-w-3xl lg:max-w-400 mx-auto px-4">
    <div class="lg:grid lg:grid-cols-[var(--battle-side-col)_1fr_var(--battle-side-col)] lg:gap-x-(--battle-gap) lg:gap-y-0">
        <div class="flex items-center justify-between gap-4 mb-2 lg:col-start-2">
            <button onclick={backToHistory} class="text-sm font-medium text-(--gold) hover:text-(--ink) active:scale-95 cursor-pointer transition-all">
                {'<-'} Wroc do historii walk
            </button>

            <h1 class="text-xl font-semibold text-(--ink-dim)">Walka #{battleId}</h1>
        </div>
    </div>

    {#if loading}
        <p class="text-(--ink-dim)">Wczytywanie walki...</p>
    {:else if error}
        <p class="bg-red-500 text-black p-4 rounded-xl mb-6 font-medium">Blad: {error}</p>
    {:else if battle}
        <BattleReplay battle={battle.full_result} instant={true} />
    {/if}
</div>
