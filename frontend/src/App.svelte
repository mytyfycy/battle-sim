<script>
    import BattlePage from './pages/BattlePage.svelte'
    import HistoryPage from './pages/HistoryPage.svelte'

    let route = $state(window.location.hash || '#/battle')

    window.addEventListener('hashchange', () => {
      route = window.location.hash
    })

    function navigate(hash) {
      window.location.hash = hash
    }
</script>

<nav class="flex gap-4 p-4 border-b text-black font-medium">
    <button onclick={() => navigate('#/battle')} class="hover:text-gray-500 cursor-pointer transition-all">Walka</button>
    <button onclick={() => navigate('#/history')} class="hover:text-gray-500 cursor-pointer transition-all">Historia walk</button>
</nav>

<main class="p-4">
    {#if route.startsWith('#/history')}
        <HistoryPage {navigate} />
    {:else}
        <BattlePage />
    {/if}
</main>
