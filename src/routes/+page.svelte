<script lang="ts">
    import { MoonIcon, SunIcon } from "@lucide/svelte";
    import "$lib/style.css";
    import { Document, getDocument, setDocument } from "../lib/document.svelte";
    import BlocksView from "$lib/BlocksView.svelte";

    let darkMode = $state(true);

    setDocument(
        new Document("root", {
            root: {
                data: {
                    tag: "Opaque",
                    value: undefined,
                },
                children: ["ul"],
            },
            ul: {
                data: {
                    tag: "List",
                    value: {
                        style: "Bullet",
                    },
                },
                children: ["li1", "li2"],
            },
            li1: {
                data: {
                    tag: "Opaque",
                    value: undefined,
                },
                children: ["a"],
            },
            a: {
                data: {
                    tag: "Text",
                    value: {
                        content: "Foo",
                        style: "P",
                    },
                },
                children: [],
            },
            li2: {
                data: {
                    tag: "Opaque",
                    value: undefined,
                },
                children: ["b"],
            },
            b: {
                data: {
                    tag: "Text",
                    value: {
                        content: "Bar",
                        style: "P",
                    },
                },
                children: [],
            },
        }),
    );
    let doc = $derived(getDocument());

    function onKeyDown(e: KeyboardEvent) {
        doc.handleKeyDown(e);
    }
</script>

<svelte:window onkeydown={onKeyDown} />

<div
    class="bg-nord6 dark:bg-nord0 text-nord0 dark:text-nord6 relative h-screen w-screen p-6 pt-16"
    data-theme={darkMode ? "dark" : undefined}
>
    <button
        class="absolute top-0 right-0 p-2"
        onclick={() => {
            darkMode = !darkMode;
        }}
    >
        {#if darkMode}
            <SunIcon size={32} />
        {:else}
            <MoonIcon size={32} />
        {/if}
    </button>

    <BlocksView id={doc.root} />
</div>
