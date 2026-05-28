<script lang="ts">
    const { kind, isClosing }: { kind: "Bold" | "Italic"; isClosing: boolean } =
        $props();
</script>

<div class="dark:bg-nord1 mx-0.5 inline rounded px-0.5 text-xs select-all">
    &lt;{#if isClosing}/{/if}{#if kind === "Bold"}<b class="mx-0.5">B</b
        >{:else}<i class="mx-0.5">I</i>{/if}&gt;
</div>
