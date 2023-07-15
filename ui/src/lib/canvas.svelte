<script lang="ts">
  import { Renderer } from 'mandelbrot_wasm';
  import { onDestroy, onMount } from 'svelte';

  export const updateZoom = (n: number) => {
    zoom = n;
    requestAnimationFrame()
  };
  export let zoom = 1.0;

  let WASMRenderer: InstanceType<typeof Renderer>;

  const max_iter = 200;
  let width: number;
  let height: number;
  let center_x = -0.6;
  let center_y = 0.0;
  let canvas: HTMLCanvasElement;
  let mouseDown = false;
  let prevMousePos = { x: 0, y: 0 };

  const run = async () => {
    console.time('compute');
    await WASMRenderer.compute(width, height, max_iter, center_x, center_y, zoom);

    console.timeEnd('compute');
  };

  const requestAnimationFrame = () => window.requestAnimationFrame(run);

  const handleResize = (_e: unknown, run = true) => {
    // canvas.width = width = window.innerWidth;
    // canvas.height = height = window.innerHeight;
    width = window.innerWidth;
    height = window.innerHeight;

    run && requestAnimationFrame();
  };

  onMount(async () => {
    WASMRenderer = await new Renderer();
    window.addEventListener('resize', handleResize);
    handleResize(null, false);

    requestAnimationFrame();
  });

  onDestroy(() => window.removeEventListener('resize', handleResize));

  const handleMouseDown = (e: MouseEvent) => {
    prevMousePos = { x: e.clientX, y: e.clientY };
    mouseDown = true;
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (mouseDown) {
        const zoomOr = Math.max(zoom, 1.5);
        center_x -= ((e.clientX - prevMousePos.x) / 1.5) / (100 * zoomOr);
        center_y -= ((e.clientY - prevMousePos.y) / 1.5) / (100 * zoomOr);
        prevMousePos = { x: e.clientX, y: e.clientY };
        requestAnimationFrame();
    }
  };

  const handleWheel = (e: Event) => {
    const zoomOrDefault = Math.max(zoom, 1.5),
      deltaX = (((e as WheelEvent).clientX - rect.left) / width - 0.5) / zoomOrDefault,
      deltaY = (((e as WheelEvent).clientY - rect.top) / height - 0.5) / zoomOrDefault;

    if ((e as WheelEvent).deltaY < 0) {
      zoom *= 1.2;
      center_x += deltaX;
      center_y += deltaY;
    } else {
      zoom /= 1.2;
      center_x -= deltaX;
      center_y -= deltaY;
    }

    requestAnimationFrame();
  };

  $: rect = canvas?.getBoundingClientRect();
</script>

<canvas
  id="canvas"
  bind:this={canvas}
  on:mousedown={handleMouseDown}
  on:mouseup={() => mouseDown = false}
  on:mouseleave={() => mouseDown = false}
  on:mousemove={handleMouseMove}
  on:wheel={handleWheel}
  on:scroll={handleWheel}
/>