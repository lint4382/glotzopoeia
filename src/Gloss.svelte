<script lang="ts">
    import { onMount } from "svelte";
    import PreviewTextBox from "./PreviewTextBox.svelte";
    import {
        ArrowLeftToLineIcon,
        ArrowRightFromLine,
        Grid2x2PlusIcon,
        SettingsIcon,
        XIcon,
    } from "@lucide/svelte";

    type Column = {
        id: string;
        name: string;
        content:
            | {
                  type: "Rows";
                  value: string[];
              }
            | {
                  type: "FreeForm";
                  value: string;
              };
    };

    export type GlossData = {
        cols: Column[];
    };

    let {
        data = $bindable(),
    }: {
        data: {
            cols: Column[];
        };
    } = $props();

    let rowCount = $derived(
        Math.max(
            ...data.cols
                .map((x) => x.content)
                .filter((x) => x.type === "Rows")
                .map((x) => x.value.length),
            1,
        ),
    );

    let rowCols = $derived(data.cols.filter((x) => x.content.type === "Rows"));

    let colCount = $derived(rowCols.length);

    let mode:
        | { type: "View"; data?: undefined }
        | { type: "Edit"; data?: { col: string; row?: number } } = $state({
        type: "View",
    });

    function distributeRows(col: Column) {
        mode.data = undefined;
        if (col.content.type === "FreeForm") {
            col.content = { type: "Rows", value: col.content.value.split(" ") };
        }
    }

    function consolidateRows(col: Column) {
        mode.data = undefined;
        if (col.content.type === "Rows") {
            col.content = {
                type: "FreeForm",
                value: col.content.value.join(" "),
            };
        }
    }

    function rowsKeyDown(e: KeyboardEvent) {
        if (mode.type !== "Edit" || mode.data?.row === undefined) return;
        if (e.key === "ArrowDown" || e.key === "Enter") {
            mode.data.row++;
            padRows(mode.data.row + 1);
        } else if (e.key === "ArrowUp") {
            mode.data.row = Math.max(0, mode.data.row - 1);
        }
    }

    function padRows(n: number) {
        for (const col of data.cols) {
            if (col.content.type === "Rows") {
                while (col.content.value.length < n) {
                    col.content.value.push("");
                }
            }
        }
    }
</script>

{#if mode.type === "View"}
    <div
        class="grid justify-start gap-x-4 overflow-x-auto"
        style:grid-template-columns={`repeat(${rowCount}, auto)`}
    >
        {#each data.cols as col (col.id)}
            {#if col.content.type === "Rows"}
                {#if col.content.value.filter((x) => x.trim()).length === 0}
                    <div class="italic">({col.name} is empty)</div>
                    {#each { length: rowCount - 1 }, i}
                        <div></div>
                    {/each}
                {:else}
                    {#each { length: rowCount }, i}
                        <div
                            class="w-full"
                            role="gridcell"
                            tabindex="0"
                            ondblclick={() =>
                                (mode = {
                                    type: "Edit",
                                    data: { col: col.id, row: i },
                                })}
                        >
                            {col.content.value[i] ?? ""}
                        </div>
                    {/each}
                {/if}
            {:else}
                <div
                    role="gridcell"
                    tabindex="0"
                    class="col-span-full"
                    ondblclick={() =>
                        (mode = { type: "Edit", data: { col: col.id } })}
                >
                    {#if col.content.value.trim()}
                        {col.content.value}
                    {:else}
                        <div class="italic">({col.name} is empty)</div>
                    {/if}
                </div>
            {/if}
        {/each}
    </div>
{:else if mode.type === "Edit"}
    <div class="-ml-6 flex">
        <div
            class="dark:bg-nord9 dark:text-nord0 flex w-6 flex-col items-center justify-between rounded-l-lg"
        >
            <div>
                <SettingsIcon size={16} />
            </div>
            <div class="writing-mode-sideways-lr">Gloss</div>
            <button onclick={() => (mode = { type: "View" })}>
                <XIcon size={16} />
            </button>
        </div>
        <div
            class="dark:bg-nord1 flex-1 rounded-r-lg p-4"
            onfocusout={(e) => {
                // if (
                //     !e.relatedTarget ||
                //     !e.currentTarget.contains(e.relatedTarget as Node)
                // ) {
                //     pos = undefined;
                // }
            }}
        >
            <div
                class="dark:bg-nord2 grid grid-cols-[auto_1fr_auto] justify-stretch gap-2 gap-x-6 rounded-lg p-4"
            >
                {#each data.cols as col (col.id)}
                    <div class="text-right text-xs">{col.name}</div>
                    {#if col.content.type === "FreeForm"}
                        <PreviewTextBox
                            onDeactivate={() => (mode.data = undefined)}
                            bind:value={col.content.value}
                            active={mode.data?.col === col.id}
                        >
                            <div
                                role="textbox"
                                tabindex="0"
                                onfocus={() => (mode.data = { col: col.id })}
                            >
                                {col.content.value}
                            </div>
                        </PreviewTextBox>
                    {:else}
                        <div class="dark:text-nord3">(interlinear row)</div>
                    {/if}
                    <button
                        onclick={() =>
                            col.content.type === "FreeForm"
                                ? distributeRows(col)
                                : consolidateRows(col)}
                    >
                        {#if col.content.type === "FreeForm"}
                            <ArrowRightFromLine />
                        {:else}
                            <ArrowLeftToLineIcon />
                        {/if}
                    </button>
                {/each}
            </div>

            <div
                class="dark:bg-nord2 mt-4 grid rounded-lg"
                class:hidden={colCount === 0}
                style:grid-template-columns={`repeat(${colCount}, 1fr)`}
                onkeydown={rowsKeyDown}
                role="none"
            >
                {#each rowCols as col, colI (col.id)}
                    {#if col.content.type === "Rows"}
                        <div
                            class="dark:bg-nord0 p-1 text-xs"
                            class:rounded-tl-lg={colI === 0}
                            class:rounded-tr-lg={colI === colCount - 1}
                            class:pl-2={colI === 0}
                            class:pr-2={colI === colCount - 1}
                        >
                            {col.name}
                        </div>
                    {/if}
                {/each}
                {#each { length: rowCount }, i}
                    {#each rowCols as col, colI (col.id)}
                        {#if col.content.type === "Rows"}
                            <div
                                class="h-full p-1"
                                class:pl-2={colI === 0}
                                class:pr-2={colI === colCount - 1}
                            >
                                <PreviewTextBox
                                    active={mode.data?.col === col.id &&
                                        mode.data.row === i}
                                    onDeactivate={() => (mode.data = undefined)}
                                    bind:value={col.content.value[i]}
                                >
                                    <div
                                        role="textbox"
                                        tabindex="0"
                                        onfocus={() =>
                                            (mode.data = {
                                                col: col.id,
                                                row: i,
                                            })}
                                    >
                                        {col.content.value[i]}
                                    </div>
                                </PreviewTextBox>
                            </div>
                        {/if}
                    {/each}
                {/each}
            </div>
        </div>
    </div>
{/if}
