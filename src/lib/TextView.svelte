<script lang="ts">
    import {
        getDocument,
        type BlockId,
        type TextData,
    } from "./document.svelte";
    import TextBox from "./TextBox.svelte";
    import TextRender from "./TextRender.svelte";

    let { id }: { id: BlockId } = $props();
    let doc = $derived(getDocument());
    let block = $derived(doc.getBlock(id, "Text"));
    let textRender = $state<TextRender | undefined>(undefined);

    function onClick(e: MouseEvent) {
        const caretPosition = document.caretRangeFromPoint(
            e.clientX,
            e.clientY,
        );
        if (caretPosition === null || textRender === undefined) return;
        doc.edit = {
            id,
            initialPosition: textRender.getPosition(
                caretPosition.startContainer,
                caretPosition.startOffset,
            ),
        };
    }
</script>

{#if doc.edit && doc.edit.id === id}
    <TextBox
        bind:text={block.data.value.content}
        initialPosition={doc.edit.initialPosition}
        style={block.data.value.style}
        onSwitchStyle={(style) => (block.data.value.style = style)}
    />
{:else}
    <div
        role="none"
        class:h1={block.data.value.style === "H1"}
        class:h2={block.data.value.style === "H2"}
        class:h3={block.data.value.style === "H3"}
        class:h4={block.data.value.style === "H4"}
        class:h5={block.data.value.style === "H5"}
        class:h6={block.data.value.style === "H6"}
        onclick={onClick}
    >
        <TextRender source={block.data.value.content} bind:this={textRender} />
    </div>
{/if}
