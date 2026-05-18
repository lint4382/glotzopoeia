<script lang="ts">
    import type { GlossData } from "./Gloss.svelte";
    import Gloss from "./Gloss.svelte";
    import Paragraph from "./Paragraph.svelte";

    export type Block = { id: string; data: BlockData };

    export type BlockData =
        | { type: "Gloss"; value: GlossData }
        | { type: "Paragraph"; value: string };

    let { block = $bindable() }: { block: Block } = $props();
</script>

<div>
    {#if block.data.type === "Gloss"}
        <Gloss bind:data={block.data.value} />
    {:else if block.data.type === "Paragraph"}
        <Paragraph bind:data={block.data.value} />
    {/if}
</div>
