<script lang="ts">
    let { data = $bindable() }: { data: string } = $props();
</script>

<p>{data}</p>
