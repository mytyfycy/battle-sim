<script>
    import BattlePage from './pages/BattlePage.svelte'
    import HistoryDetailPage from './pages/HistoryDetailPage.svelte'
    import HistoryPage from './pages/HistoryPage.svelte'
    import LoginPage from './pages/LoginPage.svelte'
    import { onMount } from 'svelte'
    import { authState, refreshAuth, logout } from './lib/auth.svelte'

    let route = $state(window.location.hash || '#/battle')

    window.addEventListener('hashchange', () => {
      route = window.location.hash
    })

    function navigate(hash) {
      window.location.hash = hash
    }

    onMount(() => {
      refreshAuth()
    })

    const historyDetailMatch = $derived(route.match(/^#\/history\/(.+)$/))
</script>

<nav class="flex justify-between items-center gap-4 p-4 border-b text-black font-medium border-(--panel-border)">
    <div class="flex gap-4">
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
    </div>

    <div class="flex items-center gap-3">
        {#if authState.nick}
            <span class="text-(--ink-dim) text-sm">{authState.nick}</span>
            <button onclick={() => logout()} class="cursor-pointer text-(--ink) hover:text-(--ink-dim) transition-all">
                Wyloguj
            </button>
        {:else}
            <button onclick={() => navigate('#/login')}
                class="cursor-pointer transition-all
                {route.startsWith('#/login') ? 'text-(--gold)' : 'text-(--ink) hover:text-(--ink-dim)'}
                ">
                    Zaloguj
            </button>
        {/if}
    </div>
</nav>

<main class="p-2">
    {#if historyDetailMatch}
        <HistoryDetailPage battleId={historyDetailMatch[1]} {navigate} />
    {:else if route.startsWith('#/history')}
        <HistoryPage {navigate} />
    {:else if route.startsWith('#/login')}
        <LoginPage {navigate} />
    {:else}
        <BattlePage />
    {/if}
</main>
