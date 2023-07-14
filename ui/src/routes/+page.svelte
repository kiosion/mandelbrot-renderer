<script lang="ts">
  import '../app.css';
  import MandelbrotCanvas from '$lib/canvas.svelte';

  let thisZoom = "1.0", zoom: number, updateZoom: ((n: number) => void) | undefined;
  $: thisZoom !== undefined && (thisZoom = `${zoom}`), updateZoom?.(parseFloat(thisZoom));
</script>

<MandelbrotCanvas bind:updateZoom bind:zoom />
<input type="range" min="1.0" max="20.0" step="0.1" bind:value={zoom} />

<style>
  input {
    position: absolute;
    bottom: 12px;
    left: 12px;
    width: 50%;
    min-width: 100px;
    max-width: 400px;
  }
</style>