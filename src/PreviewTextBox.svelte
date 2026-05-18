<script lang="ts">
    import type { Snippet, SvelteComponent } from "svelte";
    import TextBox from "./TextBox.svelte";
    import type { EditorView } from "@codemirror/view";

    let {
        value = $bindable(),
        active,
        children,
        onDeactivate,
    }: {
        value: string;
        active: boolean;
        children: Snippet<[]>;
        onDeactivate?: () => void;
    } = $props();

    let inner: TextBox | undefined = $state(undefined);
    $effect(() => {
        if (active) {
            inner!.focus();
        }
    });

    function onKeyDown(e: KeyboardEvent) {
        if (e.key === "Enter" && !e.shiftKey) {
            if (onDeactivate) onDeactivate();
        }
    }
</script>

<div>
    {#if active}
        <div role="none" onkeydown={onKeyDown}>
            <TextBox bind:this={inner} bind:text={value} />
        </div>
    {:else}
        <div>{@render children()}</div>
    {/if}
</div>
