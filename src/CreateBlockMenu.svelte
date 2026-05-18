<script module lang="ts">
    import { LanguagesIcon, TextInitialIcon } from "@lucide/svelte";
    import { mouse } from "./Overlays.svelte";
    import type { Component } from "svelte";
    import type { BlockData } from "./BlockView.svelte";

    let x = $state(0);
    let y = $state(0);
    let hidden = $state(true);
    let onSelect = $state((_: BlockData) => {});

    export function showCreateBlockMenu(
        onSelectCallback: (data: BlockData) => void,
    ) {
        x = mouse.x;
        y = mouse.y;
        hidden = false;
        onSelect = onSelectCallback;
    }

    const items: { label: string; icon: Component; data: BlockData }[] = [
        {
            label: "Paragraph",
            icon: TextInitialIcon,
            data: {
                type: "Paragraph",
                value: "",
            },
        },
        {
            label: "Gloss",
            icon: LanguagesIcon,
            data: {
                type: "Gloss",
                value: {
                    cols: [
                        {
                            id: "source",
                            name: "Source",
                            content: {
                                type: "FreeForm",
                                value: "",
                            },
                        },
                        {
                            id: "words",
                            name: "Words",
                            content: {
                                type: "Rows",
                                value: [],
                            },
                        },
                        {
                            id: "gloss",
                            name: "Gloss",
                            content: {
                                type: "Rows",
                                value: [],
                            },
                        },
                        {
                            id: "translation",
                            name: "Translation",
                            content: {
                                type: "FreeForm",
                                value: "",
                            },
                        },
                    ],
                },
            },
        },
    ];
</script>

<div
    class="dark:bg-nord1 dark:border-nord4/30 fixed w-48 rounded border shadow-lg"
    class:hidden
    style:left={`${x}px`}
    style:top={`${y}px`}
>
    {#each items as item}
        {@const Icon = item.icon}
        <button
            onclick={() => {
                hidden = true;
                onSelect(item.data);
            }}
            class="dark:hover:bg-nord3 flex items-center gap-2"
        >
            <Icon size={16} />
            <div>{item.label}</div>
        </button>
    {/each}
</div>
