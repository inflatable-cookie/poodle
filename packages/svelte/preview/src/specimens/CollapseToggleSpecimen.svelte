<script lang="ts">
  import { Eyebrow, CollapseToggle } from "@pug/svelte-primitives";

  let collapsedLeft = false;
  let collapsedRight = false;
  let collapsedUp = false;
  let collapsedDown = false;
</script>

<div class="specimen">
  <div class="specimen__group">
    <Eyebrow>Directions</Eyebrow>
    <div class="specimen__row">
      <div class="specimen__labeled">
        <CollapseToggle
          direction="left"
          isCollapsed={collapsedLeft}
          on:toggle={(e) => (collapsedLeft = e.detail.isCollapsed)}
        />
        <span>Left {collapsedLeft ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="specimen__labeled">
        <CollapseToggle
          direction="right"
          isCollapsed={collapsedRight}
          on:toggle={(e) => (collapsedRight = e.detail.isCollapsed)}
        />
        <span>Right {collapsedRight ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="specimen__labeled">
        <CollapseToggle
          direction="up"
          isCollapsed={collapsedUp}
          on:toggle={(e) => (collapsedUp = e.detail.isCollapsed)}
        />
        <span>Up {collapsedUp ? "(collapsed)" : "(expanded)"}</span>
      </div>
      <div class="specimen__labeled">
        <CollapseToggle
          direction="down"
          isCollapsed={collapsedDown}
          on:toggle={(e) => (collapsedDown = e.detail.isCollapsed)}
        />
        <span>Down {collapsedDown ? "(collapsed)" : "(expanded)"}</span>
      </div>
    </div>
  </div>

  <div class="specimen__group">
    <Eyebrow>Disabled</Eyebrow>
    <div class="specimen__row">
      <CollapseToggle direction="left" isDisabled />
      <CollapseToggle direction="right" isDisabled />
    </div>
  </div>
</div>

<style>
  .specimen {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .specimen__group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .specimen__row {
    display: flex;
    gap: 1.5rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .specimen__labeled {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .specimen__labeled span {
    font-size: 0.75rem;
    color: var(--pug-color-text-secondary);
  }
</style>
