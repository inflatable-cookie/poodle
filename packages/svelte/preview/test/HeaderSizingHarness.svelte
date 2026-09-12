<script lang="ts">
  // g18.025 paired harness: mounts the preview header inside an app-shell-like
  // UiPresentationProvider so the tests can prove all header controls follow
  // the selected specimen scale (xs–xl) together.
  import {
    UiPresentationProvider,
    type ControlDensity,
    type ControlSize,
  } from "@inflatable-cookie/poodle-svelte";
  import DisplayControls from "../src/components/DisplayControls.svelte";

  interface Props {
    sizeScale?: ControlSize;
    density?: ControlDensity;
    controlSize?: string;
    theme?: string;
    search?: string;
    contrast?: number;
    onThemeChange?: (value: string) => void;
    onDensityChange?: (value: string) => void;
    onControlSizeChange?: (value: string) => void;
    onSearchChange?: (value: string) => void;
    onContrastChange?: (value: number) => void;
  }

  const {
    sizeScale = "sm",
    density = "compact",
    controlSize = "sm",
    theme = "eclipse",
    search = "",
    contrast = 0.5,
    onThemeChange = () => {},
    onDensityChange = () => {},
    onControlSizeChange = () => {},
    onSearchChange = () => {},
    onContrastChange = () => {},
  }: Props = $props();
</script>

<UiPresentationProvider {sizeScale} {density}>
  <DisplayControls
    {theme}
    {density}
    {controlSize}
    {search}
    {contrast}
    {onThemeChange}
    {onDensityChange}
    {onControlSizeChange}
    {onSearchChange}
    {onContrastChange}
  />
</UiPresentationProvider>
