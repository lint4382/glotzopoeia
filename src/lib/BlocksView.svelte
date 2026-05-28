<script lang="ts">
    import { GripVerticalIcon } from "@lucide/svelte";
    import { getDocument, type BlockId } from "./document.svelte";
    import BlockView from "./BlockView.svelte";

    let { id }: { id: BlockId } = $props();

    let doc = $derived(getDocument());
    let ids = $derived(doc.blocks[id].children);
</script>

{#each ids as id (id)}
    <div class="grid grid-cols-[auto_1fr] not-last:mb-2">
        <div
            class="ml-2 rounded-l-lg pr-1"
            class:bg-nord3={doc.selection.has(id)}
            role="none"
            onclick={() => {
                if (doc.selection.has(id)) {
                    doc.selection.clear();
                } else {
                    doc.selection.clear();
                    doc.selection.add(id);
                }
            }}
        >
            <div class="opacity-0 hover:opacity-50">
                <GripVerticalIcon size={16} />
            </div>
        </div>
        <div class="rounded-r-lg" class:bg-nord3={doc.selection.has(id)}>
            <BlockView {id} />
        </div>
    </div>
{/each}
