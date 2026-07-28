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
            class="px-3 py-1 border-2 border-(--panel-border) text-(--ink) hover:text-(--ink-dim) rounded-lg font-semibold disabled:opacity-50 active:scale-95 transition-all cursor-pointer"
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
    <div class="px-6 py-3 text-center text-(--ink) bg-(--panel) border-2 border-(--panel-border) rounded-xl">
        <DataTable {columns} ajaxUrl="{API_BASE}/battles" onDetailsClick={goToDetails} />
    </div>
</div>

<style>
    :global(.dt-search input),
    :global(.dt-length select) {
        border: 2px solid var(--panel-border) !important;
        background-color: var(--panel) !important;
        color: inherit;
        border-radius: 4px !important;
        padding: 4px 8px !important;
    }

    :global(.dt-length select option:checked) {
        background-color: var(--panel-border) !important;
        color: #fff !important;
    }

    :global(.dt-paging .dt-paging-button) {
        border: 2px solid var(--panel-border) !important;
        border-radius: 4px !important;
        background: transparent !important;
    }

    :global(table.dataTable tbody tr:nth-child(even)) {
        background-color: rgba(255,255,255,0.03) !important;
    }

    :global(table.dataTable tbody tr:hover) {
        background-color: rgba(255,255,255, 0.07) !important;
    }

    :global(table.dataTable thead th) {
        background-color: var(--panel-border) !important;
        text-transform: uppercase;
        font-size: 14px;
        text-align: center;
        border-bottom: 2px solid var(--ink-dim) !important;
    }

    :global(table.dataTable thead th:hover) {
        background-color: rgba(255,255,255,0.05) !important;
    }

    :global(table.dataTable thead) {
        box-shadow: 0 4px 6px -1px rgba(0,0,0,0.3);
    }
</style>
