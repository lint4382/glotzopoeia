<script lang="ts">
    const { source }: { source: string } = $props();

    let root: HTMLDivElement | undefined;

    $effect(() => {
        if (root!.firstChild) {
            root!.firstChild.remove();
        }
        try {
            root!.append(
                new DOMParser().parseFromString(
                    "<div>" + source + "</div>",
                    "text/html",
                ).firstChild!,
            );
        } catch (e) {}
    });
</script>

<div bind:this={root}></div>
