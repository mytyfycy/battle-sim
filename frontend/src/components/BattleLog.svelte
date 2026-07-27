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
    class="relative min-h-28 rounded-lg p-4 text-sm"
    style="background: var(--panel); border: 2px solid var(--panel-border);"
>
    {#key turnKey}
        <div class="animate-dialog-in">
            {#if attackerName}
                <p class="leading-relaxed">
                    <span class="font-semibold" style="color: var(--gold);">{attackerName}</span>
                    <span style="color: var(--ink);">atakuje</span>
                    <span class="font-semibold" style="color: var(--ink);">{defenderName}</span>
                    <span style="color: var(--ink);">zadajac</span>
                    <span class="font-bold" style="color: var(--crimson);">{damage}</span>
                    <span style="color: var(--ink);">HP.</span>
                </p>
                {#if spellName}
                    <p class="mt-1.5 pt-1.5 border-t text-purple-300" style="border-color: var(--panel-border);">
                        <span class="font-semibold">{spellName}</span>
                        {#if spellText}
                            <span class="block text-xs mt-0.5" style="color: var(--ink-dim);">{spellText}</span>
                        {/if}
                    </p>
                {/if}
            {:else}
                <p style="color: var(--ink);">Kliknij przycisk, aby rozpoczac.</p>
            {/if}
        </div>
    {/key}
</div>
