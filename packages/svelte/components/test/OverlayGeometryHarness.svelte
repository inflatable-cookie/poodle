<script lang="ts">
  import type { OverlaySurfaceGeometryChangeHandler } from "@inflatable-cookie/poodle-headless";

  import Menu from "../src/Menu.svelte";
  import Popover from "../src/Popover.svelte";

  interface Props {
    onPopoverGeometry: OverlaySurfaceGeometryChangeHandler;
    onMenuGeometry: OverlaySurfaceGeometryChangeHandler;
  }

  let { onPopoverGeometry, onMenuGeometry }: Props = $props();

  const menuItems = [
    {
      value: "parent",
      label: "Parent",
      children: [{ value: "child", label: "Child" }],
    },
  ];
</script>

<Popover
  defaultOpen
  ariaLabel="Geometry popover"
  onSurfaceGeometryChange={onPopoverGeometry}
>
  {#snippet trigger()}<span>Popover trigger</span>{/snippet}
  <p>Popover content</p>
</Popover>

<Menu
  defaultOpen
  items={menuItems}
  ariaLabel="Geometry menu"
  onSurfaceGeometryChange={onMenuGeometry}
>
  {#snippet trigger()}<span>Menu trigger</span>{/snippet}
</Menu>
