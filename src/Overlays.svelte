<script module lang="ts">
    import CreateBlockMenu from "./CreateBlockMenu.svelte";

    export let mouse = $state({ x: 0, y: 0 });

    function onMouseMove(e: MouseEvent) {
        mouse.x = e.clientX;
        mouse.y = e.clientY;
    }
</script>

<svelte:window on:mousemove={onMouseMove} />

<CreateBlockMenu />
