<script lang="ts">
    import { MoonIcon, SunIcon } from "@lucide/svelte";
    import "../styles/index.css";
    import Gloss, { type GlossData } from "../Gloss.svelte";
    import type { Block } from "../BlockView.svelte";
    import Editor from "../Editor.svelte";
    import CreateBlockMenu from "../CreateBlockMenu.svelte";
    import Overlays from "../Overlays.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let darkMode = $state(true);

    let data: Block[] = $state([
        {
            id: "test",
            data: {
                type: "Gloss",
                value: {
                    cols: [
                        {
                            id: "source",
                            name: "Source",
                            content: {
                                type: "FreeForm",
                                value: "Pra ka vajni.",
                            },
                        },
                        {
                            id: "words",
                            name: "Words",
                            content: {
                                type: "Rows",
                                value: ["pra", "ka", "vajni"],
                            },
                        },
                        {
                            id: "gloss",
                            name: "Gloss",
                            content: {
                                type: "Rows",
                                value: ["horse", "NOM", "go"],
                            },
                        },
                        {
                            id: "translation",
                            name: "Translation",
                            content: {
                                type: "FreeForm",
                                value: "The horse goes.",
                            },
                        },
                    ],
                },
            },
        },
    ]);
</script>

<div
    class="bg-nord6 dark:bg-nord0 text-nord0 dark:text-nord6 relative h-screen w-screen p-6 pt-16"
    data-theme={darkMode ? "dark" : undefined}
>
    <button
        onclick={() => {
            invoke("save", { data })
                .then((x) => console.error(x))
                .catch((x) => console.error(x));
        }}>Save</button
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

    <Editor bind:data />

    <Overlays />
</div>

<style>
</style>
