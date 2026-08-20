<script lang="ts">
  import { ResizeHandle } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";

  const MIN_HORIZONTAL = 48;
  const MAX_HORIZONTAL = 280;
  const MIN_VERTICAL = 40;
  const MAX_VERTICAL = 120;

  let leftWidth = $state(120);
  let topHeight = $state(80);

  function clamp(value: number, min: number, max: number): number {
    return Math.min(max, Math.max(min, value));
  }

  function applyHorizontalDelta(delta: number): void {
    leftWidth = clamp(leftWidth + delta, MIN_HORIZONTAL, MAX_HORIZONTAL);
  }

  function applyVerticalDelta(delta: number): void {
    topHeight = clamp(topHeight + delta, MIN_VERTICAL, MAX_VERTICAL);
  }
</script>

<div class="poodle-specimen">
  <SpecimenGroup label="Horizontal split (vertical handle — drag left/right)">
    <div class="poodle-specimen__row">
      <div class="poodle-specimen__pane" style:flex="0 0 {leftWidth}px">Left</div>
      <div class="poodle-specimen__handle-wrapper poodle-specimen__handle-wrapper--horizontal">
        <ResizeHandle
          orientation="horizontal"
          ariaLabel="Resize horizontal"
          ariaValueNow={leftWidth}
          onResizeMove={applyHorizontalDelta}
          onResizeStep={applyHorizontalDelta}
        />
      </div>
      <div class="poodle-specimen__pane poodle-specimen__pane--grow">Right</div>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Vertical split (horizontal handle — drag up/down)">
    <div class="poodle-specimen__col">
      <div class="poodle-specimen__pane" style:flex="0 0 {topHeight}px">Top</div>
      <div class="poodle-specimen__handle-wrapper poodle-specimen__handle-wrapper--vertical">
        <ResizeHandle
          orientation="vertical"
          ariaLabel="Resize vertical"
          ariaValueNow={topHeight}
          onResizeMove={applyVerticalDelta}
          onResizeStep={applyVerticalDelta}
        />
      </div>
      <div class="poodle-specimen__pane poodle-specimen__pane--grow">Bottom</div>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled (horizontal split)">
    <div class="poodle-specimen__row">
      <div class="poodle-specimen__pane">Left</div>
      <div class="poodle-specimen__handle-wrapper poodle-specimen__handle-wrapper--horizontal">
        <ResizeHandle orientation="horizontal" disabled ariaLabel="Disabled resize" />
      </div>
      <div class="poodle-specimen__pane">Right</div>
    </div>
  </SpecimenGroup>

  <SpecimenGroup label="Disabled (vertical split)">
    <div class="poodle-specimen__col">
      <div class="poodle-specimen__pane">Top</div>
      <div class="poodle-specimen__handle-wrapper poodle-specimen__handle-wrapper--vertical">
        <ResizeHandle orientation="vertical" disabled ariaLabel="Disabled resize vertical" />
      </div>
      <div class="poodle-specimen__pane">Bottom</div>
    </div>
  </SpecimenGroup>
</div>

<style>
  .poodle-specimen {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .poodle-specimen__row {
    display: flex;
    align-items: stretch;
    height: 6rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  .poodle-specimen__col {
    display: flex;
    flex-direction: column;
    height: 10rem;
    border: 0.0625rem solid var(--poodle-color-border-subtle);
    border-radius: var(--poodle-radius-surface);
    overflow: hidden;
  }

  .poodle-specimen__pane {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8125rem;
    color: var(--poodle-color-text-secondary);
    background: color-mix(in srgb, var(--poodle-color-background-panel) 50%, transparent);
  }

  .poodle-specimen__pane--grow {
    flex: 1;
    min-width: 0;
    min-height: 0;
  }

  .poodle-specimen__handle-wrapper--horizontal {
    height: 100%;
  }

  .poodle-specimen__handle-wrapper--vertical {
    width: 100%;
  }
</style>
