<script lang="ts">
    import {
        getDocument,
        type BlockId,
        type ListData,
    } from "./document.svelte";
    import ListItemView from "./ListItemView.svelte";

    let { id }: { id: BlockId } = $props();

    let doc = $derived(getDocument());
    let block = $derived(doc.blocks[id]);
</script>

<div>
    {#each block.children as id, i (id)}
        <div>
            <ListItemView {id} />
        </div>
    {/each}
</div>
