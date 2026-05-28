<script lang="ts">
    import { getDocument, type BlockId } from "./document.svelte";
    import ListView from "./ListView.svelte";
    import TextView from "./TextView.svelte";

    let { id }: { id: BlockId } = $props();
    let tag = $derived(getDocument().blocks[id].data.tag);
</script>

<div>
    {#if tag === "List"}
        <ListView {id} />
    {:else if tag === "Text"}
        <TextView {id} />
    {/if}
</div>
