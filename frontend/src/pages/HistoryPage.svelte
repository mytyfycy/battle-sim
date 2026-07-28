<script>
    import DataTable from "../components/DataTable.svelte"

    let { navigate } = $props()

    const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:3000'

    const columns = [
      { title: 'ID walki', data: 'id' },
      { title: 'Gracz A', data: 'character_a_name' },
      { title: 'Gracz B', data: 'character_b_name' },
      { title: 'Wygrany', data: 'winner_name' },
      { title: 'Przegrany', data: 'loser_name' },
      { title: 'HP atakujacego na koniec', data: 'attacker_hp_at_end', className: 'dt-center'},
      {
        title: 'Szczegoly',
        data: 'id',
        orderable: false,
        render: (id) => `
          <button
            data-battle-id="${id}"
            class="px-6 py-3 bg-(--gold) text-black hover:text-(--ink) rounded-lg font-semibold disabled:opacity-50 active:scale-95 transition-all cursor-pointer"
            >
              Zobacz
            </button>
          `,
      },
    ]

    function goToDetails(id) {
      navigate(`#/history/${id}`)
    }
</script>

<div class="max-w-6xl mx-auto">
    <div class="px-6 py-3 text-center text-(--ink) bg-(--panel) border rounded-xl">
        <DataTable {columns} ajaxUrl="{API_BASE}/battles" onDetailsClick={goToDetails} />
    </div>
</div>

<style>
    :global(.dt-length select option) {
        background-color: var(--panel);
    }
</style>
