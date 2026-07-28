<script>
    let {
      attackerName = null,
      defenderName = null,
      damage = null,
      spellName = null,
      spellText = null,
      turnKey = 0
    } = $props()
</script>

<div
    class="relative min-h-28 rounded-lg p-4 text-sm bg-(--panel) border-2 border-(--panel-border)"
>
    {#key turnKey}
        <div class="animate-dialog-in">
            {#if attackerName}
                <p class="leading-relaxed">
                    <span class="font-semibold text-(--gold)">{attackerName}</span>
                    <span class="text-(--ink)">atakuje</span>
                    <span class="font-semibold text-(--ink)">{defenderName}</span>
                    <span class="text-(--ink)">zadajac</span>
                    <span class="font-bold text-(--crimson)">{damage}</span>
                    <span class="text-(--ink)">HP</span>
                </p>
                {#if spellName}
                    <p class="mt-1.5 pt-1.5 border-t text-purple-300 border-(--panel-border)">
                        <span class="font-semibold">{spellName}</span>
                        {#if spellText}
                            <span class="block text-xs mt-0.5 text-(--ink-dim)">{spellText}</span>
                        {/if}
                    </p>
                {/if}
            {:else}
                <p class="text-(--ink)">Kliknij przycisk, aby rozpoczac</p>
            {/if}
        </div>
    {/key}
</div>
