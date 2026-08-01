<script lang="ts">
  import { Popover, type OverlaySurfaceGeometryChangeHandler } from "@poodle/svelte";

  interface Props {
    onSurfaceGeometryChange: OverlaySurfaceGeometryChangeHandler;
  }

  let { onSurfaceGeometryChange }: Props = $props();
</script>

<Popover
  defaultOpen
  ariaLabel="Packed geometry"
  {onSurfaceGeometryChange}
>
  {#snippet trigger()}<span>Open</span>{/snippet}
  <p>Packed surface</p>
</Popover>
