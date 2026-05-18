<script lang="ts">
    import { CirclePlusIcon } from "@lucide/svelte";
    import BlockView, { type Block, type BlockData } from "./BlockView.svelte";
    import type { GlossData } from "./Gloss.svelte";
    import { showCreateBlockMenu } from "./CreateBlockMenu.svelte";

    let { data = $bindable() }: { data: Block[] } = $props();

    function insertBlock(i: number, newBlock: BlockData) {
        data.splice(i, 0, { id: crypto.randomUUID(), data: newBlock });
    }
</script>

{#snippet newBlockButton(i: number)}
    <div class="dark:text-nord3 flex justify-center">
        <button
            onclick={() =>
                showCreateBlockMenu((newBlock) => insertBlock(i, newBlock))}
            ><CirclePlusIcon size={16} /></button
        >
    </div>
{/snippet}

{#each data as block, i (block.id)}
    {@render newBlockButton(i)}
    <BlockView bind:block={data[i]} />
{/each}
{@render newBlockButton(data.length)}
