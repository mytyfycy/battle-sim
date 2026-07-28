<script>
    import BattlePage from './pages/BattlePage.svelte'
    import HistoryDetailPage from './pages/HistoryDetailPage.svelte'
    import HistoryPage from './pages/HistoryPage.svelte'

    let route = $state(window.location.hash || '#/battle')

    window.addEventListener('hashchange', () => {
      route = window.location.hash
    })

    function navigate(hash) {
      window.location.hash = hash
    }

    const historyDetailMatch = $derived(route.match(/^#\/history\/(.+)$/))
</script>

<nav class="flex gap-4 p-4 border-b text-black font-medium border-(--panel-border)">
    <button onclick={() => navigate('#/battle')}
        class="cursor-pointer transition-all
        {route.startsWith('#/battle') ? 'text-(--gold)' : 'text-(--ink) hover:text-(--ink-dim)'}
    ">
        Walka
    </button>
    <button onclick={() => navigate('#/history')}
        class="cursor-pointer transition-all
        {route.startsWith('#/history') ? 'text-(--gold)' : 'text-(--ink) hover:text-(--ink-dim)'}
    ">
        Historyczne walki
    </button>
</nav>

<main class="p-2">
    {#if historyDetailMatch}
        <HistoryDetailPage battleId={historyDetailMatch[1]} {navigate} />
    {:else if route.startsWith('#/history')}
        <HistoryPage {navigate} />
    {:else}
        <BattlePage />
    {/if}
</main>
