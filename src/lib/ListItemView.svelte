<script lang="ts">
    import { GripVerticalIcon } from "@lucide/svelte";
    import { getDocument, type BlockId } from "./document.svelte";
    import BlockView from "./BlockView.svelte";
    import BlocksView from "./BlocksView.svelte";

    let { id }: { id: BlockId } = $props();
</script>

<div class="grid grid-cols-[auto_1fr]">
    <div>&bullet;</div>
    <BlocksView {id} />
</div>
